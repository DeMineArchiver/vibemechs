fn main() {
  tauri::Builder::default()
    .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
      
    }))
    .plugin(tauri_plugin_iohook::init(|app, event| {
      println!("PRESSED!!!");
    }))
    .run(tauri::generate_context!())
    .expect("(Un)expected error occurred!");
}
