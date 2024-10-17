use std::sync::mpsc;
use std::{process, thread};

use crate::utils::log;

use crate::state::InitialState;

pub fn ctrlc(state: InitialState) {
    thread::spawn(move || -> anyhow::Result<()> {
        let (tx, rx) = mpsc::channel();

        ctrlc::set_handler(move || tx.send(()).unwrap())?;

        rx.recv()?;

        state.restore()?;

        log("\nhyprdim terminated successfully.");

        process::exit(0);
    });
}
