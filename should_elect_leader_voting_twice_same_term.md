```
TIME 2060ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(5), voted_for: Some(ServerId(1)), leader_for_term: Some(ServerId(1)) }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(5), voted_for: Some(ServerId(1)), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(5), voted_for: Some(ServerId(1)), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Candidate, current_term: TermIndex(5), voted_for: Some(ServerId(3)), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(5), voted_for: Some(ServerId(1)), leader_for_term: Some(ServerId(1)) }
TIME 2060ms: SEND RequestVote from ServerId(3) to ServerId(4) for term TermIndex(5) with latency 0ms tbd at 2060 (req id: 315b8d38-5039-4e3c-8161-a056d40a17be)
TIME 2060ms: SEND RequestVote from ServerId(3) to ServerId(1) for term TermIndex(5) with latency 2ms tbd at 2062 (req id: 36823dfa-d99e-4580-bd92-4b8c4c1fc01a)
TIME 2060ms: SEND RequestVote from ServerId(3) to ServerId(0) for term TermIndex(5) with latency 2ms tbd at 2062 (req id: 29f25593-24c1-4dd8-9387-8a78fabba6eb)
TIME 2060ms: SEND RequestVote from ServerId(3) to ServerId(2) for term TermIndex(5) with latency 0ms tbd at 2060 (req id: 67920459-2331-4eed-8ba8-b602e1c9f469)
TIME 2060ms: RECV RequestVote from ServerId(3) to ServerId(4) for term TermIndex(5) (req id: 315b8d38-5039-4e3c-8161-a056d40a17be)
TIME 2060ms: RECV RequestVote from ServerId(3) to ServerId(2) for term TermIndex(5) (req id: 67920459-2331-4eed-8ba8-b602e1c9f469)
TIME 2060ms: RECV RequestVote from ServerId(3) to ServerId(1) for term TermIndex(5) (req id: 36823dfa-d99e-4580-bd92-4b8c4c1fc01a)
TIME 2062ms: RECV RequestVote from ServerId(3) to ServerId(0) for term TermIndex(5) (req id: 29f25593-24c1-4dd8-9387-8a78fabba6eb)
TIME 2070ms: RECV RequestVote from ServerId(1) to ServerId(3) for term TermIndex(5) (req id: 65052cfa-1878-4cea-994a-aee87d99c54e)
TIME 2081ms: SEND RequestVoteReply from ServerId(1) to ServerId(3) for term TermIndex(5) with latency 7ms tbd at 2088 (req id: 36823dfa-d99e-4580-bd92-4b8c4c1fc01a)
TIME 2081ms: SEND RequestVoteReply from ServerId(4) to ServerId(3) for term TermIndex(5) with latency 5ms tbd at 2086 (req id: 315b8d38-5039-4e3c-8161-a056d40a17be)
TIME 2081ms: RECV AppendEntriesReply from ServerId(4) to ServerId(1) for term TermIndex(5) (req id: 82bac1c2-1be9-4d3b-b163-f1b14eea109d)
TIME 2082ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(5), voted_for: Some(ServerId(3)), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(5), voted_for: Some(ServerId(1)), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(5), voted_for: Some(ServerId(1)), leader_for_term: Some(ServerId(1)) }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Candidate, current_term: TermIndex(5), voted_for: Some(ServerId(3)), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(5), voted_for: Some(ServerId(3)), leader_for_term: None }
TIME 2082ms: SEND RequestVoteReply from ServerId(0) to ServerId(3) for term TermIndex(5) with latency 2ms tbd at 2084 (req id: 29f25593-24c1-4dd8-9387-8a78fabba6eb)
TIME 2082ms: SEND RequestVoteReply from ServerId(2) to ServerId(3) for term TermIndex(5) with latency 0ms tbd at 2082 (req id: 67920459-2331-4eed-8ba8-b602e1c9f469)
TIME 2082ms: RECV RequestVoteReply from ServerId(2) to ServerId(3) for term TermIndex(5) (req id: 67920459-2331-4eed-8ba8-b602e1c9f469)
TIME 2082ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(5), voted_for: Some(ServerId(3)), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(5), voted_for: Some(ServerId(1)), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(5), voted_for: Some(ServerId(3)), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Candidate, current_term: TermIndex(5), voted_for: Some(ServerId(3)), leader_for_term: None }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(5), voted_for: Some(ServerId(3)), leader_for_term: None }
TIME 2082ms: RECV RequestVoteReply from ServerId(0) to ServerId(3) for term TermIndex(5) (req id: 29f25593-24c1-4dd8-9387-8a78fabba6eb)
TIME 2084ms: RECV RequestVoteReply from ServerId(4) to ServerId(3) for term TermIndex(5) (req id: 315b8d38-5039-4e3c-8161-a056d40a17be)
TIME 2086ms: RECV RequestVoteReply from ServerId(1) to ServerId(3) for term TermIndex(5) (req id: 36823dfa-d99e-4580-bd92-4b8c4c1fc01a)
TIME 2088ms: RECV AppendEntries from ServerId(1) to ServerId(3) for term TermIndex(5) (req id: 5d3038c0-2d43-4ad3-8698-3fe78069c411)
TIME 2098ms: RECV RequestVoteReply from ServerId(1) to ServerId(0) for term TermIndex(4) (req id: 2a7cba56-31e2-42cb-a426-560a6c0d0edf)
TIME 2100ms: RECV RequestVoteReply from ServerId(2) to ServerId(1) for term TermIndex(5) (req id: a238b004-e2be-4a33-976f-4010ea168f73)
TIME 2166ms: SEND AppendEntries from ServerId(1) to ServerId(4) for term TermIndex(5) with latency 10ms tbd at 2176 (req id: 1c74c751-86ef-426d-9dd5-77ca4f892981)
TIME 2166ms: SEND AppendEntries from ServerId(1) to ServerId(2) for term TermIndex(5) with latency 11ms tbd at 2177 (req id: e97ad51e-c050-4c05-9a68-cc2512fa2e1a)
TIME 2166ms: SEND AppendEntries from ServerId(1) to ServerId(3) for term TermIndex(5) with latency 0ms tbd at 2166 (req id: 12b25b9c-59ed-4cbf-b01a-57f4f7fab1b6)
TIME 2166ms: SEND AppendEntries from ServerId(1) to ServerId(0) for term TermIndex(5) with latency 0ms tbd at 2166 (req id: d7e8ff85-bcb1-492e-9cca-e21a35039dee)
TIME 2166ms: RECV AppendEntries from ServerId(1) to ServerId(3) for term TermIndex(5) (req id: 12b25b9c-59ed-4cbf-b01a-57f4f7fab1b6)
TIME 2166ms: ServerStates...
    Server ServerId(0) is in state RaftStateEvent { server_id: ServerId(0), current_state: Follower, current_term: TermIndex(5), voted_for: Some(ServerId(3)), leader_for_term: None }
    Server ServerId(1) is in state RaftStateEvent { server_id: ServerId(1), current_state: Leader, current_term: TermIndex(5), voted_for: Some(ServerId(1)), leader_for_term: Some(ServerId(1)) }
    Server ServerId(2) is in state RaftStateEvent { server_id: ServerId(2), current_state: Follower, current_term: TermIndex(5), voted_for: Some(ServerId(3)), leader_for_term: None }
    Server ServerId(3) is in state RaftStateEvent { server_id: ServerId(3), current_state: Leader, current_term: TermIndex(5), voted_for: Some(ServerId(3)), leader_for_term: Some(ServerId(3)) }
    Server ServerId(4) is in state RaftStateEvent { server_id: ServerId(4), current_state: Follower, current_term: TermIndex(5), voted_for: Some(ServerId(3)), leader_for_term: None }
```

Server voting for two different candidates in one term, obvious bug, but why?

```
-        let have_we_voted_for_this_term = storage.voted_for_in_current_term().is_some();
-        let voted_for_same_candidate_already = storage
-            .voted_for_in_current_term()
-            .map(|_| true)
+        let we_voted_this_term_already = storage.vote_for_current_term().is_some();
+        let we_voted_for_same_candidate_this_term_already = storage
+            .vote_for_current_term()
+            .map(|current_vote| current_vote == vote_req.from)
             .unwrap_or(false);

         let vote_granted = candidate_has_same_or_newer_term
-            && (!have_we_voted_for_this_term || voted_for_same_candidate_already);
+            && (!we_voted_this_term_already || we_voted_for_same_candidate_this_term_already);
```

Relevant part is this...

```
-        let voted_for_same_candidate_already = storage
-            .voted_for_in_current_term()
-            .map(|_| true)
+        let we_voted_for_same_candidate_this_term_already = storage
+            .vote_for_current_term()
+            .map(|current_vote| current_vote == vote_req.from)
             .unwrap_or(false);
```

_facepalm_ we are not comparing our current vote to the server that made the vote request. We are always granting our vote once we vote once, we respond to any other requests in the same term with a yes.
