use crate::queries::get_version;

pub fn hyprland_version(minimum_version: &str) -> anyhow::Result<bool> {
    let version = get_version()?;
    let mut version = version.split('.');

    let version = semver::Version {
        major: version.next().unwrap_or("99").parse()?,
        minor: version.next().unwrap_or("99").parse()?,
        patch: version.next().unwrap_or("99").parse()?,
        pre: semver::Prerelease::EMPTY,
        build: semver::BuildMetadata::EMPTY,
    };

    let req = semver::VersionReq::parse(&format!(">={}", minimum_version)).unwrap();

    Ok(req.matches(&version))
}

#[cfg(test)]
mod tests {
    #[test]
    #[ignore]
    fn hyprland_version() {
        let res = super::hyprland_version("0.44.1").unwrap();
        assert_eq!(res, true);
    }
}
