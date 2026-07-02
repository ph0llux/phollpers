#[cfg(feature = "read_at")]
/// Default buffer size for read operations (64KB)
pub const DEFAULT_READ_BUFFER_SIZE: usize = 65536;
#[cfg(feature = "read_at")]
/// Small buffer size for metadata operations
pub const SMALL_BUFFER_SIZE: usize = 8192;