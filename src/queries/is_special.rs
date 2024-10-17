use hyprland::data::Client;
use hyprland::prelude::*;

/// Gets whether the current workspace is a special workspace or not.
///
/// This function works by getting which workspace the active window is in.
///
/// The if statement is used to make sure this function works when no window
/// is the active window.
pub fn is_special() -> bool {
    if let Some(client) = Client::get_active().unwrap() {
        let Client { workspace, .. } = client;
        return workspace.name.contains("special");
    }

    false
}

#[cfg(test)]
mod tests {
    #[test]
    #[ignore]
    fn is_special() {
        let res = super::is_special();
        assert_eq!(res, true);
    }
}
