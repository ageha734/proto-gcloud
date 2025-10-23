#[cfg(feature = "wasm")]
pub mod config;
pub mod version;

#[cfg(feature = "wasm")]
mod proto;

#[cfg(feature = "wasm")]
pub use proto::*;

pub use version::extract_version_from_name;
