use accelerator::AcceleratorId;
use crossbeam_channel::{unbounded, Receiver, Sender};
use once_cell::sync::Lazy;

pub mod accelerator;
pub mod global_shortcut;
pub mod keyboard;
pub mod error;

mod platform_impl;

#[macro_use]
extern crate bitflags;

static GLOBAL_SHORTCUT_CHANNEL: Lazy<(Sender<GlobalShortcutEvent>, Receiver<GlobalShortcutEvent>)> =
    Lazy::new(|| unbounded());

pub fn global_shortcut_event_receiver<'a>() -> &'a Receiver<GlobalShortcutEvent> {
    &GLOBAL_SHORTCUT_CHANNEL.1
}

pub struct GlobalShortcutEvent {
    pub id: i32,
}
