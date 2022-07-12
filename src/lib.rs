use accelerator::AcceleratorId;
use crossbeam_channel::{unbounded, Receiver, Sender};
use once_cell::sync::Lazy;

pub mod accelerator;
pub mod error;
pub mod global_shortcut;
pub mod keyboard;

mod platform_impl;

#[macro_use]
extern crate bitflags;

/// Describes a generic event.
///
/// See the module-level docs for more information on the event loop manages each event.
#[non_exhaustive]
#[derive(Debug, PartialEq)]
pub enum Event {
  GlobalShortcutEvent(AcceleratorId),
}

static GLOBAL_SHORTCUT_CHANNEL: Lazy<(Sender<Event>, Receiver<Event>)> = Lazy::new(|| unbounded());

pub fn global_shortcut_event_receiver<'a>() -> &'a Receiver<Event> {
  &GLOBAL_SHORTCUT_CHANNEL.1
}
