use std::process::Command;

#[test]
fn runtime_tree_excludes_bootstrap_and_retired_crates() {
    let output = Command::new("cargo")
        .args(["tree", "--edges", "normal", "--no-default-features"])
        .output()
        .expect("run cargo tree");
    assert!(output.status.success(), "status: {:?}", output.status);
    let tree = String::from_utf8(output.stdout).expect("dependency tree");
    for forbidden in [
        "core-ethos",
        "name-table",
        "protos",
        "rust-logos",
        "schema-language",
        "schema-rust",
        "sema-translator",
        "signal-sema-translator",
        "structural-codec",
    ] {
        assert!(
            !tree.contains(forbidden),
            "runtime contains {forbidden}:\n{tree}"
        );
    }
}

#[test]
fn build_tree_has_one_corrected_schema_rust() {
    let output = Command::new("cargo")
        .args(["tree", "--edges", "build", "--no-default-features"])
        .output()
        .expect("run cargo tree");
    assert!(output.status.success(), "status: {:?}", output.status);
    let tree = String::from_utf8(output.stdout).expect("dependency tree");
    assert_eq!(tree.matches("schema-rust v0.15.0").count(), 1, "{tree}");
    assert!(tree.contains("schema-rust.git?rev=9e36587c85bd69357e9042729ba2df0052799756#9e36587c"));
    assert!(!tree.contains("schema-language"), "{tree}");
}

#[test]
fn historical_schema_surface_is_absent() {
    let root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let mut directories = vec![root.to_path_buf()];
    while let Some(directory) = directories.pop() {
        for entry in std::fs::read_dir(directory).expect("read repository directory") {
            let path = entry.expect("read repository entry").path();
            if path.ends_with("target") || path.ends_with(".git") || path.ends_with(".jj") {
                continue;
            }
            if path.is_dir() {
                directories.push(path);
            } else {
                assert_ne!(
                    path.extension().and_then(std::ffi::OsStr::to_str),
                    Some("schema"),
                    "historical schema input survived at {}",
                    path.display(),
                );
            }
        }
    }
    for source in [
        include_str!("../Cargo.toml"),
        include_str!("../build.rs"),
        include_str!("../src/lib.rs"),
    ] {
        for forbidden in [".schema", "schema-dir", "CargoSchemaMetadata"] {
            assert!(
                !source.contains(forbidden),
                "historical token {forbidden} survived"
            );
        }
    }
}
