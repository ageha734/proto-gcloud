use crate::config::GcloudPluginConfig;
use extism_pdk::*;
use proto_pdk::*;
use schematic::SchemaBuilder;

use std::collections::HashMap;

#[host_fn]
extern "ExtismHost" {
    fn exec_command(input: Json<ExecCommandInput>) -> Json<ExecCommandOutput>;
}

static NAME: &str = "Gcloud";

fn check_version_for_os_and_arch(
    env: &HostEnvironment,
    _version_spec: &VersionSpec,
) -> FnResult<()> {
    match env.os {
        HostOS::Linux | HostOS::MacOS | HostOS::Windows => {},
        _ => return Err(plugin_err!("Unsupported operating system: {:?}", env.os)),
    }

    match env.arch {
        HostArch::X86 | HostArch::X64 | HostArch::Arm | HostArch::Arm64 => {},
        _ => return Err(plugin_err!("Unsupported architecture: {:?}", env.arch)),
    }

    Ok(())
}

#[derive(serde::Deserialize)]
struct GcloudReleaseResponse {
    items: Vec<GcloudReleaseItem>,
    #[serde(rename = "nextPageToken")]
    next_page_token: Option<String>,
}

#[derive(serde::Deserialize)]
struct GcloudReleaseItem {
    name: String,
}

#[plugin_fn]
pub fn register_tool(Json(_): Json<RegisterToolInput>) -> FnResult<Json<RegisterToolOutput>> {
    Ok(Json(RegisterToolOutput {
        name: NAME.into(),
        minimum_proto_version: Some(Version::new(0, 51, 4)),
        type_of: PluginType::CommandLine,
        default_install_strategy: InstallStrategy::DownloadPrebuilt,
        config_schema: Some(SchemaBuilder::build_root::<GcloudPluginConfig>()),
        plugin_version: Version::parse(env!("CARGO_PKG_VERSION")).ok(),
        ..RegisterToolOutput::default()
    }))
}

#[plugin_fn]
pub fn load_versions(Json(_): Json<LoadVersionsInput>) -> FnResult<Json<LoadVersionsOutput>> {
    let mut all_versions = std::collections::HashSet::new();
    let mut next_page_token: Option<String> = None;

    loop {
        let url = if let Some(ref token) = next_page_token {
            format!(
                "https://storage.googleapis.com/storage/v1/b/cloud-sdk-release/o?prefix=google-cloud-cli&fields=kind,nextPageToken,items(name)&pageToken={}",
                token
            )
        } else {
            "https://storage.googleapis.com/storage/v1/b/cloud-sdk-release/o?prefix=google-cloud-cli&fields=kind,nextPageToken,items(name)".to_string()
        };

        let response: GcloudReleaseResponse = fetch_json(&url)?;

        for item in &response.items {
            if let Some(version) = extract_version_from_name(&item.name) {
                all_versions.insert(version);
            }
        }

        if let Some(token) = response.next_page_token {
            next_page_token = Some(token);
        } else {
            break;
        }
    }

    let version_strings: Vec<String> = all_versions.into_iter().collect();
    Ok(Json(LoadVersionsOutput::from(version_strings)?))
}

pub fn extract_version_from_name(name: &str) -> Option<String> {
    if !name.contains("google-cloud-cli-") {
        return None;
    }

    let chars = name.chars().peekable();
    let mut current_token = String::new();
    let mut in_version = false;
    let mut dot_count = 0;

    for ch in chars {
        if ch.is_ascii_digit() {
            current_token.push(ch);
            in_version = true;
        } else if ch == '.' && in_version {
            current_token.push(ch);
            dot_count += 1;
        } else {
            if in_version && dot_count >= 2 && !current_token.is_empty() {
                let parts: Vec<&str> = current_token.split('.').collect();
                if parts.len() >= 3
                    && parts
                        .iter()
                        .all(|p| !p.is_empty() && p.chars().all(|c| c.is_ascii_digit()))
                {
                    return Some(current_token);
                }
            }
            current_token.clear();
            in_version = false;
            dot_count = 0;
        }
    }

    if in_version && dot_count >= 2 && !current_token.is_empty() {
        let parts: Vec<&str> = current_token.split('.').collect();
        if parts.len() >= 3
            && parts
                .iter()
                .all(|p| !p.is_empty() && p.chars().all(|c| c.is_ascii_digit()))
        {
            return Some(current_token);
        }
    }

    None
}

#[plugin_fn]
pub fn download_prebuilt(
    Json(input): Json<DownloadPrebuiltInput>,
) -> FnResult<Json<DownloadPrebuiltOutput>> {
    let env = get_host_environment()?;

    let version_spec = input.context.version;

    check_version_for_os_and_arch(&env, &version_spec)?;

    let config = get_tool_config::<GcloudPluginConfig>()?;

    let platform = match env.os {
        HostOS::Linux => "linux",
        HostOS::MacOS => "darwin",
        HostOS::Windows => "windows",
        _ => unreachable!(),
    };

    let arch = match env.arch {
        HostArch::X86 => "x86_64",
        HostArch::X64 => "x64",
        HostArch::Arm => "arm",
        HostArch::Arm64 => "arm",
        _ => unreachable!(),
    };

    let version = version_spec.as_version().unwrap();

    let filename = if env.os.is_windows() {
        "zip".to_string()
    } else {
        "tar.gz".to_string()
    };

    let download_url = config
        .dist_url
        .replace("{version}", version.to_string().as_str())
        .replace("{platform}", platform)
        .replace("{arch}", arch)
        .replace("{ext}", &filename);

    Ok(Json(DownloadPrebuiltOutput {
        download_url,
        checksum_url: None,
        ..DownloadPrebuiltOutput::default()
    }))
}

#[plugin_fn]
pub fn locate_executables(
    Json(_): Json<LocateExecutablesInput>,
) -> FnResult<Json<LocateExecutablesOutput>> {
    let env = get_host_environment()?;

    Ok(Json(LocateExecutablesOutput {
        exes: HashMap::from_iter([
            (
                "gcloud".into(),
                ExecutableConfig::new_primary(
                    env.os
                        .for_native("google-cloud-sdk/bin/gcloud", "google-cloud-sdk/bin/gcloud"),
                ),
            ),
            (
                "gsutil".into(),
                ExecutableConfig {
                    no_bin: true,
                    ..ExecutableConfig::default()
                },
            ),
        ]),
        globals_lookup_dirs: vec!["$HOME/.gcloud/bin".into()],
        ..LocateExecutablesOutput::default()
    }))
}
