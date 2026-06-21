#[cfg(feature = "wasm")]
use proto_pdk_test_utils::*;

#[cfg(feature = "wasm")]
mod gcloud_tool {
    use super::*;
    use std::ffi::OsString;

    struct EnvGuard {
        key: &'static str,
        value: Option<OsString>,
    }

    impl EnvGuard {
        fn set(key: &'static str, value: impl AsRef<std::ffi::OsStr>) -> Self {
            let value_before = std::env::var_os(key);

            unsafe {
                std::env::set_var(key, value);
            }

            Self {
                key,
                value: value_before,
            }
        }
    }

    impl Drop for EnvGuard {
        fn drop(&mut self) {
            if let Some(value) = &self.value {
                unsafe {
                    std::env::set_var(self.key, value);
                }
            } else {
                unsafe {
                    std::env::remove_var(self.key);
                }
            }
        }
    }

    #[cfg(not(windows))]
    #[tokio::test(flavor = "multi_thread")]
    async fn creates_shims() {
        let sandbox = create_empty_proto_sandbox();
        let proto_bin_dir = sandbox.proto_dir.join("bin");

        std::fs::create_dir_all(&proto_bin_dir).unwrap();
        std::fs::write(proto_bin_dir.join("proto-shim"), b"proto-shim").unwrap();

        let _proto_home = EnvGuard::set("PROTO_HOME", &sandbox.proto_dir);
        let mut plugin = sandbox.create_plugin("gcloud-test").await;

        flow::link::Linker::new(&mut plugin.tool, &ToolSpec::default())
            .link_shims(false)
            .await
            .unwrap();

        assert!(sandbox.proto_dir.join("shims").join("gcloud").exists());

        starbase_sandbox::assert_snapshot!(
            std::fs::read_to_string(sandbox.path().join(".proto/shims/registry.json")).unwrap()
        );
    }
}
