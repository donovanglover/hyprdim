use hyprland::data::{Client, WorkspaceBasic, Workspaces};
use hyprland::prelude::*;

/// Returns true if there is only one visible window in the special workspace.
///
/// In the future, this function should be updated to accommodate for fullscreen
/// windows in special workspaces if Hyprland implements it.
///
/// https://github.com/hyprwm/Hyprland/issues/2173
pub fn special_only_has_one_visible_window() -> bool {
    if let Some(client) = Client::get_active().unwrap() {
        let Client { workspace, .. } = client;
        let WorkspaceBasic { id, .. } = workspace;

        for workspace in Workspaces::get().unwrap() {
            if workspace.id == id {
                return workspace.windows == 1;
            }
        }
    }

    false
}
