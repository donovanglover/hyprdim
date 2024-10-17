use hyprland::data::Version;
use hyprland::prelude::*;

pub fn get_version() -> anyhow::Result<String> {
    let version = Version::get()?;
    let version = version.branch.trim_start_matches("v").trim_end_matches("-b");

    Ok(version.to_string())
}

#[cfg(test)]
mod tests {
    #[test]
    #[ignore]
    fn get_version() {
        let res = super::get_version().unwrap();
        assert_eq!(res, "0.44.1");
    }
}
