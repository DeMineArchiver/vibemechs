use std::sync::Mutex;

mod hook;

use hook::Hook;
use tauri::{plugin::{TauriPlugin, self}, Runtime, AppHandle, Manager, RunEvent};



pub fn init<R, F>(mut f: F) -> TauriPlugin<R>
  where
    R: Runtime,
    F: FnMut(&AppHandle<R>, hook::Event) + Send + Sync + 'static
      {
  plugin::Builder::new("iohook")
    .setup(|app| {
      let handle = app.clone();

      let hook = Hook::new(
        Box::new(move |event| {
          f(&handle, event);
        })
      );
      hook.run();

      app.manage(
        Mutex::new(hook)
      );
      Ok(())
    })
    .on_event(|app, event| {
      if let RunEvent::Exit = event {
        let hook = app.state::<Mutex<Hook>>();
        hook.lock().unwrap().stop();
      }
    })
    .build()
}
