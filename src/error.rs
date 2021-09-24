pub use failure::{Error, err_msg};
/// Result type
pub type Result<T> = std::result::Result<T, Error>;