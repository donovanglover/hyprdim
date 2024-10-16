use hyprland::data::{Client, WorkspaceBasic, Workspaces};
use hyprland::prelude::*;

/// Returns true if there is only one visible window in the active workspace.
///
/// In the future, this function should be updated to accommodate for fullscreen
/// windows in special workspaces if Hyprland implements it.
///
/// https://github.com/hyprwm/Hyprland/issues/2173
pub fn is_single() -> bool {
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

#[cfg(test)]
mod tests {
    #[test]
    fn is_single() {
        let res = super::is_single();
        assert_eq!(res, true);
    }
}
