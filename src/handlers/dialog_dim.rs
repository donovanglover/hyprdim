use crate::{cli::Cli, mutations::set_dim, queries::is_floating};

pub struct DialogDimOptions {
    pub same_workspace: bool,
    pub same_class: bool,
}

pub fn dialog_dim(cli: &Cli, options: DialogDimOptions) -> bool {
    if let Some(dialog_strength) = cli.dialog_dim {
        if options.same_workspace && options.same_class && is_floating() {
            set_dim(dialog_strength, cli.persist).unwrap();

            return true;
        }
    }

    false
}
