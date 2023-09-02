mod native;

use std::{thread::{Thread, self}, sync::{atomic::AtomicI32, mpsc}};

use serde::{Serialize, Deserialize};




#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "camelCase")]
pub struct Event {
  pub event_type: EventType,
  pub data: EventData,
  pub time: u64,
  pub mask: u16,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(rename_all = "snake_case")]
pub enum EventType {
  HookEnabled,
  HookDisabled,
  KeyTyped,
  KeyPressed,
  KeyReleased,
  MouseClicked,
  MousePresed,
  MouseReleased,
  MouseMoved,
  MouseDragged,
  MouseWheel,
}

#[derive(Deserialize, Serialize, Debug)]
#[serde(untagged)]
pub enum EventData {
  Keyboard {
    keycode: u16,
    rawcode: u16,
    keychar: char,
  },
  Mouse {
    button: u16,
    clicks: u16,
    x: i16,
    y: i16,
  },
  Wheel {
    x: i16,
    y: i16,
    type_: u8,
    rotation: i16,
    delta: u16,
    direction: u8,
  },
}

pub struct Hook {
  f: Box<dyn FnMut(Event) + Send + Sync + 'static>,
}

impl Hook {
  pub fn new(f: Box<dyn FnMut(Event) + Send + Sync + 'static>) -> Self {
    println!("New!");
    Self { f }
  }

  pub fn run(&self) -> i32 {
    println!("Gotta run");

    let (tx, rx) = mpsc::channel();

    let thread = thread::spawn(move || {
      let status = unsafe { native::hook_run() };
      tx.send(status)
    });

    let status = rx.recv().unwrap();

    println!("Running!");
    println!("{status}");
    status
  }

  pub fn stop(&mut self) {
    println!("DESTROY");
  }
}
