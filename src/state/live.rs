use hyprland::data::Workspace;
use hyprland::shared::Address;
use std::sync::atomic::AtomicBool;
use std::sync::atomic::AtomicU16;
use std::sync::{Arc, Mutex};

pub struct LiveState {
    pub num_threads: Arc<AtomicU16>,
    pub last_address: Arc<Mutex<Option<Address>>>,
    pub last_class: Arc<Mutex<Option<String>>>,
    pub last_workspace: Arc<Mutex<Option<Workspace>>>,
    pub is_set_dim: Arc<AtomicBool>,
}

impl LiveState {
    pub fn new() -> LiveState {
        Self {
            num_threads: Arc::new(AtomicU16::new(0)),
            last_address: Arc::new(Mutex::new(None)),
            last_class: Arc::new(Mutex::new(None)),
            last_workspace: Arc::new(Mutex::new(None)),
            is_set_dim: Arc::new(AtomicBool::new(false)),
        }
    }
}
