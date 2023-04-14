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
mod default_storage;
mod raft_thread;
pub mod rpc_messages;
mod state_machine;
pub mod system_clock;

pub use common::LogCommand;
pub use common::LogEntry;
pub use common::LogIndex;
pub use common::PersistentStorage;
pub use common::RaftConfig;
pub use common::ServerId;
pub use common::TermIndex;
pub use common::*;
pub use raft_thread::start_raft_in_new_thread;
pub use raft_thread::NoOpRaftEventCollector;
pub use raft_thread::RaftNodeState;
pub use raft_thread::RaftStateEvent;
pub use raft_thread::RaftStateEventCollector;
pub use rpc_messages::*;
