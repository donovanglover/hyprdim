use hyprland::data::{Client, WorkspaceBasic, Workspaces};
use hyprland::prelude::*;

/// Returns true if there is only one visible window in the active workspace.
pub fn is_single() -> bool {
    if let Some(client) = Client::get_active().unwrap() {
        let Client { workspace, .. } = client;
        let WorkspaceBasic { id, .. } = workspace;

        for workspace in Workspaces::get().unwrap() {
            if workspace.id == id {
                return workspace.windows == 1 || workspace.fullscreen;
            }
        }
    }

    false
}

#[cfg(test)]
mod tests {
    #[test]
    #[ignore]
    fn is_single() {
        let res = super::is_single();
        assert_eq!(res, true);
    }
}
