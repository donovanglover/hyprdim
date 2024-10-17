use crate::{cli::Cli, mutations::set_dim, queries::is_floating};

pub fn dialog_dim(cli: &Cli) -> bool {
    if is_floating() {
        set_dim(cli.dialog_dim).unwrap();

        return true;
    }

    false
}
