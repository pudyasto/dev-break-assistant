use tauri::{AppHandle, Manager};
use tauri::menu::{MenuBuilder, MenuItemBuilder};
use tauri::tray::{TrayIconBuilder, TrayIconEvent, MouseButton, MouseButtonState};

pub fn setup_tray(app_handle: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let dashboard = MenuItemBuilder::with_id("dashboard", "Open Dashboard").build(app_handle)?;
    let quit = MenuItemBuilder::with_id("quit", "Quit").build(app_handle)?;

    let menu = MenuBuilder::new(app_handle)
        .items(&[&dashboard, &quit])
        .build()?;

    let icon = match app_handle.default_window_icon() {
        Some(icon) => icon.clone(),
        None => {
            let icon_bytes = include_bytes!("../../icons/32x32.png");
            tauri::image::Image::from_bytes(icon_bytes)?
        }
    };

    let _tray = TrayIconBuilder::with_id("main_tray")
        .icon(icon)
        .menu(&menu)
        .show_menu_on_left_click(true)
        .on_menu_event(move |app, event| {
            let event_id = event.id.as_ref();
            tracing::info!("Tray menu event: {}", event_id);
            match event_id {
                "quit" => app.exit(0),
                "dashboard" => {
                    if let Some(window) = app.get_webview_window("main") {
                        let _ = window.unminimize();
                        let _ = window.show();
                        let _ = window.set_focus();
                    } else if let Some(window) = app.webview_windows().values().next() {
                        let _ = window.unminimize();
                        let _ = window.show();
                        let _ = window.set_focus();
                    }
                }

                _ => ()
            }
        })
        .on_tray_icon_event(|tray, event| {
            if let TrayIconEvent::Click { button: MouseButton::Left, button_state: MouseButtonState::Up, .. } = event {
                let app = tray.app_handle();
                if let Some(window) = app.get_webview_window("main") {
                    let _ = window.unminimize();
                    let _ = window.show();
                    let _ = window.set_focus();
                } else if let Some(window) = app.webview_windows().values().next() {
                    let _ = window.unminimize();
                    let _ = window.show();
                    let _ = window.set_focus();
                }
            }
        })
        .build(app_handle)?;

    Ok(())
}
