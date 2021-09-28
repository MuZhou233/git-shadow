#![warn(missing_docs)]
//! git-shadow library

/// Command argument rules
pub mod arguments;
/// Get information from git
pub mod git;
/// Command Line logging
pub mod logging;

mod error;
pub use error::{err_msg, Result};
