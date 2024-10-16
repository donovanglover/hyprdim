use hyprland::keyword::Keyword;

pub fn set_animation(fade: u8, bezier: String) -> anyhow::Result<()> {
    Keyword::set("animation", format!("fadeDim,1,{fade},{bezier}"))?;

    Ok(())
}
