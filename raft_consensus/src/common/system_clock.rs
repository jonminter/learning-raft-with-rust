#[cfg(feature = "mock_time")]
pub use mock_instant::Instant;

#[cfg(not(feature = "mock_time"))]
pub use std::time::Instant;

/// Return the current system monotonic clock time
pub fn now() -> Instant {
    Instant::now()
}
