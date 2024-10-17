use hyprland::keyword::Keyword;

/// Sets the dim strength to a specific value
///
/// Always sets `dim_inactive` to ensure that changes are applied.
pub fn set_dim(strength: f64) -> hyprland::Result<()> {
    Keyword::set("decoration:dim_inactive", "yes")?;
    Keyword::set("decoration:dim_strength", strength)?;

    Ok(())
}
