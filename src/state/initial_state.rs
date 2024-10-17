use hyprland::keyword::{Keyword, OptionValue};

pub struct InitialState {
    dim_strength: f64,
    dim_inactive: i64,
}

impl InitialState {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self {
            dim_strength: match Keyword::get("decoration:dim_strength")?.value {
                OptionValue::Float(i) => i,
                _ => 0.5,
            },

            dim_inactive: match Keyword::get("decoration:dim_inactive")?.value {
                OptionValue::Int(i) => i,
                _ => 0,
            },
        })
    }

    pub fn restore(self) -> anyhow::Result<()> {
        Keyword::set("decoration:dim_strength", self.dim_strength)?;
        Keyword::set("decoration:dim_inactive", self.dim_inactive)?;

        Ok(())
    }
}
