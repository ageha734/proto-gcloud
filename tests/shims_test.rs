#[cfg(feature = "wasm")]
use proto_pdk_test_utils::*;

#[cfg(feature = "wasm")]
mod gcloud_tool {
    use super::*;

    #[cfg(not(windows))]
    generate_shims_test!("gcloud-test", ["gcloud"]);
}
