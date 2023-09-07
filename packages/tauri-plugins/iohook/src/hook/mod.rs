mod native;
pub mod event;

use std::{
    os::raw::c_void,
    sync::Arc,
    thread::{self},
};

pub use event::Event;

// use serde::{Deserialize, Serialize};

// #[derive(Deserialize, Serialize, Debug)]
// #[serde(rename_all = "camelCase")]
// pub struct Event {
//     pub event_type: EventType,
//     pub data: EventData,
//     pub time: u64,
//     pub mask: u16,
// }

// impl From<native::uiohook_event> for Event {
//     fn from(value: native::uiohook_event) -> Self {

//     }
// }

// #[derive(Deserialize, Serialize, Debug)]
// #[serde(rename_all = "snake_case")]
// pub enum EventType {
//     HookEnabled,
//     HookDisabled,
//     KeyTyped,
//     KeyPressed,
//     KeyReleased,
//     MouseClicked,
//     MousePressed,
//     MouseReleased,
//     MouseMoved,
//     MouseDragged,
//     MouseWheel,
// }

// #[derive(Deserialize, Serialize, Debug)]
// #[serde(untagged)]
// pub enum EventData {
//     Keyboard {
//         keycode: u16,
//         rawcode: u16,
//         keychar: Option<char>,
//     },
//     Mouse {
//         button: MouseButton,
//         clicks: u16,
//         x: i16,
//         y: i16,
//     },
//     Wheel {
//         x: i16,
//         y: i16,
//         type_: u8,
//         rotation: i16,
//         delta: u16,
//         direction: u8,
//     },
// }
// #[derive(Deserialize, Serialize, Debug)]
// #[serde(rename_all = "snake_case")]
// pub enum MouseButton {
//     Any,
//     Left,
//     Right,
//     Middle,
//     Extra1,
//     Extra2,
// }
// pub enum ErrorKind {
//     Failure,
//     OutOfMemory,
//     // Windows
//     SetWindowsHookEx,
//     GetModuleHandle,
//     // Linux,

//     // macOS
//     AXAPIDisabled,
//     CreateEventPort,
//     CreateRunLoopSource,
//     GetRunLoop,
//     CreateObserver,
// }

// static HOOK_THREAD: Option<Thread> = None;

// static HOOK_RUNNING_MUTEX: Arc<Mutex<()>> = Arc::new(Mutex::new(()));
// static HOOK_CONTROL_MUTEX: Arc<Mutex<()>> = Arc::new(Mutex::new(()));
// static HOOK_CONTROL_CONDITION: Condvar = Condvar::new();

pub fn run(f: Box<dyn FnMut(Event) + Send + Sync + 'static>) -> i32 {
    unsafe extern "C" fn dispatch_proc(event: *mut native::uiohook_event, user_data: *mut c_void) {
        (*(user_data as *mut Box<dyn FnMut(Event) + Send + Sync + 'static>))(Event::from(*event));
    }
    let mut cb = Box::pin(f);
    unsafe {
        native::hook_set_dispatch_proc(
            Some(dispatch_proc),
            &mut *cb as *mut Box<dyn FnMut(Event) + Send + Sync + 'static> as *mut _,
        );
    }

    let status = unsafe { native::hook_run() };
    status
}
pub fn stop() -> i32 {
    let status = unsafe { native::hook_stop() };
    status
}
