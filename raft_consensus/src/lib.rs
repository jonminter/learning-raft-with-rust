/// This is an example of a Raft implementation in rust
#[deny(
    bad_style,
    dead_code,
    improper_ctypes,
    non_shorthand_field_patterns,
    no_mangle_generic_items,
    overflowing_literals,
    path_statements,
    patterns_in_fns_without_body,
    private_in_public,
    unconditional_recursion,
    unused,
    unused_allocation,
    unused_comparisons,
    unused_parens,
    while_true,
    missing_debug_implementations,
    missing_docs,
    trivial_casts,
    trivial_numeric_casts,
    unused_extern_crates,
    unused_import_braces,
    unused_qualifications,
    unused_results
)]
mod common;
mod raft;

pub use common::*;
pub use raft::common::LogCommand;
pub use raft::common::LogEntry;
pub use raft::common::LogIndex;
pub use raft::common::PersistentStorage;
pub use raft::common::RaftConfig;
pub use raft::common::ServerId;
pub use raft::common::TermIndex;
pub use raft::rpc_messages;
pub use raft::start_raft_in_new_thread;
pub use raft::transport;
pub use raft::NoOpRaftEventCollector;
pub use raft::RaftNodeState;
pub use raft::RaftStateEvent;
pub use raft::RaftStateEventCollector;
