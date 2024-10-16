use hyprland::data::Client;
use hyprland::prelude::*;

/// Checks if the active window is floating or not.
///
/// Returns false if no window is active.
pub fn is_floating() -> bool {
    if let Some(client) = Client::get_active().unwrap() {
        let Client { floating, .. } = client;
        return floating;
    }

    false
}

#[cfg(test)]
mod tests {
    #[test]
    #[ignore]
    fn is_floating() {
        let res = super::is_floating();
        assert_eq!(res, true);
    }
}
