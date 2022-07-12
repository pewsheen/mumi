use std::str::FromStr;

use mumi::{
  accelerator::{Accelerator, AcceleratorId, RawMods, SysMods},
  global_shortcut::ShortcutManager,
  global_shortcut_event_receiver,
  keyboard::KeyCode,
  Event::GlobalShortcutEvent,
};

use winit::{
  event::{Event, WindowEvent},
  event_loop::{ControlFlow, EventLoopBuilder},
  window::WindowBuilder,
};

fn main() {
  let mut event_loop_builder = EventLoopBuilder::new();

  #[allow(unused_mut)]
  let mut event_loop = event_loop_builder.build();

  let window = WindowBuilder::new().build(&event_loop).unwrap();

  // create new shortcut manager instance
  let mut hotkey_manager = ShortcutManager::new();

  // create our accelerators
  let shortcut_1 = Accelerator::new(SysMods::Shift, KeyCode::ArrowUp);
  let shortcut_2 = Accelerator::new(RawMods::AltCtrlMeta, KeyCode::KeyB);
  // use string parser to generate accelerator (require `std::str::FromStr`)
  let shortcut_3 = Accelerator::from_str("COMMANDORCONTROL+SHIFT+3").unwrap();
  let shortcut_4 = Accelerator::from_str("COMMANDORCONTROL+shIfT+DOWN").unwrap();

  // save a reference to unregister it later
  let global_shortcut_1 = hotkey_manager.register(shortcut_1.clone()).unwrap();

  // register other accelerator's
  hotkey_manager.register(shortcut_2.clone()).unwrap();
  hotkey_manager.register(shortcut_3).unwrap();
  hotkey_manager.register(shortcut_4.clone()).unwrap();

  let menu_channel = global_shortcut_event_receiver();

  event_loop.run(move |event, _, control_flow| {
    *control_flow = ControlFlow::Wait;

    match event {
      Event::WindowEvent {
        event: WindowEvent::CloseRequested,
        ..
      } => *control_flow = ControlFlow::Exit,
      Event::MainEventsCleared => {
        window.request_redraw();
      }
      _ => (),
    }

    if let Ok(event) = menu_channel.try_recv() {
      match event {
        GlobalShortcutEvent(hotkey_id) if hotkey_id == shortcut_1.clone().id() => {
          println!("Pressed `shortcut_1` -- unregister for future use");
          // unregister key
          hotkey_manager
            .unregister(global_shortcut_1.clone())
            .unwrap();
        }
        GlobalShortcutEvent(hotkey_id) if hotkey_id == shortcut_2.clone().id() => {
          println!("Pressed on `shortcut_2`");
        }
        // you can match hotkey_id with accelerator_string only if you used `from_str`
        // by example `shortcut_1` will NOT match AcceleratorId::new("SHIFT+UP") as it's
        // been created with a struct and the ID is generated automatically
        GlobalShortcutEvent(hotkey_id)
          if hotkey_id == AcceleratorId::new("COMMANDORCONTROL+SHIFT+3") =>
        {
          println!("Pressed on `shortcut_3`");
        }
        GlobalShortcutEvent(hotkey_id) if hotkey_id == shortcut_4.clone().id() => {
          println!("Pressed on `shortcut_4`");
        }
        GlobalShortcutEvent(hotkey_id) => {
          println!("hotkey_id {:?}", hotkey_id);
        }
        _ => {}
      }
    }
  })
}
