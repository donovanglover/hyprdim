use rustympkglib::pkgdata::PkgData;
use std::fs;

#[test]
/// We have the version number in quite a few places. This test ensures that all
/// version numbers are updated as appropriate when a new version is released.
fn versions_are_the_same() {
    let pkgbuild = &fs::read_to_string("PKGBUILD").unwrap();
    let pkgbuild = PkgData::from_source(pkgbuild).unwrap();
    let pkgbuild = pkgbuild.pkgver;

    // let cargo = &fs::read_to_string("Cargo.toml").unwrap();
    // let cargo = TODO: Parse toml
    assert_eq!(pkgbuild, "2.0.1");
}
