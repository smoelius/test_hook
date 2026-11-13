use assert_cmd::assert::OutputAssertExt;
use cargo_metadata::{
    Dependency, Metadata, MetadataCommand,
    semver::{Op, Version, VersionReq},
};
use regex::Regex;
use std::{
    ffi::OsStr,
    fs::{read_dir, read_to_string},
    path::Path,
    process::Command,
    sync::LazyLock,
};
use tempfile::tempdir;
use test_hook::{test, test_hook};
use walkdir::WalkDir;

#[test_hook]
fn test_hook(command: &mut Command) {
    command.current_dir("..");
    command.env_remove("CARGO_TERM_COLOR");
}

#[test]
fn clippy() {
    let mut command = assert_cmd::Command::new("cargo");
    command.args([
        "+nightly",
        "clippy",
        "--all-targets",
        "--workspace",
        "--",
        "--deny=warnings",
    ]);
    command.assert().success();
}

#[test]
fn dylint() {
    let assert = Command::new("cargo")
        .args(["dylint", "--all", "--", "--all-targets"])
        .env("DYLINT_RUSTFLAGS", "--deny warnings")
        .assert();
    let stderr = String::from_utf8(assert.get_output().stderr.clone()).unwrap();
    assert!(assert.try_success().is_ok(), "{}", stderr);
}

#[test]
fn fixture_names_match_their_directories() {
    for result in WalkDir::new("fixtures") {
        let entry = result.unwrap();
        if entry.file_name() != OsStr::new("Cargo.toml") {
            continue;
        }
        let path = entry.path();
        let dir_name = path.parent().and_then(Path::file_name).unwrap();
        let contents = read_to_string(path).unwrap();
        let table = toml::from_str::<toml::Table>(&contents).unwrap();
        let package_name = table
            .get("package")
            .and_then(|value| value.as_table())
            .and_then(|table| table.get("name"))
            .and_then(toml::Value::as_str)
            .unwrap();
        assert_eq!(dir_name, package_name, "failed for `{}`", path.display());
    }
}

#[test]
fn fixtures_are_unpublishable() {
    for result in WalkDir::new("fixtures") {
        let entry = result.unwrap();
        if entry.file_name() != OsStr::new("Cargo.toml") {
            continue;
        }
        let path = entry.path();
        let contents = read_to_string(path).unwrap();
        let table = toml::from_str::<toml::Table>(&contents).unwrap();
        let publish = table
            .get("package")
            .and_then(|value| value.as_table())
            .and_then(|table| table.get("publish"))
            .and_then(toml::Value::as_bool);
        assert_eq!(Some(false), publish, "failed for `{}`", path.display());
    }
}

#[test]
fn fixtures_disable_doctests() {
    for result in WalkDir::new("fixtures") {
        let entry = result.unwrap();
        if entry.file_name() != OsStr::new("Cargo.toml") {
            continue;
        }
        let path = entry.path();
        let contents = read_to_string(path).unwrap();
        let table = toml::from_str::<toml::Table>(&contents).unwrap();
        let doctest = table
            .get("lib")
            .and_then(toml::Value::as_table)
            .and_then(|table| table.get("doctest"))
            .and_then(toml::Value::as_bool);
        assert_eq!(Some(false), doctest, "failed for `{}`", path.display());
    }
}

#[test]
fn fmt() {
    let mut command = Command::new("cargo");
    command.args(["+nightly", "fmt", "--check"]);
    command.assert().success();

    // The fixtures are excluded from the workspace, so check them separately.
    for result in read_dir("fixtures").unwrap() {
        let entry = result.unwrap();
        let path = entry.path();
        if !path.is_dir() {
            continue;
        }
        let mut command = Command::new("cargo");
        command.args(["+nightly", "fmt", "--check"]);
        command.current_dir(path);
        command.assert().success();
    }
}

#[cfg_attr(windows, ignore = "`node` is not installed on Windows")]
#[test]
fn markdown_link_check() {
    // smoelius: Use of `CARGO_MANIFEST_DIR` is needed because `markdown-link-check` is run from a
    // temporary directory.
    const README_MD: &str = concat!(env!("CARGO_MANIFEST_DIR"), "/../README.md");

    let tempdir = tempdir().unwrap();

    Command::new("npm")
        .args(["install", "markdown-link-check"])
        .current_dir(&tempdir)
        .assert()
        .success();

    Command::new("npx")
        .args(["markdown-link-check", README_MD])
        .current_dir(&tempdir)
        .assert()
        .success();
}

#[test]
fn readme_reference_links_are_sorted() {
    let re = Regex::new(r"^\[[^^\]]*\]:").unwrap();
    let readme = read_to_string("README.md").unwrap();
    let links = readme
        .lines()
        .filter(|line| re.is_match(line))
        .collect::<Vec<_>>();
    let mut links_sorted = links.clone();
    links_sorted.sort_unstable();
    assert!(
        links_sorted == links,
        "contents of README.md are not what was expected:\n{}",
        links_sorted.join("\n")
    );
}

static METADATA: LazyLock<Metadata> =
    LazyLock::new(|| MetadataCommand::new().no_deps().exec().unwrap());

#[test]
fn versions_are_equal() {
    for package in &METADATA.packages {
        assert_eq!(
            env!("CARGO_PKG_VERSION"),
            package.version.to_string(),
            "{}",
            package.name
        );
    }
}

#[test]
fn versions_are_exact_and_match() {
    for package in &METADATA.packages {
        for Dependency { name: dep, req, .. } in &package.dependencies {
            if dep == "test_hook_macro" {
                assert!(
                    is_exact(req),
                    "`{}` dependency on `{dep}` is not exact",
                    package.name
                );
                assert!(
                    req.matches(&Version::parse(env!("CARGO_PKG_VERSION")).unwrap()),
                    "`{}` dependency on `{dep}` does not match `{}`",
                    package.name,
                    env!("CARGO_PKG_VERSION"),
                );
            }
        }
    }
}

/// Determines whether `req` is an exact requirement.
fn is_exact(req: &VersionReq) -> bool {
    req.comparators
        .iter()
        .map(|comparator| comparator.op)
        .collect::<Vec<_>>()
        == [Op::Exact]
}
