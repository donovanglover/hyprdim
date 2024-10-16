use hyprland::keyword::{Keyword, OptionValue};

pub struct DimState {
    pub dim_strength: f64,
    pub dim_inactive: i64,
}

impl DimState {
    pub fn new() -> anyhow::Result<DimState> {
        let dim_strength = match Keyword::get("decoration:dim_strength")?.value {
            OptionValue::Float(i) => i,
            _ => 0.5,
        };

        let dim_inactive = match Keyword::get("decoration:dim_inactive")?.value {
            OptionValue::Int(i) => i,
            _ => 0,
        };

        Ok(Self {
            dim_strength,
            dim_inactive
        })
    }
}
