use crate::{cli::Cli, mutations::set_dim, queries::is_floating};

pub fn dialog_dim(cli: &Cli) -> bool {
    if let Some(dialog_strength) = cli.dialog_dim {
        if is_floating() {
            set_dim(dialog_strength).unwrap();

            return true;
        }
    }

    false
}
