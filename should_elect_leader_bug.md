```
TIME 181ms: ServerStates...
TIME 181ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(0), voted_for: None, leader_for_term: None }
TIME 380ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Candidate, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(0), voted_for: None, leader_for_term: None }
TIME 380ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Candidate, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(0), voted_for: None, leader_for_term: None }
TIME 380ms: RequestVote from ServerId(0) to ServerId(3) for term TermIndex(1)
TIME 380ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Candidate, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(0), voted_for: None, leader_for_term: None }
TIME 380ms: RequestVote from ServerId(0) to ServerId(1) for term TermIndex(1)
TIME 380ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Candidate, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(0), voted_for: None, leader_for_term: None }
TIME 382ms: RequestVote from ServerId(0) to ServerId(4) for term TermIndex(1)
TIME 382ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Candidate, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(0), voted_for: None, leader_for_term: None }
TIME 385ms: RequestVote from ServerId(0) to ServerId(2) for term TermIndex(1)
TIME 385ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Candidate, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(0), voted_for: None, leader_for_term: None }
TIME 572ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Candidate, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(0), voted_for: None, leader_for_term: None }
TIME 574ms: RequestVoteReply from ServerId(4) to ServerId(0) for term TermIndex(1)
TIME 574ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Candidate, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(0), voted_for: None, leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
TIME 575ms: RequestVoteReply from ServerId(3) to ServerId(0) for term TermIndex(1)
TIME 575ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Candidate, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(0), voted_for: None, leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
TIME 577ms: RequestVoteReply from ServerId(2) to ServerId(0) for term TermIndex(1)
TIME 577ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Candidate, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(0), voted_for: None, leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
TIME 577ms: RequestVote from ServerId(1) to ServerId(4) for term TermIndex(1)
TIME 577ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Candidate, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
TIME 578ms: RequestVote from ServerId(1) to ServerId(3) for term TermIndex(1)
TIME 578ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Candidate, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
TIME 578ms: RequestVote from ServerId(1) to ServerId(0) for term TermIndex(1)
TIME 578ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Candidate, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
TIME 584ms: RequestVoteReply from ServerId(1) to ServerId(0) for term TermIndex(1)
TIME 584ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Candidate, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
TIME 601ms: RequestVote from ServerId(1) to ServerId(2) for term TermIndex(1)
TIME 601ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Candidate, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
TIME 609ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
TIME 609ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
TIME 609ms: AppendEntries from ServerId(0) to ServerId(4) for term TermIndex(1)
TIME 609ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
TIME 613ms: RequestVoteReply from ServerId(4) to ServerId(1) for term TermIndex(1)
TIME 613ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
TIME 616ms: RequestVoteReply from ServerId(0) to ServerId(1) for term TermIndex(1)
TIME 616ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
TIME 616ms: AppendEntries from ServerId(0) to ServerId(2) for term TermIndex(1)
TIME 616ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
TIME 618ms: RequestVoteReply from ServerId(2) to ServerId(1) for term TermIndex(1)
TIME 618ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
TIME 618ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
TIME 618ms: AppendEntriesReply from ServerId(4) to ServerId(0) for term TermIndex(1)
TIME 618ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 635ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 635ms: RequestVoteReply from ServerId(3) to ServerId(1) for term TermIndex(1)
TIME 635ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 670ms: AppendEntries from ServerId(0) to ServerId(3) for term TermIndex(1)
TIME 670ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 709ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 709ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 709ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 709ms: AppendEntriesReply from ServerId(3) to ServerId(0) for term TermIndex(1)
TIME 709ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 709ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 709ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 735ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 735ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 735ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 735ms: AppendEntries from ServerId(0) to ServerId(4) for term TermIndex(1)
TIME 735ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 737ms: AppendEntries from ServerId(0) to ServerId(3) for term TermIndex(1)
TIME 737ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 737ms: AppendEntries from ServerId(0) to ServerId(1) for term TermIndex(1)
TIME 737ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 739ms: AppendEntries from ServerId(0) to ServerId(2) for term TermIndex(1)
TIME 739ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 744ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 744ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 744ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 744ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 744ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 744ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 745ms: AppendEntriesReply from ServerId(4) to ServerId(0) for term TermIndex(1)
TIME 745ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 748ms: AppendEntriesReply from ServerId(3) to ServerId(0) for term TermIndex(1)
TIME 748ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 758ms: AppendEntriesReply from ServerId(2) to ServerId(0) for term TermIndex(1)
TIME 758ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 764ms: AppendEntries from ServerId(0) to ServerId(1) for term TermIndex(1)
TIME 764ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 775ms: AppendEntriesReply from ServerId(1) to ServerId(0) for term TermIndex(1)
TIME 775ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 796ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 796ms: AppendEntriesReply from ServerId(1) to ServerId(0) for term TermIndex(1)
TIME 796ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 834ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 834ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 835ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 835ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 835ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 835ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 835ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 835ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 835ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 852ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 852ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 852ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 854ms: AppendEntries from ServerId(0) to ServerId(2) for term TermIndex(1)
TIME 854ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 855ms: AppendEntries from ServerId(0) to ServerId(4) for term TermIndex(1)
TIME 855ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 865ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 865ms: AppendEntriesReply from ServerId(4) to ServerId(0) for term TermIndex(1)
TIME 865ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 868ms: AppendEntries from ServerId(0) to ServerId(1) for term TermIndex(1)
TIME 868ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 869ms: AppendEntriesReply from ServerId(2) to ServerId(0) for term TermIndex(1)
TIME 869ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 869ms: AppendEntries from ServerId(0) to ServerId(3) for term TermIndex(1)
TIME 869ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 898ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 898ms: AppendEntriesReply from ServerId(1) to ServerId(0) for term TermIndex(1)
TIME 898ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 902ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 903ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 911ms: AppendEntriesReply from ServerId(3) to ServerId(0) for term TermIndex(1)
TIME 911ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 952ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 952ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 952ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 952ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 952ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 984ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 984ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 984ms: AppendEntries from ServerId(0) to ServerId(3) for term TermIndex(1)
TIME 984ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 985ms: AppendEntries from ServerId(0) to ServerId(2) for term TermIndex(1)
TIME 985ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 985ms: AppendEntries from ServerId(0) to ServerId(1) for term TermIndex(1)
TIME 985ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 987ms: AppendEntries from ServerId(0) to ServerId(4) for term TermIndex(1)
TIME 987ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1026ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1027ms: AppendEntriesReply from ServerId(4) to ServerId(0) for term TermIndex(1)
TIME 1027ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1027ms: AppendEntriesReply from ServerId(1) to ServerId(0) for term TermIndex(1)
TIME 1027ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1038ms: AppendEntriesReply from ServerId(2) to ServerId(0) for term TermIndex(1)
TIME 1038ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1050ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1055ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1063ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1071ms: AppendEntriesReply from ServerId(3) to ServerId(0) for term TermIndex(1)
TIME 1071ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1084ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1084ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1084ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1084ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1084ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1084ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1135ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1135ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1136ms: AppendEntries from ServerId(0) to ServerId(1) for term TermIndex(1)
TIME 1136ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1141ms: AppendEntries from ServerId(0) to ServerId(3) for term TermIndex(1)
TIME 1141ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1143ms: AppendEntries from ServerId(0) to ServerId(2) for term TermIndex(1)
TIME 1143ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1147ms: AppendEntries from ServerId(0) to ServerId(4) for term TermIndex(1)
TIME 1147ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1186ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1186ms: AppendEntriesReply from ServerId(4) to ServerId(0) for term TermIndex(1)
TIME 1186ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1188ms: AppendEntriesReply from ServerId(2) to ServerId(0) for term TermIndex(1)
TIME 1188ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1191ms: AppendEntriesReply from ServerId(1) to ServerId(0) for term TermIndex(1)
TIME 1191ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1193ms: AppendEntriesReply from ServerId(3) to ServerId(0) for term TermIndex(1)
TIME 1193ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1200ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1235ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1235ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1235ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1235ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1235ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1235ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1297ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1297ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1301ms: AppendEntries from ServerId(0) to ServerId(4) for term TermIndex(1)
TIME 1301ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1306ms: AppendEntries from ServerId(0) to ServerId(1) for term TermIndex(1)
TIME 1306ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1310ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1313ms: AppendEntries from ServerId(0) to ServerId(3) for term TermIndex(1)
TIME 1313ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1314ms: AppendEntriesReply from ServerId(4) to ServerId(0) for term TermIndex(1)
TIME 1314ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1332ms: AppendEntriesReply from ServerId(1) to ServerId(0) for term TermIndex(1)
TIME 1332ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1349ms: AppendEntries from ServerId(0) to ServerId(2) for term TermIndex(1)
TIME 1349ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1386ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1386ms: AppendEntriesReply from ServerId(2) to ServerId(0) for term TermIndex(1)
TIME 1386ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1388ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1396ms: AppendEntriesReply from ServerId(3) to ServerId(0) for term TermIndex(1)
TIME 1396ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1397ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1397ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1397ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1397ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1397ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1426ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1426ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1426ms: AppendEntries from ServerId(0) to ServerId(1) for term TermIndex(1)
TIME 1426ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1426ms: AppendEntries from ServerId(0) to ServerId(3) for term TermIndex(1)
TIME 1426ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1426ms: AppendEntries from ServerId(0) to ServerId(2) for term TermIndex(1)
TIME 1426ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1460ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1460ms: AppendEntriesReply from ServerId(2) to ServerId(0) for term TermIndex(1)
TIME 1460ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1464ms: AppendEntriesReply from ServerId(3) to ServerId(0) for term TermIndex(1)
TIME 1464ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1468ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1481ms: AppendEntriesReply from ServerId(1) to ServerId(0) for term TermIndex(1)
TIME 1481ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1485ms: AppendEntries from ServerId(0) to ServerId(4) for term TermIndex(1)
TIME 1485ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1516ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1516ms: AppendEntriesReply from ServerId(4) to ServerId(0) for term TermIndex(1)
TIME 1516ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1526ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1526ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1526ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1526ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1526ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1564ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1564ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1566ms: AppendEntries from ServerId(0) to ServerId(2) for term TermIndex(1)
TIME 1566ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1569ms: AppendEntries from ServerId(0) to ServerId(3) for term TermIndex(1)
TIME 1569ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1570ms: AppendEntries from ServerId(0) to ServerId(4) for term TermIndex(1)
TIME 1570ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1577ms: AppendEntries from ServerId(0) to ServerId(1) for term TermIndex(1)
TIME 1577ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1602ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1602ms: AppendEntriesReply from ServerId(4) to ServerId(0) for term TermIndex(1)
TIME 1602ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1603ms: AppendEntriesReply from ServerId(2) to ServerId(0) for term TermIndex(1)
TIME 1603ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1604ms: AppendEntriesReply from ServerId(1) to ServerId(0) for term TermIndex(1)
TIME 1604ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1612ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1664ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1664ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1664ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1664ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1664ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1673ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1673ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1674ms: AppendEntries from ServerId(0) to ServerId(4) for term TermIndex(1)
TIME 1674ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1675ms: AppendEntries from ServerId(0) to ServerId(1) for term TermIndex(1)
TIME 1675ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1687ms: AppendEntries from ServerId(0) to ServerId(2) for term TermIndex(1)
TIME 1687ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1702ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1702ms: AppendEntriesReply from ServerId(2) to ServerId(0) for term TermIndex(1)
TIME 1702ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1703ms: AppendEntriesReply from ServerId(4) to ServerId(0) for term TermIndex(1)
TIME 1703ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1743ms: AppendEntriesReply from ServerId(1) to ServerId(0) for term TermIndex(1)
TIME 1743ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1773ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1773ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1776ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1776ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1776ms: AppendEntries from ServerId(0) to ServerId(1) for term TermIndex(1)
TIME 1776ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1777ms: AppendEntries from ServerId(0) to ServerId(4) for term TermIndex(1)
TIME 1777ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1777ms: AppendEntries from ServerId(0) to ServerId(3) for term TermIndex(1)
TIME 1777ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1781ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1795ms: AppendEntriesReply from ServerId(3) to ServerId(0) for term TermIndex(1)
TIME 1795ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1797ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1800ms: AppendEntriesReply from ServerId(3) to ServerId(0) for term TermIndex(1)
TIME 1800ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1824ms: AppendEntries from ServerId(0) to ServerId(2) for term TermIndex(1)
TIME 1824ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1834ms: AppendEntriesReply from ServerId(4) to ServerId(0) for term TermIndex(1)
TIME 1834ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1864ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1864ms: AppendEntriesReply from ServerId(2) to ServerId(0) for term TermIndex(1)
TIME 1864ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1876ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1876ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1876ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1876ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1876ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1882ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1882ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1882ms: AppendEntries from ServerId(0) to ServerId(3) for term TermIndex(1)
TIME 1882ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1885ms: AppendEntries from ServerId(0) to ServerId(2) for term TermIndex(1)
TIME 1885ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1895ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1896ms: AppendEntries from ServerId(0) to ServerId(1) for term TermIndex(1)
TIME 1896ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1896ms: AppendEntriesReply from ServerId(2) to ServerId(0) for term TermIndex(1)
TIME 1896ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1906ms: AppendEntriesReply from ServerId(3) to ServerId(0) for term TermIndex(1)
TIME 1906ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1951ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1951ms: AppendEntriesReply from ServerId(1) to ServerId(0) for term TermIndex(1)
TIME 1951ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1958ms: AppendEntries from ServerId(0) to ServerId(4) for term TermIndex(1)
TIME 1958ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1969ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1979ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1982ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1982ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1982ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1982ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1982ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1985ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1985ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1986ms: AppendEntries from ServerId(0) to ServerId(3) for term TermIndex(1)
TIME 1986ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 1992ms: AppendEntries from ServerId(0) to ServerId(2) for term TermIndex(1)
TIME 1992ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2007ms: AppendEntriesReply from ServerId(4) to ServerId(0) for term TermIndex(1)
TIME 2007ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2014ms: AppendEntries from ServerId(0) to ServerId(4) for term TermIndex(1)
TIME 2014ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2079ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2079ms: AppendEntriesReply from ServerId(4) to ServerId(0) for term TermIndex(1)
TIME 2079ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2080ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2080ms: AppendEntriesReply from ServerId(2) to ServerId(0) for term TermIndex(1)
TIME 2080ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2083ms: AppendEntriesReply from ServerId(3) to ServerId(0) for term TermIndex(1)
TIME 2083ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2085ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2085ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2085ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2085ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2141ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2141ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2142ms: AppendEntries from ServerId(0) to ServerId(4) for term TermIndex(1)
TIME 2142ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2142ms: AppendEntries from ServerId(0) to ServerId(2) for term TermIndex(1)
TIME 2142ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2158ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2158ms: AppendEntriesReply from ServerId(4) to ServerId(0) for term TermIndex(1)
TIME 2158ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2161ms: AppendEntriesReply from ServerId(2) to ServerId(0) for term TermIndex(1)
TIME 2161ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2167ms: AppendEntriesReply from ServerId(2) to ServerId(0) for term TermIndex(1)
TIME 2167ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2173ms: AppendEntries from ServerId(0) to ServerId(1) for term TermIndex(1)
TIME 2173ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2174ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2174ms: AppendEntriesReply from ServerId(1) to ServerId(0) for term TermIndex(1)
TIME 2174ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2192ms: AppendEntries from ServerId(0) to ServerId(3) for term TermIndex(1)
TIME 2192ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2212ms: AppendEntries from ServerId(0) to ServerId(3) for term TermIndex(1)
TIME 2212ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2224ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2224ms: AppendEntriesReply from ServerId(3) to ServerId(0) for term TermIndex(1)
TIME 2224ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2227ms: AppendEntriesReply from ServerId(3) to ServerId(0) for term TermIndex(1)
TIME 2227ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2233ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2241ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2241ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2241ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2241ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2241ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2241ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2241ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2241ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2322ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2322ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2322ms: AppendEntries from ServerId(0) to ServerId(2) for term TermIndex(1)
TIME 2322ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2322ms: AppendEntries from ServerId(0) to ServerId(1) for term TermIndex(1)
TIME 2322ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2322ms: AppendEntriesReply from ServerId(2) to ServerId(0) for term TermIndex(1)
TIME 2322ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2324ms: AppendEntries from ServerId(0) to ServerId(4) for term TermIndex(1)
TIME 2324ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2324ms: AppendEntries from ServerId(0) to ServerId(3) for term TermIndex(1)
TIME 2324ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2341ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2342ms: AppendEntriesReply from ServerId(3) to ServerId(0) for term TermIndex(1)
TIME 2342ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2343ms: AppendEntriesReply from ServerId(4) to ServerId(0) for term TermIndex(1)
TIME 2343ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2344ms: AppendEntriesReply from ServerId(1) to ServerId(0) for term TermIndex(1)
TIME 2344ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2365ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2369ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2419ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2422ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2422ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2422ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2422ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2422ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2422ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2455ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2455ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2456ms: AppendEntries from ServerId(0) to ServerId(2) for term TermIndex(1)
TIME 2456ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2460ms: AppendEntries from ServerId(0) to ServerId(3) for term TermIndex(1)
TIME 2460ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2462ms: AppendEntries from ServerId(0) to ServerId(4) for term TermIndex(1)
TIME 2462ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2466ms: AppendEntries from ServerId(0) to ServerId(1) for term TermIndex(1)
TIME 2466ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2504ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2504ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2505ms: AppendEntriesReply from ServerId(2) to ServerId(0) for term TermIndex(1)
TIME 2505ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2505ms: AppendEntriesReply from ServerId(1) to ServerId(0) for term TermIndex(1)
TIME 2505ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2507ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2513ms: AppendEntriesReply from ServerId(3) to ServerId(0) for term TermIndex(1)
TIME 2513ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2521ms: AppendEntries from ServerId(0) to ServerId(1) for term TermIndex(1)
TIME 2521ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2542ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2543ms: AppendEntriesReply from ServerId(1) to ServerId(0) for term TermIndex(1)
TIME 2543ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2548ms: AppendEntriesReply from ServerId(4) to ServerId(0) for term TermIndex(1)
TIME 2548ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2554ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2555ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2555ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2555ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2555ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2555ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2555ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2555ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2675ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2675ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2675ms: AppendEntries from ServerId(0) to ServerId(3) for term TermIndex(1)
TIME 2675ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 2679ms: AppendEntries from ServerId(0) to ServerId(2) for term TermIndex(1)
TIME 2679ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Candidate, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
TIME 2679ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Candidate, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
TIME 2679ms: AppendEntriesReply from ServerId(3) to ServerId(0) for term TermIndex(1)
TIME 2679ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Candidate, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
TIME 2680ms: RequestVote from ServerId(4) to ServerId(0) for term TermIndex(2)
TIME 2680ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Candidate, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
TIME 2681ms: RequestVote from ServerId(4) to ServerId(1) for term TermIndex(2)
TIME 2681ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Candidate, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
TIME 2687ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Candidate, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
TIME 2687ms: RequestVoteReply from ServerId(1) to ServerId(4) for term TermIndex(2)
TIME 2687ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Candidate, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
TIME 2691ms: AppendEntriesReply from ServerId(2) to ServerId(0) for term TermIndex(1)
TIME 2691ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Candidate, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
TIME 2700ms: AppendEntries from ServerId(0) to ServerId(4) for term TermIndex(1)
TIME 2700ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Candidate, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
TIME 2700ms: AppendEntries from ServerId(0) to ServerId(1) for term TermIndex(1)
TIME 2700ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Candidate, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
TIME 2709ms: RequestVote from ServerId(4) to ServerId(3) for term TermIndex(2)
TIME 2709ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Candidate, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
TIME 2753ms: RequestVote from ServerId(4) to ServerId(2) for term TermIndex(2)
TIME 2753ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Candidate, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
TIME 2764ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(1), voted_for: Some((TermIndex(1), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Candidate, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
TIME 2765ms: RequestVoteReply from ServerId(3) to ServerId(4) for term TermIndex(2)
TIME 2765ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Candidate, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
TIME 2768ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Candidate, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
TIME 2768ms: AppendEntries from ServerId(4) to ServerId(1) for term TermIndex(2)
TIME 2768ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 2768ms: AppendEntries from ServerId(4) to ServerId(0) for term TermIndex(2)
TIME 2768ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 2769ms: AppendEntriesReply from ServerId(4) to ServerId(0) for term TermIndex(2)
TIME 2769ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 2771ms: AppendEntries from ServerId(4) to ServerId(2) for term TermIndex(2)
TIME 2771ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 2775ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 2775ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 2776ms: AppendEntriesReply from ServerId(1) to ServerId(4) for term TermIndex(2)
TIME 2776ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 2777ms: AppendEntriesReply from ServerId(2) to ServerId(4) for term TermIndex(2)
TIME 2777ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 2780ms: RequestVoteReply from ServerId(2) to ServerId(4) for term TermIndex(2)
TIME 2780ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 2785ms: AppendEntriesReply from ServerId(0) to ServerId(4) for term TermIndex(2)
TIME 2785ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 2787ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 2792ms: AppendEntriesReply from ServerId(1) to ServerId(0) for term TermIndex(2)
TIME 2792ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 2806ms: AppendEntries from ServerId(4) to ServerId(3) for term TermIndex(2)
TIME 2806ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 2841ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 2841ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 2847ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 2847ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 2850ms: AppendEntriesReply from ServerId(3) to ServerId(4) for term TermIndex(2)
TIME 2850ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 2868ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 2868ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 2868ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 2868ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 2868ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 2868ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 2895ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 2895ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 2895ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 2895ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 2895ms: AppendEntries from ServerId(4) to ServerId(2) for term TermIndex(2)
TIME 2895ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 2898ms: AppendEntries from ServerId(4) to ServerId(1) for term TermIndex(2)
TIME 2898ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 2902ms: AppendEntries from ServerId(4) to ServerId(0) for term TermIndex(2)
TIME 2902ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 2923ms: AppendEntries from ServerId(4) to ServerId(3) for term TermIndex(2)
TIME 2923ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 2924ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 2924ms: AppendEntriesReply from ServerId(0) to ServerId(4) for term TermIndex(2)
TIME 2924ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 2927ms: AppendEntriesReply from ServerId(1) to ServerId(4) for term TermIndex(2)
TIME 2927ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 2932ms: AppendEntriesReply from ServerId(2) to ServerId(4) for term TermIndex(2)
TIME 2932ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 2937ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 2960ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 2964ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 2973ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 2973ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 2973ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 2992ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 2995ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 2995ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 2995ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 2995ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 2995ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 2998ms: RequestVoteReply from ServerId(0) to ServerId(4) for term TermIndex(2)
TIME 2998ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 2998ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 2998ms: AppendEntries from ServerId(4) to ServerId(2) for term TermIndex(2)
TIME 2998ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 2999ms: AppendEntries from ServerId(4) to ServerId(3) for term TermIndex(2)
TIME 2999ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 2999ms: AppendEntries from ServerId(4) to ServerId(1) for term TermIndex(2)
TIME 2999ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3007ms: AppendEntries from ServerId(4) to ServerId(0) for term TermIndex(2)
TIME 3007ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3024ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3025ms: AppendEntriesReply from ServerId(0) to ServerId(4) for term TermIndex(2)
TIME 3025ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3027ms: AppendEntriesReply from ServerId(1) to ServerId(4) for term TermIndex(2)
TIME 3027ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3030ms: AppendEntriesReply from ServerId(3) to ServerId(4) for term TermIndex(2)
TIME 3030ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3092ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3097ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3098ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3098ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3098ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3098ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3098ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3098ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3105ms: AppendEntriesReply from ServerId(2) to ServerId(4) for term TermIndex(2)
TIME 3105ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3105ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3105ms: AppendEntries from ServerId(4) to ServerId(3) for term TermIndex(2)
TIME 3105ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3105ms: AppendEntries from ServerId(4) to ServerId(2) for term TermIndex(2)
TIME 3105ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3106ms: AppendEntries from ServerId(4) to ServerId(0) for term TermIndex(2)
TIME 3106ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3131ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3140ms: AppendEntriesReply from ServerId(3) to ServerId(4) for term TermIndex(2)
TIME 3140ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3141ms: AppendEntries from ServerId(4) to ServerId(1) for term TermIndex(2)
TIME 3141ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3141ms: AppendEntriesReply from ServerId(0) to ServerId(4) for term TermIndex(2)
TIME 3141ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3143ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3143ms: AppendEntriesReply from ServerId(2) to ServerId(4) for term TermIndex(2)
TIME 3143ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3145ms: AppendEntriesReply from ServerId(3) to ServerId(4) for term TermIndex(2)
TIME 3145ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3146ms: AppendEntriesReply from ServerId(1) to ServerId(4) for term TermIndex(2)
TIME 3146ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3179ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3201ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3205ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3205ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3205ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3205ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3205ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3205ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3205ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3205ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3221ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3221ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3221ms: AppendEntries from ServerId(4) to ServerId(2) for term TermIndex(2)
TIME 3221ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3221ms: AppendEntries from ServerId(4) to ServerId(0) for term TermIndex(2)
TIME 3221ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3223ms: AppendEntries from ServerId(4) to ServerId(3) for term TermIndex(2)
TIME 3223ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3224ms: AppendEntries from ServerId(4) to ServerId(1) for term TermIndex(2)
TIME 3224ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3252ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3252ms: AppendEntriesReply from ServerId(1) to ServerId(4) for term TermIndex(2)
TIME 3252ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3256ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3257ms: AppendEntriesReply from ServerId(0) to ServerId(4) for term TermIndex(2)
TIME 3257ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3258ms: AppendEntriesReply from ServerId(2) to ServerId(4) for term TermIndex(2)
TIME 3258ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3265ms: AppendEntriesReply from ServerId(3) to ServerId(4) for term TermIndex(2)
TIME 3265ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3321ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3321ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3321ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3324ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3324ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3325ms: AppendEntries from ServerId(4) to ServerId(0) for term TermIndex(2)
TIME 3325ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3328ms: AppendEntries from ServerId(4) to ServerId(3) for term TermIndex(2)
TIME 3328ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3329ms: AppendEntries from ServerId(4) to ServerId(2) for term TermIndex(2)
TIME 3329ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3370ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3370ms: AppendEntriesReply from ServerId(2) to ServerId(4) for term TermIndex(2)
TIME 3370ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3373ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3373ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3402ms: AppendEntriesReply from ServerId(0) to ServerId(4) for term TermIndex(2)
TIME 3402ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3414ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3414ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3424ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3424ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3424ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3424ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3427ms: RequestVote from ServerId(1) to ServerId(3) for term TermIndex(3)
TIME 3427ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3427ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3428ms: AppendEntries from ServerId(4) to ServerId(1) for term TermIndex(2)
TIME 3428ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3430ms: RequestVote from ServerId(1) to ServerId(2) for term TermIndex(3)
TIME 3430ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3431ms: RequestVoteReply from ServerId(3) to ServerId(1) for term TermIndex(3)
TIME 3431ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3435ms: AppendEntries from ServerId(4) to ServerId(2) for term TermIndex(2)
TIME 3435ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3438ms: RequestVote from ServerId(1) to ServerId(0) for term TermIndex(3)
TIME 3438ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3443ms: AppendEntries from ServerId(4) to ServerId(0) for term TermIndex(2)
TIME 3443ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3475ms: RequestVote from ServerId(1) to ServerId(4) for term TermIndex(3)
TIME 3475ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3503ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(2), voted_for: Some((TermIndex(2), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 3503ms: RequestVoteReply from ServerId(0) to ServerId(1) for term TermIndex(3)
TIME 3503ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
TIME 3504ms: AppendEntriesReply from ServerId(0) to ServerId(4) for term TermIndex(3)
TIME 3504ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
TIME 3510ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
TIME 3510ms: AppendEntries from ServerId(4) to ServerId(3) for term TermIndex(2)
TIME 3510ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
TIME 3510ms: AppendEntries from ServerId(1) to ServerId(3) for term TermIndex(3)
TIME 3510ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
TIME 3513ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
TIME 3516ms: AppendEntriesReply from ServerId(3) to ServerId(4) for term TermIndex(3)
TIME 3516ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
TIME 3517ms: AppendEntries from ServerId(1) to ServerId(2) for term TermIndex(3)
TIME 3517ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
TIME 3517ms: AppendEntries from ServerId(1) to ServerId(4) for term TermIndex(3)
TIME 3517ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
TIME 3517ms: AppendEntriesReply from ServerId(1) to ServerId(4) for term TermIndex(3)
TIME 3517ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
TIME 3520ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
TIME 3523ms: AppendEntriesReply from ServerId(2) to ServerId(1) for term TermIndex(3)
TIME 3523ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
TIME 3527ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
TIME 3528ms: RequestVoteReply from ServerId(4) to ServerId(1) for term TermIndex(3)
TIME 3528ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
TIME 3529ms: AppendEntriesReply from ServerId(2) to ServerId(4) for term TermIndex(3)
TIME 3529ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
TIME 3531ms: AppendEntriesReply from ServerId(4) to ServerId(1) for term TermIndex(3)
TIME 3531ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
TIME 3535ms: AppendEntriesReply from ServerId(3) to ServerId(4) for term TermIndex(2)
TIME 3535ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
TIME 3539ms: AppendEntriesReply from ServerId(3) to ServerId(1) for term TermIndex(3)
TIME 3539ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
TIME 3539ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
TIME 3567ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
TIME 3604ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
TIME 3604ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
TIME 3610ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
TIME 3610ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
TIME 3610ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
TIME 3610ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
TIME 3610ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
TIME 3610ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
TIME 3654ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
TIME 3654ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
TIME 3654ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
TIME 3654ms: AppendEntries from ServerId(1) to ServerId(2) for term TermIndex(3)
TIME 3654ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
TIME 3660ms: AppendEntries from ServerId(1) to ServerId(0) for term TermIndex(3)
TIME 3660ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Candidate, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
TIME 3660ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Candidate, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
TIME 3660ms: RequestVote from ServerId(0) to ServerId(3) for term TermIndex(4)
TIME 3660ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Candidate, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
TIME 3660ms: AppendEntriesReply from ServerId(0) to ServerId(1) for term TermIndex(4)
TIME 3660ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Candidate, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
TIME 3660ms: AppendEntriesReply from ServerId(2) to ServerId(1) for term TermIndex(3)
TIME 3660ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Candidate, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
TIME 3660ms: RequestVote from ServerId(0) to ServerId(1) for term TermIndex(4)
TIME 3660ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Candidate, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
TIME 3669ms: RequestVote from ServerId(0) to ServerId(4) for term TermIndex(4)
TIME 3669ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Candidate, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
TIME 3671ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Candidate, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
TIME 3671ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Candidate, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
TIME 3671ms: RequestVoteReply from ServerId(4) to ServerId(0) for term TermIndex(4)
TIME 3671ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Candidate, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
TIME 3672ms: RequestVoteReply from ServerId(3) to ServerId(0) for term TermIndex(4)
TIME 3672ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Candidate, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
TIME 3672ms: AppendEntries from ServerId(1) to ServerId(4) for term TermIndex(3)
TIME 3672ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Candidate, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
TIME 3673ms: RequestVoteReply from ServerId(1) to ServerId(0) for term TermIndex(4)
TIME 3673ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Candidate, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
TIME 3701ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Candidate, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
TIME 3701ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
TIME 3701ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
TIME 3701ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
TIME 3702ms: AppendEntries from ServerId(0) to ServerId(1) for term TermIndex(4)
TIME 3702ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
TIME 3703ms: AppendEntries from ServerId(0) to ServerId(3) for term TermIndex(4)
TIME 3703ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
TIME 3703ms: AppendEntriesReply from ServerId(4) to ServerId(1) for term TermIndex(4)
TIME 3703ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
TIME 3703ms: AppendEntries from ServerId(0) to ServerId(2) for term TermIndex(4)
TIME 3703ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
TIME 3712ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(3), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
TIME 3712ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
TIME 3712ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
TIME 3712ms: AppendEntriesReply from ServerId(1) to ServerId(0) for term TermIndex(4)
TIME 3712ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
TIME 3716ms: AppendEntriesReply from ServerId(2) to ServerId(0) for term TermIndex(4)
TIME 3716ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
TIME 3726ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
TIME 3727ms: AppendEntries from ServerId(1) to ServerId(0) for term TermIndex(3)
TIME 3727ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
TIME 3730ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
TIME 3730ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
TIME 3730ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
TIME 3731ms: AppendEntriesReply from ServerId(0) to ServerId(1) for term TermIndex(4)
TIME 3731ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
TIME 3739ms: RequestVote from ServerId(0) to ServerId(2) for term TermIndex(4)
TIME 3739ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
TIME 3740ms: AppendEntries from ServerId(0) to ServerId(4) for term TermIndex(4)
TIME 3740ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
TIME 3754ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(3), ServerId(1))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
TIME 3761ms: AppendEntriesReply from ServerId(4) to ServerId(0) for term TermIndex(4)
TIME 3761ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 3775ms: AppendEntries from ServerId(1) to ServerId(3) for term TermIndex(3)
TIME 3775ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 3792ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 3800ms: AppendEntriesReply from ServerId(3) to ServerId(1) for term TermIndex(4)
TIME 3800ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 3801ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 3801ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 3801ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 3801ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 3801ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 3801ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 3801ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 3840ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 3840ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 3840ms: AppendEntries from ServerId(0) to ServerId(3) for term TermIndex(4)
TIME 3840ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 3842ms: AppendEntries from ServerId(0) to ServerId(2) for term TermIndex(4)
TIME 3842ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 3861ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 3861ms: AppendEntries from ServerId(0) to ServerId(4) for term TermIndex(4)
TIME 3861ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 3861ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 3867ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 3867ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 3867ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 3874ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 3874ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 3880ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 3880ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 3880ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 3880ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 3880ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 3889ms: AppendEntriesReply from ServerId(3) to ServerId(0) for term TermIndex(4)
TIME 3889ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 3889ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 3890ms: RequestVote from ServerId(1) to ServerId(2) for term TermIndex(5)
TIME 3890ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 3891ms: RequestVote from ServerId(1) to ServerId(0) for term TermIndex(5)
TIME 3891ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 3895ms: AppendEntriesReply from ServerId(3) to ServerId(0) for term TermIndex(4)
TIME 3895ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 3895ms: AppendEntries from ServerId(4) to ServerId(1) for term TermIndex(2)
TIME 3895ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 3902ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Leader, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 3902ms: RequestVoteReply from ServerId(2) to ServerId(1) for term TermIndex(5)
TIME 3902ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 3908ms: AppendEntriesReply from ServerId(2) to ServerId(0) for term TermIndex(4)
TIME 3908ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 3909ms: RequestVoteReply from ServerId(0) to ServerId(1) for term TermIndex(5)
TIME 3909ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 3919ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 3919ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 3919ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 3920ms: AppendEntries from ServerId(1) to ServerId(4) for term TermIndex(5)
TIME 3920ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 3920ms: AppendEntries from ServerId(1) to ServerId(3) for term TermIndex(5)
TIME 3920ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 3921ms: AppendEntries from ServerId(1) to ServerId(0) for term TermIndex(5)
TIME 3921ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 3923ms: AppendEntries from ServerId(1) to ServerId(2) for term TermIndex(5)
TIME 3923ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 3940ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(4), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(0)) }
TIME 3940ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(1)) }
TIME 3942ms: RequestVote from ServerId(1) to ServerId(4) for term TermIndex(5)
TIME 3942ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(1)) }
TIME 3946ms: AppendEntriesReply from ServerId(2) to ServerId(1) for term TermIndex(5)
TIME 3946ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(1)) }
TIME 3947ms: AppendEntriesReply from ServerId(0) to ServerId(1) for term TermIndex(5)
TIME 3947ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(1)) }
TIME 3950ms: AppendEntriesReply from ServerId(4) to ServerId(1) for term TermIndex(5)
TIME 3950ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(1)) }
TIME 3953ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(1)) }
TIME 3961ms: RequestVoteReply from ServerId(4) to ServerId(1) for term TermIndex(5)
TIME 3961ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: None }
TIME 3963ms: AppendEntriesReply from ServerId(1) to ServerId(4) for term TermIndex(5)
TIME 3963ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: None }
TIME 3966ms: AppendEntriesReply from ServerId(3) to ServerId(1) for term TermIndex(5)
TIME 3966ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: None }
TIME 3967ms: AppendEntriesReply from ServerId(4) to ServerId(0) for term TermIndex(4)
TIME 3967ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: None }
TIME 3999ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: None }
TIME 3999ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: None }
TIME 4019ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: None }
TIME 4019ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: None }
TIME 4019ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: None }
TIME 4019ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: None }
TIME 4019ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: None }
TIME 4019ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: None }
TIME 4019ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: None }
TIME 4060ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: None }
TIME 4060ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: None }
TIME 4060ms: AppendEntries from ServerId(1) to ServerId(3) for term TermIndex(5)
TIME 4060ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: None }
TIME 4063ms: AppendEntries from ServerId(1) to ServerId(0) for term TermIndex(5)
TIME 4063ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: None }
TIME 4064ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: None }
TIME 4065ms: AppendEntriesReply from ServerId(0) to ServerId(1) for term TermIndex(5)
TIME 4065ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: None }
TIME 4070ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: None }
TIME 4070ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: None }
TIME 4070ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: None }
TIME 4071ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: None }
TIME 4071ms: AppendEntries from ServerId(1) to ServerId(4) for term TermIndex(5)
TIME 4071ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: None }
TIME 4078ms: AppendEntries from ServerId(1) to ServerId(2) for term TermIndex(5)
TIME 4078ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: None }
TIME 4083ms: RequestVoteReply from ServerId(2) to ServerId(1) for term TermIndex(3)
TIME 4083ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: None }
TIME 4090ms: AppendEntriesReply from ServerId(3) to ServerId(1) for term TermIndex(5)
TIME 4090ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: None }
TIME 4160ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: None }
TIME 4160ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Candidate, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
TIME 4160ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Candidate, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
TIME 4160ms: RequestVote from ServerId(4) to ServerId(3) for term TermIndex(6)
TIME 4160ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Candidate, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
TIME 4160ms: RequestVote from ServerId(4) to ServerId(1) for term TermIndex(6)
TIME 4160ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Candidate, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
TIME 4161ms: RequestVote from ServerId(4) to ServerId(2) for term TermIndex(6)
TIME 4161ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Candidate, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
TIME 4161ms: RequestVote from ServerId(4) to ServerId(0) for term TermIndex(6)
TIME 4161ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Candidate, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
TIME 4164ms: AppendEntriesReply from ServerId(2) to ServerId(1) for term TermIndex(5)
TIME 4164ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Candidate, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
TIME 4166ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(5), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(5), voted_for: Some((TermIndex(4), ServerId(0))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Candidate, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
TIME 4166ms: RequestVoteReply from ServerId(3) to ServerId(4) for term TermIndex(6)
TIME 4166ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Candidate, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
TIME 4169ms: RequestVoteReply from ServerId(0) to ServerId(4) for term TermIndex(6)
TIME 4169ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Candidate, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
TIME 4170ms: RequestVoteReply from ServerId(1) to ServerId(4) for term TermIndex(6)
TIME 4170ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Candidate, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
TIME 4171ms: RequestVoteReply from ServerId(2) to ServerId(4) for term TermIndex(6)
TIME 4171ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Candidate, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
TIME 4173ms: AppendEntriesReply from ServerId(4) to ServerId(1) for term TermIndex(6)
TIME 4173ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Candidate, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
TIME 4176ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Candidate, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
TIME 4177ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4177ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4178ms: AppendEntries from ServerId(4) to ServerId(3) for term TermIndex(6)
TIME 4178ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4185ms: AppendEntries from ServerId(4) to ServerId(0) for term TermIndex(6)
TIME 4185ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4190ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4190ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4190ms: AppendEntriesReply from ServerId(3) to ServerId(4) for term TermIndex(6)
TIME 4190ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4190ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4191ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4203ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4203ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4203ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4221ms: AppendEntries from ServerId(4) to ServerId(1) for term TermIndex(6)
TIME 4221ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4231ms: AppendEntries from ServerId(4) to ServerId(2) for term TermIndex(6)
TIME 4231ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4276ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4276ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4276ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4276ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4278ms: AppendEntriesReply from ServerId(2) to ServerId(4) for term TermIndex(6)
TIME 4278ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4278ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4278ms: AppendEntries from ServerId(4) to ServerId(3) for term TermIndex(6)
TIME 4278ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4279ms: AppendEntriesReply from ServerId(1) to ServerId(4) for term TermIndex(6)
TIME 4279ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4291ms: AppendEntries from ServerId(4) to ServerId(1) for term TermIndex(6)
TIME 4291ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4304ms: AppendEntries from ServerId(4) to ServerId(2) for term TermIndex(6)
TIME 4304ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4324ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4325ms: AppendEntries from ServerId(4) to ServerId(0) for term TermIndex(6)
TIME 4325ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4325ms: AppendEntriesReply from ServerId(2) to ServerId(4) for term TermIndex(6)
TIME 4325ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4327ms: AppendEntriesReply from ServerId(3) to ServerId(4) for term TermIndex(6)
TIME 4327ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4327ms: AppendEntriesReply from ServerId(1) to ServerId(4) for term TermIndex(6)
TIME 4327ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4337ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4356ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4376ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4376ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4376ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4378ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4378ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4378ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4378ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4378ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4378ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4378ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4381ms: AppendEntriesReply from ServerId(0) to ServerId(4) for term TermIndex(6)
TIME 4381ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4381ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4383ms: AppendEntries from ServerId(4) to ServerId(1) for term TermIndex(6)
TIME 4383ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4386ms: AppendEntries from ServerId(4) to ServerId(3) for term TermIndex(6)
TIME 4386ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4389ms: AppendEntries from ServerId(4) to ServerId(2) for term TermIndex(6)
TIME 4389ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4402ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4403ms: AppendEntriesReply from ServerId(1) to ServerId(4) for term TermIndex(6)
TIME 4403ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4403ms: AppendEntriesReply from ServerId(2) to ServerId(4) for term TermIndex(6)
TIME 4403ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4406ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4419ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4419ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4430ms: AppendEntriesReply from ServerId(3) to ServerId(4) for term TermIndex(6)
TIME 4430ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4440ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4453ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4481ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4481ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4481ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4481ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4481ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4481ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4481ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4492ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4492ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4492ms: AppendEntries from ServerId(4) to ServerId(1) for term TermIndex(6)
TIME 4492ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4492ms: AppendEntries from ServerId(4) to ServerId(3) for term TermIndex(6)
TIME 4492ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4493ms: AppendEntries from ServerId(4) to ServerId(0) for term TermIndex(6)
TIME 4493ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4496ms: AppendEntries from ServerId(4) to ServerId(2) for term TermIndex(6)
TIME 4496ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4497ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4497ms: AppendEntriesReply from ServerId(0) to ServerId(4) for term TermIndex(6)
TIME 4497ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4497ms: AppendEntriesReply from ServerId(2) to ServerId(4) for term TermIndex(6)
TIME 4497ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4498ms: AppendEntriesReply from ServerId(1) to ServerId(4) for term TermIndex(6)
TIME 4498ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4530ms: AppendEntriesReply from ServerId(3) to ServerId(4) for term TermIndex(6)
TIME 4530ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4536ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4552ms: AppendEntries from ServerId(4) to ServerId(0) for term TermIndex(6)
TIME 4552ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4571ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4573ms: AppendEntriesReply from ServerId(0) to ServerId(4) for term TermIndex(6)
TIME 4573ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4575ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4591ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4592ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4592ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4592ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4592ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4592ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4592ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4592ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4627ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4627ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4627ms: AppendEntries from ServerId(4) to ServerId(1) for term TermIndex(6)
TIME 4627ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4628ms: AppendEntries from ServerId(4) to ServerId(3) for term TermIndex(6)
TIME 4628ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4629ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4629ms: AppendEntriesReply from ServerId(3) to ServerId(4) for term TermIndex(6)
TIME 4629ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4630ms: AppendEntries from ServerId(4) to ServerId(0) for term TermIndex(6)
TIME 4630ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4631ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4634ms: AppendEntriesReply from ServerId(0) to ServerId(4) for term TermIndex(6)
TIME 4634ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4638ms: AppendEntries from ServerId(4) to ServerId(2) for term TermIndex(6)
TIME 4638ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4647ms: AppendEntriesReply from ServerId(1) to ServerId(4) for term TermIndex(6)
TIME 4647ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4667ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4688ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4690ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4727ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4727ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4727ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4727ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4727ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4734ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4734ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4736ms: AppendEntries from ServerId(4) to ServerId(2) for term TermIndex(6)
TIME 4736ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4740ms: AppendEntries from ServerId(4) to ServerId(1) for term TermIndex(6)
TIME 4740ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4740ms: AppendEntries from ServerId(4) to ServerId(0) for term TermIndex(6)
TIME 4740ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4788ms: AppendEntries from ServerId(4) to ServerId(3) for term TermIndex(6)
TIME 4788ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4812ms: AppendEntriesReply from ServerId(2) to ServerId(4) for term TermIndex(6)
TIME 4812ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4815ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4815ms: AppendEntriesReply from ServerId(1) to ServerId(4) for term TermIndex(6)
TIME 4815ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4818ms: AppendEntriesReply from ServerId(3) to ServerId(4) for term TermIndex(6)
TIME 4818ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4820ms: AppendEntriesReply from ServerId(0) to ServerId(4) for term TermIndex(6)
TIME 4820ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4834ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4834ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4836ms: AppendEntriesReply from ServerId(2) to ServerId(4) for term TermIndex(6)
TIME 4836ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4836ms: AppendEntries from ServerId(4) to ServerId(0) for term TermIndex(6)
TIME 4836ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4837ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4838ms: AppendEntries from ServerId(4) to ServerId(3) for term TermIndex(6)
TIME 4838ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4842ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4850ms: AppendEntriesReply from ServerId(3) to ServerId(4) for term TermIndex(6)
TIME 4850ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4855ms: AppendEntries from ServerId(4) to ServerId(1) for term TermIndex(6)
TIME 4855ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4864ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4869ms: AppendEntriesReply from ServerId(1) to ServerId(4) for term TermIndex(6)
TIME 4869ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4877ms: AppendEntries from ServerId(4) to ServerId(2) for term TermIndex(6)
TIME 4877ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4890ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4891ms: AppendEntriesReply from ServerId(2) to ServerId(4) for term TermIndex(6)
TIME 4891ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4936ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4936ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4936ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4936ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4936ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4936ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4936ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4989ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4989ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 4990ms: AppendEntries from ServerId(4) to ServerId(0) for term TermIndex(6)
TIME 4990ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 5009ms: AppendEntries from ServerId(4) to ServerId(2) for term TermIndex(6)
TIME 5009ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 5022ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 5022ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 5025ms: AppendEntriesReply from ServerId(0) to ServerId(4) for term TermIndex(6)
TIME 5025ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 5025ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 5025ms: RequestVote from ServerId(1) to ServerId(3) for term TermIndex(7)
TIME 5025ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 5027ms: AppendEntriesReply from ServerId(2) to ServerId(4) for term TermIndex(6)
TIME 5027ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 5028ms: RequestVote from ServerId(1) to ServerId(2) for term TermIndex(7)
TIME 5028ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 5029ms: RequestVote from ServerId(1) to ServerId(4) for term TermIndex(7)
TIME 5029ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 5068ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Leader, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
TIME 5069ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
TIME 5071ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
TIME 5076ms: AppendEntries from ServerId(4) to ServerId(1) for term TermIndex(6)
TIME 5076ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
TIME 5079ms: RequestVoteReply from ServerId(2) to ServerId(1) for term TermIndex(7)
TIME 5079ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
TIME 5089ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
TIME 5089ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
TIME 5089ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
TIME 5097ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
TIME 5104ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
TIME 5114ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
TIME 5115ms: RequestVoteReply from ServerId(4) to ServerId(1) for term TermIndex(7)
TIME 5115ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
TIME 5151ms: AppendEntries from ServerId(0) to ServerId(1) for term TermIndex(4)
TIME 5151ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
TIME 5174ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Candidate, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
TIME 5174ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
TIME 5174ms: AppendEntries from ServerId(1) to ServerId(3) for term TermIndex(7)
TIME 5174ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(6), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
TIME 5174ms: AppendEntriesReply from ServerId(1) to ServerId(0) for term TermIndex(7)
TIME 5174ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(7), voted_for: Some((TermIndex(6), ServerId(4))), leader_for_term: Some(ServerId(4)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Follower, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(7), voted_for: Some((TermIndex(7), ServerId(1))), leader_for_term: None }
```

This is caused by the last RPC request:

TIME 5174ms: AppendEntriesReply from ServerId(1) to ServerId(0) for term TermIndex(7)

This updated ServerId(0)'s term to 7 but it still has the leader_id from term 6 causing an inconsistency.
It should make leader_id = None when updating term from higher term RPC.

Would this bug keep it from working properly?

I think it would mainly cause a problem if this node was forwarding a client request to the leader.
The request would error out because ServerId(4) is no longer leader and can't process the request.
Depending on implementation I guess you could have it make an extra hop to the new leader but that would be inefficient.
But as long as the AppendEntries request from the new leader makes it to this node it would eventually update to new leader.

Also ServerId(0) used to be a leader. This seems to be a late arriving RPC request from 0 to 1.
We know that one is late and not the reply bc the reply has term 7.

This was interesting. This was happening intermittently not every time.
To capture this I had to run the tests in a loop until I got it to crash again.
