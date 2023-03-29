/// Common code (for example system clock abstraction that allows for mocking time)
pub mod system_clock;
pub use system_clock::now;
pub use system_clock::Instant;
