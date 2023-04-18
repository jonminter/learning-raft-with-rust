use self::state_defs::*;

/// Implementation of Raft consensus protocol
/// See: <https://raft.github.io/raft.pdf> for details
/// Currently, only implements the leader election part of the protocol
use super::common::*;
use super::rpc_messages::*;
use crate::system_clock;
use crate::system_clock::Instant;
use divrem::DivCeil;
use rand::Rng;
use rand_chacha::ChaCha8Rng;
use std::collections::HashSet;
use std::fmt::Debug;
use std::time::Duration;
use tracing::debug;
use tracing::info;
use tracing::trace;
use uuid::Uuid;

#[derive(Debug, Clone)]
pub(crate) enum Event<C: LogCommand> {
    Tick(Instant),
    LogEntryAppliedByApplication(LogIndex),
    IncomingRpc(RpcMessage<C>),
}

#[derive(Debug, Clone)]
pub(crate) enum Action<C: LogCommand> {
    SetNextTimeout(Duration),
    ApplyLogEntries(Vec<C>),
    OutgoingRpc(RpcMessage<C>),
}

#[derive(Debug, Clone)]
pub(crate) enum Node {
    Leader(NodeState<Leader>),
    Follower(NodeState<Follower>),
    Candidate(NodeState<Candidate>),
}
impl Node {
    pub(crate) fn new(
        server_id: ServerId,
        other_servers: HashSet<ServerId>,
        config: &RaftConfig,
        rng: &mut ChaCha8Rng,
    ) -> (Self, FirstElectionTimeout) {
        let (initial_state, first_timer) =
            NodeState::<Follower>::new(server_id, other_servers, config, rng);

        (initial_state.into(), first_timer)
    }

    fn server_id(&self) -> ServerId {
        match self {
            Node::Leader(state) => state.server_id,
            Node::Follower(state) => state.server_id,
            Node::Candidate(state) => state.server_id,
        }
    }

    fn update_clock(&mut self) {
        match self {
            Node::Leader(state) => state.current_time = system_clock::now(),
            Node::Follower(state) => state.current_time = system_clock::now(),
            Node::Candidate(state) => state.current_time = system_clock::now(),
        }
    }

    fn if_rpc_message_has_higher_term_become_follower<C: LogCommand>(
        self,
        storage: &mut impl PersistentStorage<C>,
        event: &Event<C>,
        config: &RaftConfig,
        rng: &mut ChaCha8Rng,
    ) -> Result<(Self, Vec<Action<C>>), PersistentStorageError> {
        let (should_become_follower, new_term) = match event {
            Event::IncomingRpc(RpcMessage::Request(r)) => {
                (r.term() > storage.current_term(), r.term())
            }
            Event::IncomingRpc(RpcMessage::Reply(r)) => {
                (r.term() > storage.current_term(), r.term())
            }
            _ => (false, storage.current_term()),
        };

        if should_become_follower {
            info!(
                "{:?}: Becoming follower because we've observed a higher term {:?} than ours {:?}",
                self.server_id(),
                new_term,
                storage.current_term()
            );
            storage.update_term(new_term).sync()?;
            let mut follower_state: NodeState<Follower> = match self {
                Node::Leader(state) => state.transition_to(),
                Node::Follower(state) => state,
                Node::Candidate(state) => state.transition_to(),
            };

            // Ensure we don't have a leader ID set, if we were already follower this would be set
            // but since there is a newer term there might be a new leader
            // if so new leader will send us a heartbeat eventually and we'll update this
            follower_state.inner.leader_id = None;
            let election_timeout = follower_state.reset_election_timer(config, rng);
            Ok((
                follower_state.into(),
                vec![Action::SetNextTimeout(election_timeout)],
            ))
        } else {
            Ok((self, vec![]))
        }
    }

    pub fn next<C: LogCommand, PS: PersistentStorage<C>>(
        mut self,
        event: Event<C>,
        storage: &mut PS,
        config: &RaftConfig,
        rng: &mut ChaCha8Rng,
    ) -> Result<(Self, Vec<Action<C>>), PersistentStorageError> {
        self.update_clock();

        self.if_rpc_message_has_higher_term_become_follower(storage, &event, config, rng)
            .and_then(|(new_node, mut maybe_tick_timer)| {
                let (new_node, mut actions) = match new_node {
                    Self::Leader(state) => state.handle_event(event, storage, config, rng)?,
                    Self::Follower(state) => state.handle_event(event, storage, config, rng)?,
                    Self::Candidate(state) => state.handle_event(event, storage, config, rng)?,
                };

                actions.append(&mut maybe_tick_timer);
                Ok((new_node, actions))
            })
    }
}
impl From<NodeState<Leader>> for Node {
    fn from(state: NodeState<Leader>) -> Self {
        Node::Leader(state)
    }
}
impl From<NodeState<Follower>> for Node {
    fn from(state: NodeState<Follower>) -> Self {
        Node::Follower(state)
    }
}
impl From<NodeState<Candidate>> for Node {
    fn from(state: NodeState<Candidate>) -> Self {
        Node::Candidate(state)
    }
}

#[derive(Debug, Clone)]
pub(crate) struct NodeState<S: State> {
    server_id: ServerId,
    start_time: Instant,
    current_time: Instant,
    other_servers: HashSet<ServerId>,
    commit_index: LogIndex,
    last_applied: LogIndex,
    pub(crate) inner: S,
}

trait Transitions {
    fn handle_event<C, PS>(
        self,
        event: Event<C>,
        storage: &mut PS,
        config: &RaftConfig,
        rng: &mut ChaCha8Rng,
    ) -> Result<(Node, Vec<Action<C>>), PersistentStorageError>
    where
        C: LogCommand,
        PS: PersistentStorage<C>;
}

trait CanTransitionTo<NewState: State> {
    fn transition_to(self) -> NodeState<NewState>;
}

trait ElectionTimer {
    fn reset_election_timer(&mut self, config: &RaftConfig, rng: &mut ChaCha8Rng) -> Duration;
}
macro_rules! has_election_timer {
    ($state:ident) => {
        impl ElectionTimer for NodeState<$state> {
            fn reset_election_timer(
                &mut self,
                config: &RaftConfig,
                rng: &mut ChaCha8Rng,
            ) -> Duration {
                let election_timeout = Duration::from_millis(rng.gen_range(
                    config.min_election_timeout_ms.into()..config.max_election_timeout_ms.into(),
                ));

                self.inner.election_timeout = election_timeout;
                self.inner.last_election_timer_started = system_clock::now();

                election_timeout
            }
        }
    };
}

mod state_defs {
    use crate::common::LogIndex;
    use crate::common::ServerId;
    use crate::system_clock;
    use crate::system_clock::Instant;

    use std::collections::HashMap;
    use std::collections::HashSet;
    use std::fmt::Debug;
    use std::time::Duration;

    #[derive(Debug, Clone)]
    struct Priv {}

    pub(crate) trait State: Debug {}
    #[derive(Debug, Clone)]
    pub(crate) struct Leader {
        pub(crate) last_heartbeat_sent: Instant,
        pub(crate) next_index: HashMap<ServerId, LogIndex>,
        pub(crate) match_index: HashMap<ServerId, LogIndex>,
        _priv: Priv,
    }

    impl State for Leader {}
    impl From<Candidate> for Leader {
        fn from(_: Candidate) -> Self {
            Leader {
                last_heartbeat_sent: system_clock::now(),
                next_index: HashMap::new(),
                match_index: HashMap::new(),
                _priv: Priv {},
            }
        }
    }

    #[derive(Debug, Clone)]
    pub(crate) struct Candidate {
        pub(crate) last_election_timer_started: Instant,
        pub(crate) election_timeout: Duration,
        pub(crate) votes_received: HashSet<ServerId>,
        _priv: Priv,
    }
    impl State for Candidate {}
    impl From<Follower> for Candidate {
        fn from(_: Follower) -> Self {
            Candidate {
                last_election_timer_started: system_clock::now(),
                election_timeout: Duration::from_millis(0),
                votes_received: HashSet::new(),
                _priv: Priv {},
            }
        }
    }

    #[derive(Debug, Clone)]
    pub(crate) struct Follower {
        pub(crate) last_election_timer_started: Instant,
        pub(crate) election_timeout: Duration,
        pub(crate) leader_id: Option<ServerId>,
        _priv: Priv,
    }
    impl Follower {
        pub(crate) fn new() -> Self {
            Follower {
                last_election_timer_started: system_clock::now(),
                election_timeout: Duration::from_millis(0),
                leader_id: None,
                _priv: Priv {},
            }
        }
    }
    impl State for Follower {}
    impl From<Leader> for Follower {
        fn from(_: Leader) -> Self {
            Follower {
                last_election_timer_started: system_clock::now(),
                leader_id: None,
                election_timeout: Duration::from_millis(0),
                _priv: Priv {},
            }
        }
    }
    impl From<Candidate> for Follower {
        fn from(candidate: Candidate) -> Self {
            Follower {
                last_election_timer_started: system_clock::now(),
                election_timeout: candidate.election_timeout,
                leader_id: None,
                _priv: Priv {},
            }
        }
    }
}

impl<St: State> NodeState<St> {
    fn ack_append_entries<C, PS>(
        &self,
        storage: &PS,
        append_entries_req: AppendEntries<C>,
        success: bool,
    ) -> Vec<Action<C>>
    where
        C: LogCommand,
        PS: PersistentStorage<C>,
    {
        vec![Action::OutgoingRpc(RpcMessage::ack_append_entries(
            AppendEntriesAck {
                request_id: append_entries_req.request_id,
                from: self.server_id,
                to: append_entries_req.from,
                term: storage.current_term(),
                success,
            },
        ))]
    }

    fn vote_no<C, PS>(
        &self,
        storage: &mut PS,
        vote_req: RequestVote,
        reason: &str,
    ) -> Vec<Action<C>>
    where
        C: LogCommand,
        PS: PersistentStorage<C>,
    {
        debug!(
            "{server_id:?}: Vote NO for candidate {candidate_id:?} because {reason:?} (my term: {my_term:?}, vote term: {candidate_term:?})",
            server_id = self.server_id,
            candidate_id = vote_req.from,
            reason = reason,
            my_term=storage.current_term(),
            candidate_term=vote_req.term,
        );
        vec![Action::OutgoingRpc(RpcMessage::vote(Vote {
            request_id: vote_req.request_id,
            from: self.server_id,
            to: vote_req.from,
            term: storage.current_term(),
            vote_granted: false,
        }))]
    }
}

impl NodeState<Leader> {
    fn send_leader_heartbeat_to_cluster<C, PS>(
        &mut self,
        storage: &PS,
        config: &RaftConfig,
    ) -> Vec<Action<C>>
    where
        C: LogCommand,
        PS: PersistentStorage<C>,
    {
        let mut actions = Vec::new();

        trace!("Sending heartbeat to cluster...");

        for other_server in &self.other_servers {
            actions.push(Action::OutgoingRpc(RpcMessage::append_entries(
                AppendEntries {
                    request_id: Uuid::new_v4(),
                    from: self.server_id,
                    to: *other_server,
                    term: storage.current_term(),
                    // TODO: Last log item index
                    // TODO: missing prev log term
                    prev_log_index: LogIndex(0),
                    prev_log_term: TermIndex(0),
                    entries: Vec::new(),
                    leader_commit: self.commit_index,
                },
            )));
        }

        self.inner.last_heartbeat_sent = self.current_time;

        actions.push(Action::SetNextTimeout(config.leader_heartbeat_interval));

        actions
    }
}

impl Transitions for NodeState<Leader> {
    fn handle_event<C, PS>(
        mut self,
        event: Event<C>,
        storage: &mut PS,
        config: &RaftConfig,
        _: &mut ChaCha8Rng,
    ) -> Result<(Node, Vec<Action<C>>), PersistentStorageError>
    where
        C: LogCommand,
        PS: PersistentStorage<C>,
    {
        match event {
            Event::Tick(now) => {
                let maybe_heartbeat =
                    if now > self.inner.last_heartbeat_sent + config.leader_heartbeat_interval {
                        self.send_leader_heartbeat_to_cluster(storage, config)
                    } else {
                        vec![]
                    };

                Ok((self.into(), maybe_heartbeat))
            }

            Event::LogEntryAppliedByApplication(_) => todo!(),

            Event::IncomingRpc(RpcMessage::Request(rpc_req)) => match rpc_req {
                Request::RequestVote(req) => {
                    let vote = self.vote_no(storage, req, "I am the leader");
                    Ok((self.into(), vote))
                }

                Request::AppendEntries(req) => {
                    if req.term == storage.current_term() {
                        unreachable!("BUG: Leader should not receive append entries from another leader with same term")
                    } else if req.term < storage.current_term() {
                        let ack = self.ack_append_entries(storage, req, false);
                        Ok((self.into(), ack))
                    } else {
                        unreachable!("BUG: If leader receives an append entries from a higher term, it should have become a follower already")
                    }
                }
            },
            Event::IncomingRpc(RpcMessage::Reply(reply)) => match reply {
                ReplyTo::AppendEntries(_) => {
                    // TODO: When we receive an append entries ack, we should update the next index and match index for the node that sent the ack
                    // TODO: We should also resend append entries if the follower is behind in the log
                    Ok((self.into(), vec![]))
                }

                ReplyTo::RequestVote(_) => Ok((self.into(), vec![])),
            },
        }
    }
}

has_election_timer!(Candidate);
impl NodeState<Candidate> {
    fn start_new_election<PS, C>(
        &mut self,
        config: &RaftConfig,
        storage: &mut PS,
        rng: &mut ChaCha8Rng,
    ) -> Result<Vec<Action<C>>, PersistentStorageError>
    where
        PS: PersistentStorage<C>,
        C: LogCommand,
    {
        trace!(
            "{server_id:?}: Starting new election!",
            server_id = self.server_id
        );
        storage
            .update_term(storage.current_term().increment())
            .record_vote(self.server_id)
            .sync()?;

        let election_timeout = self.reset_election_timer(config, rng);
        self.inner.votes_received = HashSet::new();
        self.inner.votes_received.insert(self.server_id);

        let mut start_tick_timer_and_request_votes = vec![Action::SetNextTimeout(election_timeout)];

        for other_server in self.other_servers.iter() {
            start_tick_timer_and_request_votes.push(Action::OutgoingRpc(RpcMessage::request_vote(
                RequestVote {
                    request_id: Uuid::new_v4(),
                    from: self.server_id,
                    to: *other_server,
                    term: storage.current_term(),
                    // TODO: Use last log entry for these
                    last_log_index: LogIndex(0),
                    last_log_term: TermIndex(0),
                },
            )));
        }
        Ok(start_tick_timer_and_request_votes)
    }
}

impl Transitions for NodeState<Candidate> {
    fn handle_event<C, PS>(
        mut self,
        event: Event<C>,
        storage: &mut PS,
        config: &RaftConfig,
        rng: &mut ChaCha8Rng,
    ) -> Result<(Node, Vec<Action<C>>), PersistentStorageError>
    where
        C: LogCommand,
        PS: PersistentStorage<C>,
    {
        match event {
            Event::Tick(now) => {
                let maybe_vote_requests = if now
                    > self.inner.last_election_timer_started + self.inner.election_timeout
                {
                    trace!(
                        "{server_id:?}: In candidate mode, did not receive enough votes before election timeout {timeout:?}ms, starting new election",
                        server_id = self.server_id,
                        timeout=self.inner.election_timeout.as_millis()
                    );
                    self.start_new_election(config, storage, rng)?
                } else {
                    vec![]
                };

                Ok((self.into(), maybe_vote_requests))
            }

            Event::LogEntryAppliedByApplication(_) => todo!(),

            Event::IncomingRpc(RpcMessage::Request(rpc_req)) => match rpc_req {
                Request::RequestVote(req) => {
                    let vote_no_reason = if req.term < storage.current_term() {
                        "I am in a higher term than this vote request"
                    } else {
                        "I am a candidate for the same term"
                    };
                    let vote = self.vote_no(storage, req, vote_no_reason);
                    Ok((self.into(), vote))
                }

                Request::AppendEntries(req) => {
                    if req.term < storage.current_term() {
                        let ack = self.ack_append_entries(storage, req, false);
                        Ok((self.into(), ack))
                    } else if req.term == storage.current_term() {
                        let mut follower_state: NodeState<Follower> = self.transition_to();
                        let ack = follower_state.ack_append_entries(storage, req, true);
                        follower_state.reset_election_timer(config, rng);
                        Ok((follower_state.into(), ack))
                    } else {
                        unreachable!("BUG: If candidate receives an append entries from a higher term, it should have become a follower already")
                    }
                }
            },

            Event::IncomingRpc(RpcMessage::Reply(reply)) => match reply {
                ReplyTo::RequestVote(vote) => {
                    let qorum = DivCeil::div_ceil(self.other_servers.len() + 1, 2);

                    if vote.term == storage.current_term() && vote.vote_granted {
                        self.inner.votes_received.insert(vote.from);

                        if self.inner.votes_received.len() >= qorum {
                            info!(
                                "{server_id:?}: Received vote from {from:?} and won election with {votes:?} votes, becoming leader in term {term:?}",
                                server_id=self.server_id,
                                from=vote.from,
                                votes = self.inner.votes_received,
                                term=storage.current_term()
                            );
                            let mut new_state: NodeState<Leader> = self.transition_to();
                            let actions =
                                new_state.send_leader_heartbeat_to_cluster(storage, config);
                            Ok((new_state.into(), actions))
                        } else {
                            self.inner.votes_received.insert(vote.from);
                            info!(
                                "{server_id:?}: Received vote from {from:?}, but still need {votes_needed:?} more votes to win election in term {term:?}",
                                server_id=self.server_id,
                                from=vote.from,
                                votes_needed=qorum - self.inner.votes_received.len(),
                                term=storage.current_term()
                            );
                            Ok((self.into(), vec![]))
                        }
                    } else {
                        Ok((self.into(), vec![]))
                    }
                }

                ReplyTo::AppendEntries(_) => Ok((self.into(), vec![])),
            },
        }
    }
}

pub(crate) struct FirstElectionTimeout(pub(crate) Duration);

has_election_timer!(Follower);
impl NodeState<Follower> {
    pub(crate) fn new(
        server_id: ServerId,
        other_servers: HashSet<ServerId>,
        config: &RaftConfig,
        rng: &mut ChaCha8Rng,
    ) -> (Self, FirstElectionTimeout) {
        let follower_state = Follower::new();

        let mut node_state = Self {
            start_time: system_clock::now(),
            current_time: system_clock::now(),
            server_id,
            other_servers,
            commit_index: LogIndex(0),
            last_applied: LogIndex(0),
            inner: follower_state,
        };
        let election_timeout = node_state.reset_election_timer(config, rng);
        (node_state, FirstElectionTimeout(election_timeout))
    }

    fn vote_in_election<C, PS>(
        &mut self,
        storage: &mut PS,
        vote_req: RequestVote,
    ) -> Result<Vec<Action<C>>, PersistentStorageError>
    where
        C: LogCommand,
        PS: PersistentStorage<C>,
    {
        self.inner.leader_id = None;

        // Reply false if term < currentTerm (§5.1)
        // If votedFor is null or candidateId, and candidate’s log is at
        // least as up-to-date as receiver’s log (TODO), grant vote (§5.2, §5.4)
        let candidate_has_same_or_newer_term = vote_req.term >= storage.current_term();
        let have_we_voted_for_this_term = storage.voted_for_in_current_term().is_some();
        let voted_for_same_candidate_already = storage
            .voted_for_in_current_term()
            .map(|_| true)
            .unwrap_or(false);

        let vote_granted = candidate_has_same_or_newer_term
            && (!have_we_voted_for_this_term || voted_for_same_candidate_already);

        if vote_granted {
            info!(
                "{server_id:?}: Voting for candidate {candidate_id:?} in term {term:?}",
                server_id = self.server_id,
                candidate_id = vote_req.from,
                term = vote_req.term
            );
            storage.record_vote(vote_req.from).sync()?;
        }

        Ok(vec![Action::OutgoingRpc(RpcMessage::vote(Vote {
            request_id: vote_req.request_id,
            from: self.server_id,
            to: vote_req.from,
            term: storage.current_term(),
            vote_granted,
        }))])
    }
}

impl Transitions for NodeState<Follower> {
    fn handle_event<C, PS>(
        mut self,
        event: Event<C>,
        storage: &mut PS,
        config: &RaftConfig,
        rng: &mut ChaCha8Rng,
    ) -> Result<(Node, Vec<Action<C>>), PersistentStorageError>
    where
        C: LogCommand,
        PS: PersistentStorage<C>,
    {
        match event {
            Event::Tick(now) => {
                if now > self.inner.last_election_timer_started + self.inner.election_timeout {
                    info!(
                        "{server_id:?}: In follower state, did not receive heartbeat before election timeout {timeout:?}ms, becoming candidate...",
                        server_id=self.server_id,
                        timeout=self.inner.election_timeout.as_millis(),
                    );
                    let mut new_state: NodeState<Candidate> = self.transition_to();
                    let vote_requests = new_state.start_new_election(config, storage, rng)?;
                    Ok((new_state.into(), vote_requests))
                } else {
                    Ok((self.into(), vec![]))
                }
            }

            Event::LogEntryAppliedByApplication(_) => todo!(),

            Event::IncomingRpc(RpcMessage::Request(rpc_req)) => match rpc_req {
                Request::RequestVote(req) => {
                    let vote;
                    if req.term < storage.current_term() {
                        vote = self.vote_no(storage, req, "term is less than current term");
                    } else {
                        self.inner.leader_id = None;
                        vote = self.vote_in_election(storage, req)?;
                    }
                    Ok((self.into(), vote))
                }

                Request::AppendEntries(req) => {
                    let (ack_success, mut maybe_start_timer) = if req.term < storage.current_term()
                    {
                        (false, vec![])
                    } else {
                        self.inner.leader_id = Some(req.from);
                        //TODO: Check if we have all entries up to leader term & prev log index, and replicate entries
                        let election_timeout = self.reset_election_timer(config, rng);
                        (true, vec![Action::SetNextTimeout(election_timeout)])
                    };
                    let mut maybe_start_timer_and_ack =
                        self.ack_append_entries(storage, req, ack_success);
                    maybe_start_timer_and_ack.append(&mut maybe_start_timer);
                    Ok((self.into(), maybe_start_timer_and_ack))
                }
            },

            // Followers don't send out RPCs so ignore replies, this can only happen for rpc responses delivered late
            Event::IncomingRpc(RpcMessage::Reply(_)) => Ok((self.into(), vec![])),
        }
    }
}

impl<InState, OutState> CanTransitionTo<OutState> for NodeState<InState>
where
    InState: State,
    OutState: State + From<InState>,
{
    fn transition_to(self) -> NodeState<OutState> {
        NodeState {
            inner: self.inner.into(),
            server_id: self.server_id,
            start_time: self.start_time,
            current_time: self.current_time,
            other_servers: self.other_servers,
            commit_index: self.commit_index,
            last_applied: self.last_applied,
        }
    }
}
