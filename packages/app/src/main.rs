use std::os::windows;

use tauri::{
    generate_context, CustomMenuItem, Manager, RunEvent, SystemTray, SystemTrayEvent,
    SystemTrayMenu, Window, WindowBuilder, GlobalWindowEvent, WindowEvent, SystemTrayMenuItem, SystemTraySubmenu, command, generate_handler,
};
use window_vibrancy::{clear_mica, apply_mica};

mod app;

// #[command(async)]
// fn show_window(window: Window) {
//   window.set_decorations(false).unwrap();
  
//   window.show().unwrap();

//   #[cfg(target_os = "windows")]
//   apply_mica(&window, Some(true)).unwrap();

//   #[cfg(any(target_os = "windows", target_os = "macos"))]
//   set_shadow(&window, true).unwrap();

//   window.set_decorations(true).unwrap();

//   // window.minimize().unwrap();
//   // window.unminimize().unwrap();
//   // window.maximize().unwrap();
//   // window.unmaximize().unwrap();
// }

fn main() {
  let tray = SystemTrayMenu::new()
    .add_submenu(
      SystemTraySubmenu::new(
        "Pack",
        SystemTrayMenu::new()
          .add_item(CustomMenuItem::new("pack_1", "Test Pack"))
      )
    )
    .add_item(CustomMenuItem::new("open_editor", "Editor"))
    .add_native_item(SystemTrayMenuItem::Separator)
    .add_item(CustomMenuItem::new("visibility", "Hide"))
    .add_item(CustomMenuItem::new("quit", "Quit"));

  let app = tauri::Builder::default()
    .invoke_handler(
      generate_handler![

      ]
    )
    .setup(|app| {
      let window = app.get_window("main").unwrap();

      apply_mica(&window, Some(true)).unwrap();
      clear_mica(&window).unwrap();

      Ok(())
    })
    .on_window_event(|event| {
      match event.event() {
        WindowEvent::CloseRequested { api, .. } => {
          api.prevent_close();
          event.window().hide().unwrap();
        },
        _ => {}
      }
    })
    .plugin(tauri_plugin_single_instance::init(|app, argv, cwd| {
      println!("Second instance!\nArgV: {argv:?}\nCWD: {cwd}");
    }))
    .system_tray(SystemTray::new().with_menu(tray).with_tooltip("Vibemechs"))
    .on_system_tray_event(|app, event| {
      // WindowBuilder::from_config(app, app.config().tauri.windows.get(0).unwrap().clone()).build().unwrap();
      let tray = app.tray_handle();
      let window = app.get_window("main").unwrap();

      let visibility_item = tray.get_item("visibility");

      if window.is_visible().unwrap() {
        visibility_item.set_title("Hide").unwrap();
      } else {
        visibility_item.set_title("Show").unwrap();
      }
      match event {
        SystemTrayEvent::LeftClick { .. } => {
          let window = app.get_window("main").unwrap();
          if window.is_visible().unwrap() {
            window.hide().unwrap();
          } else {
            // show_window(window);
            window.show().unwrap();
          }
        }
        SystemTrayEvent::MenuItemClick { id, .. } => {
          let item_handle = app.tray_handle().get_item(&id);
          match id.as_str() {
            "visibility" => {
              let window = app.get_window("main").unwrap();
              if window.is_visible().unwrap() {
                window.hide().unwrap();
              } else {
                window.show().unwrap();
              }
            }
            "quit" => {
              app.exit(0);
            },
            _ => {}
          }
        }
        _ => {}
      };
    })
    .build(generate_context!())
    .expect("Could not build the application!");
  app.run(|app, event| match event {
    // RunEvent::ExitRequested { api, .. } => {
    //   api.prevent_exit();
    // }
    _ => {}
  })
}
