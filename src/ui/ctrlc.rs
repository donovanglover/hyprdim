use std::sync::mpsc;
use std::{process, thread};

use hyprdim::log;
use hyprland::keyword::Keyword;

use crate::state::DimState;

pub fn ctrlc(state: DimState) {
    thread::spawn(move || -> hyprland::Result<()> {
        let (tx, rx) = mpsc::channel();

        ctrlc::set_handler(move || tx.send(()).expect("Could not send signal on channel."))
            .expect("Error setting Ctrl-C handler");

        rx.recv().expect("Could not receive from channel.");

        Keyword::set("decoration:dim_strength", state.dim_strength)?;
        Keyword::set("decoration:dim_inactive", state.dim_inactive)?;

        log("\nhyprdim terminated successfully.");

        process::exit(0);
    });
}
