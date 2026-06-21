#[cfg(feature = "wasm")]
use proto_pdk_test_utils::*;

#[cfg(feature = "wasm")]
mod gcloud_tool {
    use super::*;

    async fn download_url_for(os: HostOS, arch: HostArch) -> String {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox
            .create_plugin_with_config("gcloud-test", |config| {
                config.host(os, arch);
            })
            .await;

        plugin
            .download_prebuilt(DownloadPrebuiltInput {
                context: PluginContext {
                    version: VersionSpec::parse("533.0.0").unwrap(),
                    ..Default::default()
                },
                ..Default::default()
            })
            .await
            .download_url
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn generates_linux_download_url() {
        assert_eq!(
            download_url_for(HostOS::Linux, HostArch::X64).await,
            "https://storage.googleapis.com/storage/v1/b/cloud-sdk-release/o/google-cloud-cli-533.0.0-linux-x86_64.tar.gz?alt=media"
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn generates_macos_download_url() {
        assert_eq!(
            download_url_for(HostOS::MacOS, HostArch::Arm64).await,
            "https://storage.googleapis.com/storage/v1/b/cloud-sdk-release/o/google-cloud-cli-533.0.0-darwin-arm.tar.gz?alt=media"
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn generates_windows_download_url() {
        assert_eq!(
            download_url_for(HostOS::Windows, HostArch::X64).await,
            "https://storage.googleapis.com/storage/v1/b/cloud-sdk-release/o/google-cloud-cli-533.0.0-windows-x86_64.zip?alt=media"
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn locates_linux_bin() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox
            .create_plugin_with_config("gcloud-test", |config| {
                config.host(HostOS::Linux, HostArch::Arm64);
            })
            .await;

        assert_eq!(
            plugin
                .locate_executables(LocateExecutablesInput {
                    context: PluginContext {
                        version: VersionSpec::parse("533.0.0").unwrap(),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .await
                .exes
                .get("gcloud")
                .unwrap()
                .exe_path,
            Some("google-cloud-sdk/bin/gcloud".into())
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn locates_macos_bin() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox
            .create_plugin_with_config("gcloud-test", |config| {
                config.host(HostOS::MacOS, HostArch::X64);
            })
            .await;

        assert_eq!(
            plugin
                .locate_executables(LocateExecutablesInput {
                    context: PluginContext {
                        version: VersionSpec::parse("533.0.0").unwrap(),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .await
                .exes
                .get("gcloud")
                .unwrap()
                .exe_path,
            Some("google-cloud-sdk/bin/gcloud".into())
        );
    }

    #[tokio::test(flavor = "multi_thread")]
    async fn locates_windows_bin() {
        let sandbox = create_empty_proto_sandbox();
        let plugin = sandbox
            .create_plugin_with_config("gcloud-test", |config| {
                config.host(HostOS::Windows, HostArch::X86);
            })
            .await;

        assert_eq!(
            plugin
                .locate_executables(LocateExecutablesInput {
                    context: PluginContext {
                        version: VersionSpec::parse("533.0.0").unwrap(),
                        ..Default::default()
                    },
                    ..Default::default()
                })
                .await
                .exes
                .get("gcloud")
                .unwrap()
                .exe_path,
            Some("google-cloud-sdk/bin/gcloud.cmd".into())
        );
    }
}
