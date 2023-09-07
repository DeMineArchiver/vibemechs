use tauri::{Runtime, plugin::{Builder, TauriPlugin}};

mod state;

pub const PLUGIN_NAME: &str = "app";

pub fn init<R: Runtime>() -> TauriPlugin<R> {
  Builder::new(PLUGIN_NAME)
    .js_init_script("".into())
    .setup(|app| {
      
      Ok(())
    })
    .build()
}
