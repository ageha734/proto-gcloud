use gcloud_plugin::*;
use proto_pdk_test_utils::*;
use starbase_sandbox::create_empty_sandbox;

generate_resolve_versions_tests!("gcloud-test", {
    "519" => "519.0.0",
    "519.0" => "519.0.0",
    "519.0.0" => "519.0.0",
});

#[test]
fn can_load_versions() {
    let sandbox = create_empty_sandbox();
    let plugin = create_plugin("gcloud-test", sandbox.path());

    let output = plugin.load_versions(LoadVersionsInput::default());

    assert!(!output.versions.is_empty());
}

#[test]
fn sets_latest_alias() {
    let sandbox = create_empty_sandbox();
    let plugin = create_plugin("gcloud-test", sandbox.path());

    let output = plugin.load_versions(LoadVersionsInput::default());

    assert!(output.latest.is_some());
    assert!(output.aliases.contains_key("latest"));
    assert_eq!(output.aliases.get("latest"), output.latest.as_ref());
}

#[test]
fn test_extract_version_from_name() {
    // Test basic version extraction
    assert_eq!(
        extract_version_from_name("google-cloud-cli-371.0.0-darwin-arm.tar.gz"),
        Some("371.0.0".to_string())
    );

    assert_eq!(
        extract_version_from_name("google-cloud-cli-383.0.1-linux-x86.tar.gz"),
        Some("383.0.1".to_string())
    );

    assert_eq!(
        extract_version_from_name("google-cloud-cli-469.0.0-windows-arm.zip"),
        Some("469.0.0".to_string())
    );

    // Test different platforms and architectures
    assert_eq!(
        extract_version_from_name("google-cloud-cli-464.0.0-darwin-x86.tar.gz"),
        Some("464.0.0".to_string())
    );

    assert_eq!(
        extract_version_from_name("google-cloud-cli-465.0.0-linux-arm.tar.gz"),
        Some("465.0.0".to_string())
    );

    assert_eq!(
        extract_version_from_name("google-cloud-cli-466.0.0-windows-x86.zip"),
        Some("466.0.0".to_string())
    );

    // Test patch versions
    assert_eq!(
        extract_version_from_name("google-cloud-cli-405.0.1-darwin-arm.tar.gz"),
        Some("405.0.1".to_string())
    );

    assert_eq!(
        extract_version_from_name("google-cloud-cli-417.0.1-linux-x86.tar.gz"),
        Some("417.0.1".to_string())
    );

    // Test without file extensions
    assert_eq!(
        extract_version_from_name("google-cloud-cli-371.0.0-darwin-arm"),
        Some("371.0.0".to_string())
    );

    // Test invalid cases
    assert_eq!(
        extract_version_from_name("not-gcloud-cli-371.0.0-darwin-arm.tar.gz"),
        None
    );

    assert_eq!(
        extract_version_from_name("google-cloud-cli-invalid-version-darwin-arm.tar.gz"),
        None
    );

    assert_eq!(
        extract_version_from_name("google-cloud-cli-371.0-darwin-arm.tar.gz"),
        None
    );

    assert_eq!(
        extract_version_from_name("google-cloud-cli-371-darwin-arm.tar.gz"),
        None
    );

    // Test edge cases
    assert_eq!(extract_version_from_name(""), None);

    assert_eq!(extract_version_from_name("google-cloud-cli-"), None);
}

#[test]
fn test_extract_version_removes_platform_arch() {
    // Ensure that platform and architecture information is completely removed
    let test_cases = vec![
        ("google-cloud-cli-371.0.0-darwin-arm.tar.gz", "371.0.0"),
        ("google-cloud-cli-371.0.0-darwin-x86.tar.gz", "371.0.0"),
        ("google-cloud-cli-371.0.0-linux-arm.tar.gz", "371.0.0"),
        ("google-cloud-cli-371.0.0-linux-x86.tar.gz", "371.0.0"),
        ("google-cloud-cli-371.0.0-windows-arm.zip", "371.0.0"),
        ("google-cloud-cli-371.0.0-windows-x86.zip", "371.0.0"),
    ];

    for (input, expected) in test_cases {
        assert_eq!(
            extract_version_from_name(input),
            Some(expected.to_string()),
            "Failed for input: {}",
            input
        );
    }
}

#[test]
fn test_extract_version_handles_patch_versions() {
    // Test that patch versions (x.y.z.w) are handled correctly
    let test_cases = vec![
        ("google-cloud-cli-383.0.1-darwin-arm.tar.gz", "383.0.1"),
        ("google-cloud-cli-405.0.1-linux-x86.tar.gz", "405.0.1"),
        ("google-cloud-cli-417.0.1-windows-arm.zip", "417.0.1"),
        ("google-cloud-cli-433.0.1-darwin-x86.tar.gz", "433.0.1"),
        ("google-cloud-cli-435.0.1-linux-arm.tar.gz", "435.0.1"),
    ];

    for (input, expected) in test_cases {
        assert_eq!(
            extract_version_from_name(input),
            Some(expected.to_string()),
            "Failed for input: {}",
            input
        );
    }
}
