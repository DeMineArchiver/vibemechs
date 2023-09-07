use std::thread;

mod hook;
use tauri::{plugin::{TauriPlugin, self}, Runtime, AppHandle, RunEvent, Manager};



pub fn init<R, F>(mut f: F) -> TauriPlugin<R>
  where
    R: Runtime,
    F: FnMut(&AppHandle<R>, hook::Event) + Send + Sync + 'static
      {
  plugin::Builder::new("iohook")
    .setup(|app| {
      let handle = app.clone();

      thread::spawn(move || {
        hook::run(Box::new(move |event| {
          handle.emit_all("iohook://event", event.clone()).unwrap();
          f(&handle, event);
        }));
      });

      Ok(())
    })
    .on_event(|_app, event| {
      if let RunEvent::Exit = event {
        hook::stop();
      }
    })
    .build()
}
