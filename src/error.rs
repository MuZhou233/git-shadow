pub use failure::{err_msg, Error};
/// Result type
pub type Result<T> = std::result::Result<T, Error>;
