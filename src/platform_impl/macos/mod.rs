mod global_shortcut;
mod keycode;

pub use self::{
  global_shortcut::{GlobalShortcut, ShortcutManager},
  keycode::{keycode_from_scancode, keycode_to_scancode},
};
use std::fmt;

#[non_exhaustive]
#[derive(Debug)]
pub enum OsError {
  CGError(core_graphics::base::CGError),
  CreationError(&'static str),
}

impl fmt::Display for OsError {
  fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      OsError::CGError(e) => f.pad(&format!("CGError {}", e)),
      OsError::CreationError(e) => f.pad(e),
    }
  }
}
