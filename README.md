# Raft Implementation Written In Rust

Currently only leader election is implemented. Get/set commands are ignored.

- Distributed single value store with Raft consensus
- Simulator that runs Raft nodes and provides a simulated network between the nodes where latency and message drop probability can be adjusted
- Program to run a cluster of nodes locally

Run tests:

```
make test
```

Run a cluster of 5 nodes:

```
make run-cluster
```

Use client to send GET/SET request to servers:
GET

```
SERVER=0 make client-get
```

SET

```
SERVER=3 VALUE=12345 client-set
```
