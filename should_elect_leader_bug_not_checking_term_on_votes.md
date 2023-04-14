Look we have two leaders, how?

```
TIME 67975ms: SEND AppendEntriesReply from ServerId(3) to ServerId(2) for term TermIndex(85) with latency 2ms
TIME 67975ms: SEND AppendEntriesReply from ServerId(4) to ServerId(2) for term TermIndex(85) with latency 70ms
TIME 67975ms: SEND AppendEntriesReply from ServerId(1) to ServerId(2) for term TermIndex(85) with latency 29ms
TIME 67975ms: SEND AppendEntriesReply from ServerId(0) to ServerId(2) for term TermIndex(85) with latency 2ms
TIME 67975ms: SEND AppendEntries from ServerId(3) to ServerId(0) for term TermIndex(85) with latency 121ms
TIME 67975ms: SEND AppendEntries from ServerId(3) to ServerId(1) for term TermIndex(85) with latency 20ms
TIME 67975ms: SEND AppendEntries from ServerId(3) to ServerId(4) for term TermIndex(85) with latency 50ms
TIME 67975ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(85), voted_for: Some((TermIndex(85), ServerId(3))), leader_for_term: Some(ServerId(2)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(85), voted_for: Some((TermIndex(85), ServerId(3))), leader_for_term: Some(ServerId(2)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Leader, current_term: TermIndex(85), voted_for: Some((TermIndex(85), ServerId(2))), leader_for_term: Some(ServerId(2)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Leader, current_term: TermIndex(85), voted_for: Some((TermIndex(85), ServerId(3))), leader_for_term: Some(ServerId(3)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(85), voted_for: Some((TermIndex(85), ServerId(2))), leader_for_term: Some(ServerId(2)) }
```

From the app logs we see both servers think they won the same term:

```
[2m2023-04-14T05:33:51.647396Z[0m [32m INFO[0m [2mraft_consensus::state_machine[0m[2m:[0m ServerId(2): Received vote from ServerId(4) and won election with {ServerId(2), ServerId(1), ServerId(4)} votes, becoming leader in term TermIndex(85)
[2m2023-04-14T05:33:51.654789Z[0m [32m INFO[0m [2mraft_consensus::state_machine[0m[2m:[0m ServerId(3): Received vote from ServerId(1) and won election with {ServerId(3), ServerId(0), ServerId(1)} votes, becoming leader in term TermIndex(85)
```

How did they both get server 1's vote?

- We ignore votes when we've already voted, check!
- We ignore votes when its from an older term, right? Turns out we don't!

Update from this:

```
if vote.vote_granted {
    self.inner.votes_received.insert(vote.from);
```

To this:

```
if vote.term == storage.current_term() && vote.vote_granted {
    self.inner.votes_received.insert(vote.from);
```
