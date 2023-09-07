use serde::{Deserialize, Serialize};
use super::native;


#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(tag = "type", rename_all = "snake_case")]
pub enum Event {
  Status {
    status: Status,
    time: u64,
  },
  Keyboard {
    event: KeyboardEvent,
    time: u64,
    mask: u16,
    keycode: u16,
    rawcode: u16,
    keychar: Option<char>,
  },
  Mouse {
    event: MouseEvent,
    time: u64,
    mask: u16,
    button: MouseButton,
    clicks: u16,
    x: i16,
    y: i16,
  },
  MouseWheel {
    x: i16,
    y: i16,
    // pub type: u8,
    rotation: i16,
    delta: u16,
    direction: u8,
  },
  Unknown
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum KeyboardEvent {
  Typed,
  Pressed,
  Released,
  Unknown(i32)
}

impl From<native::event_type> for KeyboardEvent {
  fn from(value: native::event_type) -> Self {
    match value {
      native::_event_type_EVENT_KEY_TYPED => Self::Typed,
      native::_event_type_EVENT_KEY_PRESSED => Self::Pressed,
      native::_event_type_EVENT_KEY_RELEASED => Self::Released,
      _ => Self::Unknown(value)
    }
  }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum MouseEvent {
  Clicked,
  Pressed,
  Released,
  Moved,
  Dragged,
  Unknown(i32)
}

impl From<native::event_type> for MouseEvent {
  fn from(value: native::event_type) -> Self {
    match value {
      native::_event_type_EVENT_MOUSE_CLICKED => Self::Clicked,
      native::_event_type_EVENT_MOUSE_PRESSED => Self::Pressed,
      native::_event_type_EVENT_MOUSE_RELEASED => Self::Released,
      native::_event_type_EVENT_MOUSE_MOVED => Self::Moved,
      native::_event_type_EVENT_MOUSE_DRAGGED => Self::Dragged,
      _ => Self::Unknown(value)
    }
  }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum MouseButton {
  Right,
  Left,
  Middle,
  Extra1,
  Extra2,
  Unknown(u16)
}

impl From<u16> for MouseButton {
    fn from(value: u16) -> Self {
      match value as u32 {
        native::MOUSE_BUTTON1 => Self::Left,
        native::MOUSE_BUTTON2 => Self::Right,
        native::MOUSE_BUTTON3 => Self::Middle,
        native::MOUSE_BUTTON4 => Self::Extra1,
        native::MOUSE_BUTTON5 => Self::Extra1,
        _ => Self::Unknown(value)
      }
    }
}

#[derive(Deserialize, Serialize, Clone, Debug)]
#[serde(rename_all = "snake_case")]
pub enum Status {
  Enabled,
  Disabled,
  Unknown(i32)
}
impl From<native::event_type> for Status {
  fn from(value: native::event_type) -> Self {
    match value {
      native::_event_type_EVENT_HOOK_ENABLED => Self::Enabled,
      native::_event_type_EVENT_HOOK_DISABLED => Self::Disabled,
      _ => Self::Unknown(value)
    }
  }
}


impl From<native::uiohook_event> for Event {
  fn from(value: native::uiohook_event) -> Self {
    match value.type_ {
      status @ (
        native::_event_type_EVENT_HOOK_ENABLED
        | native::_event_type_EVENT_HOOK_DISABLED
      ) => Self::Status { status: Status::from(status), time: value.time },
      event @ (
        native::_event_type_EVENT_KEY_TYPED
        | native::_event_type_EVENT_KEY_PRESSED
        | native::_event_type_EVENT_KEY_RELEASED
      ) => Self::Keyboard {
        event: KeyboardEvent::from(event),
        time: value.time,
        mask: value.mask,
        keycode: unsafe { value.data.keyboard.keycode },
        rawcode: unsafe { value.data.keyboard.rawcode },
        keychar: char::from_u32(unsafe { value.data.keyboard.keychar } as u32)
      },
      event @ (
        native::_event_type_EVENT_MOUSE_CLICKED 
        | native::_event_type_EVENT_MOUSE_PRESSED 
        | native::_event_type_EVENT_MOUSE_RELEASED
        | native::_event_type_EVENT_MOUSE_MOVED
        | native::_event_type_EVENT_MOUSE_DRAGGED 
      ) => Self::Mouse {
        event: MouseEvent::from(event),
        time: value.time,
        mask: value.mask,
        button: MouseButton::from(unsafe { value.data.mouse.button }),
        clicks: unsafe { value.data.mouse.clicks },
        x: unsafe { value.data.mouse.x },
        y: unsafe { value.data.mouse.y }
      },
      native::_event_type_EVENT_MOUSE_WHEEL => Self::MouseWheel {
        x: unsafe { value.data.wheel.x },
        y: unsafe { value.data.wheel.y },
        rotation: unsafe { value.data.wheel.rotation },
        delta: unsafe { value.data.wheel.delta },
        direction: unsafe { value.data.wheel.direction },
      },
      _ => Self::Unknown
    }
  }
}
