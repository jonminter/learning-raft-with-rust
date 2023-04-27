```
====================================
RNG SEED FOR TESTS: 6043988003234447901
====================================
fault injected at raft_consensus raft_consensus/src/default_storage.rs 115
Restarting server 1...
fault injected at raft_consensus raft_consensus/src/default_storage.rs 115
Restarting server 1...
fault injected at raft_consensus raft_consensus/src/default_storage.rs 115
Restarting server 1...
fault injected at raft_consensus raft_consensus/src/default_storage.rs 149
Restarting server 0...
fault injected at raft_consensus raft_consensus/src/default_storage.rs 115
Restarting server 4...
thread 'test_with_quickcheck' panicked at 'ServerId(4): Term index should always increase, old: TermIndex(30), new: TermIndex(29)', raft_consensus/tests/simulator/invariant_checker.rs:105:13
stack backtrace:
   0: rust_begin_unwind
             at /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library/std/src/panicking.rs:575:5
   1: core::panicking::panic_fmt
             at /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library/core/src/panicking.rs:64:14
   2: raft_tests::simulator::invariant_checker::InvariantChecker::check_state_change_invariants
             at ./tests/simulator/invariant_checker.rs:105:13
   3: raft_tests::simulator::invariant_checker::InvariantChecker::check_invariants
             at ./tests/simulator/invariant_checker.rs:75:13
   4: raft_tests::simulator::ClusterSim::run_step
             at ./tests/simulator/mod.rs:263:13
   5: raft_tests::simulator::ClusterSim::run_until_time
             at ./tests/simulator/mod.rs:294:13
   6: raft_tests::run_simulation_with_sequence_of_events
             at ./tests/raft_tests.rs:382:5
   7: raft_tests::test_with_quickcheck::test_impl::prop
             at ./tests/raft_tests.rs:390:9
   8: <fn(A) .> T as quickcheck::tester::Testable>::result::{{closure}}
             at /home/jon/.cargo/registry/src/github.com-1ecc6299db9ec823/quickcheck-1.0.3/src/tester.rs:371:35
   9: <core::panic::unwind_safe::AssertUnwindSafe<F> as core::ops::function::FnOnce<()>>::call_once
             at /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library/core/src/panic/unwind_safe.rs:271:9
  10: std::panicking::try::do_call
             at /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library/std/src/panicking.rs:483:40
  11: __rust_try
  12: std::panicking::try
             at /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library/std/src/panicking.rs:447:19
  13: std::panic::catch_unwind
             at /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library/std/src/panic.rs:137:14
  14: quickcheck::tester::safe
             at /home/jon/.cargo/registry/src/github.com-1ecc6299db9ec823/quickcheck-1.0.3/src/tester.rs:402:5
  15: <fn(A) .> T as quickcheck::tester::Testable>::result
             at /home/jon/.cargo/registry/src/github.com-1ecc6299db9ec823/quickcheck-1.0.3/src/tester.rs:371:21
  16: quickcheck::tester::QuickCheck::quicktest
             at /home/jon/.cargo/registry/src/github.com-1ecc6299db9ec823/quickcheck-1.0.3/src/tester.rs:121:19
  17: quickcheck::tester::QuickCheck::quickcheck
             at /home/jon/.cargo/registry/src/github.com-1ecc6299db9ec823/quickcheck-1.0.3/src/tester.rs:163:36
  18: raft_tests::test_with_quickcheck::test_impl
             at ./tests/raft_tests.rs:393:5
  19: raft_tests::test_with_quickcheck
             at ./tests/raft_tests.rs:387:1
  20: raft_tests::test_with_quickcheck::{{closure}}
             at ./tests/raft_tests.rs:387:1
  21: core::ops::function::FnOnce::call_once
             at /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library/core/src/ops/function.rs:507:5
  22: core::ops::function::FnOnce::call_once
             at /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library/core/src/ops/function.rs:507:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
thread 'test_with_quickcheck' panicked at '[quickcheck] TEST FAILED (runtime error). Arguments: (SimInstructionSequence { generated_state_changes: [SimulatorEvent { time: SimTime(100ms), action: PartitionNetwork([{ServerId(0), ServerId(1)}, {ServerId(2), ServerId(3), ServerId(4)}]) }, SimulatorEvent { time: SimTime(600ms), action: InjectIOFailureEveryNOps(100) }, SimulatorEvent { time: SimTime(10.6s), action: HealNetworkPartition }, SimulatorEvent { time: SimTime(11.1s), action: RestoreIOFunctioning }, SimulatorEvent { time: SimTime(21.1s), action: InjectIOFailureEveryNOps(100) }, SimulatorEvent { time: SimTime(21.6s), action: PartitionNetwork([{ServerId(0)}, {ServerId(2)}, {ServerId(1)}, {ServerId(3)}, {ServerId(4)}]) }, SimulatorEvent { time: SimTime(21.7s), action: HealNetworkPartition }, SimulatorEvent { time: SimTime(21.8s), action: RestoreIOFunctioning }, SimulatorEvent { time: SimTime(22.8s), action: PartitionNetwork([{ServerId(1), ServerId(4)}, {ServerId(0)}, {ServerId(3)}, {ServerId(2)}]) }, SimulatorEvent { time: SimTime(27.8s), action: HealNetworkPartition }, SimulatorEvent { time: SimTime(27.9s), action: InjectIOFailureEveryNOps(100) }, SimulatorEvent { time: SimTime(28s), action: RestoreIOFunctioning }, SimulatorEvent { time: SimTime(28.1s), action: PartitionNetwork([{ServerId(4)}, {ServerId(3)}, {ServerId(0)}, {ServerId(1)}, {ServerId(2)}]) }, SimulatorEvent { time: SimTime(38.1s), action: InjectIOFailureEveryNOps(18446744073709551615) }, SimulatorEvent { time: SimTime(38.6s), action: RestoreIOFunctioning }, SimulatorEvent { time: SimTime(39.6s), action: InjectIOFailureEveryNOps(5) }, SimulatorEvent { time: SimTime(39.7s), action: HealNetworkPartition }, SimulatorEvent { time: SimTime(44.7s), action: PartitionNetwork([{ServerId(1), ServerId(3)}, {ServerId(4)}, {ServerId(0), ServerId(2)}]) }, SimulatorEvent { time: SimTime(49.7s), action: HealNetworkPartition }, SimulatorEvent { time: SimTime(49.8s), action: RestoreIOFunctioning }, SimulatorEvent { time: SimTime(49.9s), action: InjectIOFailureEveryNOps(5) }, SimulatorEvent { time: SimTime(59.9s), action: RestoreIOFunctioning }, SimulatorEvent { time: SimTime(69.9s), action: PartitionNetwork([{ServerId(4), ServerId(2)}, {ServerId(3), ServerId(1)}, {ServerId(0)}]) }, SimulatorEvent { time: SimTime(70s), action: HealNetworkPartition }, SimulatorEvent { time: SimTime(70.1s), action: InjectIOFailureEveryNOps(5) }, SimulatorEvent { time: SimTime(71.1s), action: PartitionNetwork([{ServerId(1)}, {ServerId(3)}, {ServerId(4)}, {ServerId(2)}, {ServerId(0)}]) }, SimulatorEvent { time: SimTime(71.2s), action: RestoreIOFunctioning }, SimulatorEvent { time: SimTime(81.2s), action: HealNetworkPartition }, SimulatorEvent { time: SimTime(81.3s), action: InjectIOFailureEveryNOps(18446744073709551615) }, SimulatorEvent { time: SimTime(81.8s), action: RestoreIOFunctioning }, SimulatorEvent { time: SimTime(81.9s), action: InjectIOFailureEveryNOps(100) }, SimulatorEvent { time: SimTime(82s), action: PartitionNetwork([{ServerId(4), ServerId(1), ServerId(3)}, {ServerId(2), ServerId(0)}]) }, SimulatorEvent { time: SimTime(82.5s), action: HealNetworkPartition }, SimulatorEvent { time: SimTime(83.5s), action: RestoreIOFunctioning }, SimulatorEvent { time: SimTime(83.6s), action: InjectIOFailureEveryNOps(5) }, SimulatorEvent { time: SimTime(84.6s), action: RestoreIOFunctioning }, SimulatorEvent { time: SimTime(84.7s), action: InjectIOFailureEveryNOps(5) }, SimulatorEvent { time: SimTime(89.7s), action: RestoreIOFunctioning }, SimulatorEvent { time: SimTime(90.2s), action: InjectIOFailureEveryNOps(18446744073709551615) }, SimulatorEvent { time: SimTime(95.2s), action: PartitionNetwork([{ServerId(1)}, {ServerId(4)}, {ServerId(2)}, {ServerId(0)}, {ServerId(3)}]) }, SimulatorEvent { time: SimTime(95.3s), action: RestoreIOFunctioning }, SimulatorEvent { time: SimTime(95.4s), action: HealNetworkPartition }, SimulatorEvent { time: SimTime(95.5s), action: InjectIOFailureEveryNOps(18446744073709551615) }, SimulatorEvent { time: SimTime(96.5s), action: RestoreIOFunctioning }, SimulatorEvent { time: SimTime(96.6s), action: InjectIOFailureEveryNOps(100) }, SimulatorEvent { time: SimTime(96.7s), action: PartitionNetwork([{ServerId(1), ServerId(2)}, {ServerId(3), ServerId(4)}, {ServerId(0)}]) }, SimulatorEvent { time: SimTime(101.7s), action: HealNetworkPartition }, SimulatorEvent { time: SimTime(101.8s), action: RestoreIOFunctioning }, SimulatorEvent { time: SimTime(101.9s), action: InjectIOFailureEveryNOps(18446744073709551615) }, SimulatorEvent { time: SimTime(102s), action: RestoreIOFunctioning }, SimulatorEvent { time: SimTime(112s), action: InjectIOFailureEveryNOps(18446744073709551615) }, SimulatorEvent { time: SimTime(112.5s), action: RestoreIOFunctioning }, SimulatorEvent { time: SimTime(112.6s), action: PartitionNetwork([{ServerId(0)}, {ServerId(1)}, {ServerId(3), ServerId(4)}, {ServerId(2)}]) }, SimulatorEvent { time: SimTime(112.7s), action: HealNetworkPartition }, SimulatorEvent { time: SimTime(113.2s), action: InjectIOFailureEveryNOps(100) }, SimulatorEvent { time: SimTime(113.3s), action: RestoreIOFunctioning }, SimulatorEvent { time: SimTime(113.4s), action: PartitionNetwork([{ServerId(3)}, {ServerId(1)}, {ServerId(4)}, {ServerId(2)}, {ServerId(0)}]) }, SimulatorEvent { time: SimTime(114.4s), action: HealNetworkPartition }, SimulatorEvent { time: SimTime(114.5s), action: PartitionNetwork([{ServerId(2)}, {ServerId(1)}, {ServerId(4)}, {ServerId(3)}, {ServerId(0)}]) }, SimulatorEvent { time: SimTime(115s), action: InjectIOFailureEveryNOps(100) }, SimulatorEvent { time: SimTime(115.5s), action: RestoreIOFunctioning }, SimulatorEvent { time: SimTime(116.5s), action: InjectIOFailureEveryNOps(100) }, SimulatorEvent { time: SimTime(117.5s), action: HealNetworkPartition }, SimulatorEvent { time: SimTime(127.5s), action: RestoreIOFunctioning }, SimulatorEvent { time: SimTime(132.5s), action: PartitionNetwork([{ServerId(1)}, {ServerId(3)}, {ServerId(2)}, {ServerId(4)}, {ServerId(0)}]) }, SimulatorEvent { time: SimTime(132.6s), action: HealNetworkPartition }, SimulatorEvent { time: SimTime(132.7s), action: InjectIOFailureEveryNOps(5) }, SimulatorEvent { time: SimTime(133.7s), action: RestoreIOFunctioning }, SimulatorEvent { time: SimTime(134.7s), action: PartitionNetwork([{ServerId(4), ServerId(1)}, {ServerId(2)}, {ServerId(3)}, {ServerId(0)}]) }, SimulatorEvent { time: SimTime(135.7s), action: InjectIOFailureEveryNOps(5) }, SimulatorEvent { time: SimTime(135.8s), action: RestoreIOFunctioning }, SimulatorEvent { time: SimTime(136.3s), action: HealNetworkPartition }, SimulatorEvent { time: SimTime(136.8s), action: PartitionNetwork([{ServerId(2), ServerId(0)}, {ServerId(3), ServerId(1), ServerId(4)}]) }, SimulatorEvent { time: SimTime(136.9s), action: InjectIOFailureEveryNOps(5) }, SimulatorEvent { time: SimTime(146.9s), action: RestoreIOFunctioning }, SimulatorEvent { time: SimTime(156.9s), action: InjectIOFailureEveryNOps(5) }, SimulatorEvent { time: SimTime(161.9s), action: HealNetworkPartition }, SimulatorEvent { time: SimTime(162s), action: RestoreIOFunctioning }, SimulatorEvent { time: SimTime(162.1s), action: PartitionNetwork([{ServerId(0), ServerId(4)}, {ServerId(3)}, {ServerId(2), ServerId(1)}]) }, SimulatorEvent { time: SimTime(163.1s), action: InjectIOFailureEveryNOps(18446744073709551615) }, SimulatorEvent { time: SimTime(163.2s), action: HealNetworkPartition }, SimulatorEvent { time: SimTime(163.3s), action: RestoreIOFunctioning }, SimulatorEvent { time: SimTime(173.3s), action: InjectIOFailureEveryNOps(100) }, SimulatorEvent { time: SimTime(183.3s), action: PartitionNetwork([{ServerId(3), ServerId(4)}, {ServerId(1), ServerId(0)}, {ServerId(2)}]) }, SimulatorEvent { time: SimTime(183.4s), action: HealNetworkPartition }, SimulatorEvent { time: SimTime(184.4s), action: RestoreIOFunctioning }, SimulatorEvent { time: SimTime(185.4s), action: PartitionNetwork([{ServerId(3), ServerId(0)}, {ServerId(1), ServerId(4), ServerId(2)}]) }, SimulatorEvent { time: SimTime(195.4s), action: HealNetworkPartition }, SimulatorEvent { time: SimTime(205.4s), action: InjectIOFailureEveryNOps(5) }, SimulatorEvent { time: SimTime(205.5s), action: RestoreIOFunctioning }, SimulatorEvent { time: SimTime(205.6s), action: PartitionNetwork([{ServerId(2), ServerId(4)}, {ServerId(3), ServerId(0), ServerId(1)}]) }, SimulatorEvent { time: SimTime(206.1s), action: InjectIOFailureEveryNOps(5) }, SimulatorEvent { time: SimTime(207.1s), action: HealNetworkPartition }, SimulatorEvent { time: SimTime(208.1s), action: PartitionNetwork([{ServerId(3), ServerId(0)}, {ServerId(4), ServerId(2)}, {ServerId(1)}]) }, SimulatorEvent { time: SimTime(208.6s), action: RestoreIOFunctioning }, SimulatorEvent { time: SimTime(208.7s), action: HealNetworkPartition }, SimulatorEvent { time: SimTime(208.8s), action: InjectIOFailureEveryNOps(5) }, SimulatorEvent { time: SimTime(209.3s), action: RestoreIOFunctioning }, SimulatorEvent { time: SimTime(209.4s), action: InjectIOFailureEveryNOps(18446744073709551615) }, SimulatorEvent { time: SimTime(210.4s), action: PartitionNetwork([{ServerId(3), ServerId(4)}, {ServerId(1)}, {ServerId(2)}, {ServerId(0)}]) }] })
Error: "ServerId(4): Term index should always increase, old: TermIndex(30), new: TermIndex(29)"', /home/jon/.cargo/registry/src/github.com-1ecc6299db9ec823/quickcheck-1.0.3/src/tester.rs:165:28
stack backtrace:
   0: std::panicking::begin_panic
             at /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library/std/src/panicking.rs:607:12
   1: quickcheck::tester::QuickCheck::quickcheck
             at /home/jon/.cargo/registry/src/github.com-1ecc6299db9ec823/quickcheck-1.0.3/src/tester.rs:165:28
   2: raft_tests::test_with_quickcheck::test_impl
             at ./tests/raft_tests.rs:393:5
   3: raft_tests::test_with_quickcheck
             at ./tests/raft_tests.rs:387:1
   4: raft_tests::test_with_quickcheck::{{closure}}
             at ./tests/raft_tests.rs:387:1
   5: core::ops::function::FnOnce::call_once
             at /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library/core/src/ops/function.rs:507:5
   6: core::ops::function::FnOnce::call_once
             at /rustc/d5a82bbd26e1ad8b7401f6a718a9c57c96905483/library/core/src/ops/function.rs:507:5
note: Some details are omitted, run with `RUST_BACKTRACE=full` for a verbose backtrace.
FAILED

failures:

failures:
    test_with_quickcheck

test result: FAILED. 0 passed; 1 failed; 0 ignored; 0 measured; 10 filtered out; finished in 42.96s

error: test failed, to rerun pass `-p raft_consensus --test raft_tests`
make: *** [Makefile:4: test] Error 101
make RUST_BACKTRACE=1 TEST_TO_RUN=test_with_quickcheck test  94.64s user 16.09s system 241% cpu 45.779 total
```
