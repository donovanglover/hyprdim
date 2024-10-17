use hyprland::data::Workspace;
use hyprland::prelude::*;

pub fn get_parent() -> Workspace {
    Workspace::get_active().unwrap()
}
