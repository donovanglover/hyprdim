use rustympkglib::pkgdata::PkgData;
use std::fs;
use serde::Deserialize;

#[derive(Debug, Deserialize)]
struct Config {
    package: Option<PackageConfig>,
}

#[derive(Debug, Deserialize)]
struct PackageConfig {
    version: Option<String>,
    authors: Option<Vec<String>>,
}

#[test]
/// We have the version number in quite a few places. This test ensures that all
/// version numbers are updated as appropriate when a new version is released.
fn versions_are_the_same() {
    let pkgbuild = &fs::read_to_string("PKGBUILD").unwrap();
    let pkgbuild = PkgData::from_source(pkgbuild).unwrap();
    let pkgbuild = pkgbuild.pkgver;

    let cargo = &fs::read_to_string("Cargo.toml").unwrap();
    let cargo: Config = toml::from_str(cargo).unwrap();
    let cargo = cargo.package.unwrap().version.unwrap();

    assert_eq!(
        pkgbuild,
        cargo.as_str(),
        "Cargo.toml and PKGBUILD should have the same version"
    );
}

#[test]
/// Ensures that the copyright year is updated in both files if the LICENSE is updated
fn copyright_is_the_same() {
    let license = &fs::read_to_string("LICENSE").unwrap();
    let license = license.split("\n").collect::<Vec<&str>>()[0];

    let cargo = &fs::read_to_string("Cargo.toml").unwrap();
    let cargo: Config = toml::from_str(cargo).unwrap();
    let cargo = &cargo.package.unwrap().authors.unwrap()[0];

    assert!(
        cargo.starts_with(license),
        "Cargo.toml should have the same copyright year as LICENSE"
    );
}

#[test]
/// Ensures that the usage code block in the README is the same as the output of hyprdim -h
fn usage_is_the_same() {
    let readme = &fs::read_to_string("README.md").unwrap();
    let mut inside_code_block = false;
    let mut readme_usage = String::new();

    for line in readme.lines() {
        if line == "```" {
            inside_code_block = false;
            continue;
        }

        if inside_code_block {
            readme_usage.push_str(&(line.to_owned() + "\n"));
            continue;
        }

        if line == "```man" {
            inside_code_block = true;
        }
    }

    println!("{}", readme_usage);

    assert!(false)
}
