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
fn build_tree_has_one_exact_strict_bootstrap_producer_train() {
    let output = Command::new("cargo")
        .args(["tree", "--no-default-features"])
        .output()
        .expect("run cargo tree");
    assert!(output.status.success(), "status: {:?}", output.status);
    let tree = String::from_utf8(output.stdout).expect("dependency tree");
    let lock = include_str!("../Cargo.lock");
    for exact_source in [
        "core-ethos.git?rev=43b48c779c54ee9f05cbcc111d5d88074b162461#43b48c77",
        "core-logos.git?rev=734363bd5fca01c7dab46028f4a36e3ce6ae6650#734363bd",
        "core-nomos.git?rev=7b60721d199551b648d42a49934a2f0ef950c595#7b60721d",
        "protos.git?rev=cdc74bd28187bdb39b8ddc2228eef4934873dd45#cdc74bd2",
        "rust-logos.git?rev=081e99596826b15e2ff7f1356ae8d797b18aeffc#081e9959",
        "schema-rust.git?rev=664335240a40728826cfaa09e3100cd867031912#66433524",
        "sema-translator.git?rev=287fbd728a05b1a6be1dc8a28bcf3ca06d9916b3#287fbd72",
        "signal-frame.git?rev=8aa0bcaeb29fe9e461a11706a469638d2fd109ac#8aa0bcae",
        "signal-sema-translator.git?rev=3f41813dd63904c7e2b3da4382eff64ed1bf12fe#3f41813d",
    ] {
        assert!(
            tree.contains(exact_source),
            "missing {exact_source}:\n{tree}"
        );
    }
    for sole_package in [
        "core-ethos",
        "core-logos",
        "core-nomos",
        "protos",
        "rust-logos",
        "schema-rust",
        "sema-translator",
        "signal-frame",
        "signal-sema-translator",
    ] {
        assert_eq!(
            lock.matches(&format!("name = \"{sole_package}\"")).count(),
            1,
            "build graph contains more than one {sole_package}:\n{tree}"
        );
    }
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
