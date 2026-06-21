#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
pub mod config;
pub mod version;

#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
mod proto;

#[cfg(all(feature = "wasm", target_arch = "wasm32"))]
pub use proto::*;

pub use version::extract_version_from_name;
