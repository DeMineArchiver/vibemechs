use tauri::Runtime;

pub struct AppState {
  window_visible: bool,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
          window_visible: true
        }
    }
}
