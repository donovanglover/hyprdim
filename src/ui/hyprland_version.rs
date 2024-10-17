use crate::queries::get_version;

pub fn hyprland_version(minimum_version: &str) -> anyhow::Result<bool> {
    let version = get_version()?;
    let version: Vec<&str> = version.split('.').collect();

    if version.len() != 3 {
        return Ok(false);
    }

    let version = semver::Version {
        major: version.get(0).unwrap().parse()?,
        minor: version.get(1).unwrap().parse()?,
        patch: version.get(2).unwrap().parse()?,
        pre: semver::Prerelease::EMPTY,
        build: semver::BuildMetadata::EMPTY,
    };

    let req = semver::VersionReq::parse(&format!(">={}", minimum_version)).unwrap();

    Ok(req.matches(&version))
}

#[cfg(test)]
mod tests {
    #[test]
    fn hyprland_version() {
        let res = super::hyprland_version("0.44.1").unwrap();
        assert_eq!(res, true);
    }
}
