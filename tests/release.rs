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

    assert_eq!(pkgbuild, cargo.as_str());
}
