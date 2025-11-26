//! A set of very common logging [clap] options which also initialize various logging/tracing backends
//!
//! # TODOs
//!
//! - Implement features for different logger backends (e.g. `env_logger`, `tracing`, etc...)
//! - Provide a facility to defer to the app for argument names, especially short `-q` style arg names to work with apps with colliding options
#![deny(unsafe_code, missing_docs)]

pub mod backend;
mod backends;
mod stdcons;
mod verbosity;

pub use self::backend::Backend;
pub use self::stdcons::StandardConsole;
pub use self::verbosity::Verbosity;
