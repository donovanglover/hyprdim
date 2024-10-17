use hyprland::keyword::Keyword;

use crate::utils::log;

/// Sets the dim strength to a specific value permanently until it gets changed again.
///
/// Useful for setting the dim of dialog windows.
pub fn set_dim(strength: f64) -> hyprland::Result<()> {
    Keyword::set("decoration:dim_inactive", "yes")?;
    Keyword::set("decoration:dim_strength", strength)?;

    log("info: Set a permanent dim (until next event) without spawning thread");

    Ok(())
}
