Now we get this:

```
thread '<unnamed>' panicked at 'internal error: entered unreachable code: BUG: If candidate receives an append entries from a higher term, it should have become a follower already', /home/jon/Projects/shard_per_core/raft_consensus/src/state_machine.rs:531:25
note: run with `RUST_BACKTRACE=1` environment variable to display a backtrace
[2m2023-04-14T05:58:46.890994Z[0m [32m INFO[0m [2mraft_consensus::state_machine[0m[2m:[0m ServerId(0): Becoming follower because we've observed a higher term TermIndex(17) than ours TermIndex(16)
[2m2023-04-14T05:58:46.891071Z[0m [32m INFO[0m [2mraft_consensus::state_machine[0m[2m:[0m ServerId(0): Voting for candidate ServerId(3) in term TermIndex(17)
thread 'should_elect_leader_without_network_partition' panicked at 'SIM: Could not send network message to server: SendError { .. }', raft_consensus/tests/simulator/sim_network.rs:357:14
```

That last change we made to not ignore append entries from same term if we are candidate? We also need to switch to follower mode if its the same term, oops!

```
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
```
