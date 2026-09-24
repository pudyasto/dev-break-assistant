use tauri::{AppHandle, Manager, Emitter};
use tauri::menu::{MenuBuilder, MenuItemBuilder};
use tauri::tray::{TrayIconBuilder, TrayIconEvent, MouseButton, MouseButtonState};
use crate::state::AppState;
use crate::domain::activity::ActivityState;
use crate::scheduler::monitor::ActivityChangedPayload;

pub fn setup_tray(app_handle: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
    let dashboard = MenuItemBuilder::with_id("dashboard", "Open Dashboard").build(app_handle)?;
    let start_break = MenuItemBuilder::with_id("start_break", "Take Break").build(app_handle)?;
    let pause = MenuItemBuilder::with_id("pause", "Toggle Pause Monitoring").build(app_handle)?;
    let quit = MenuItemBuilder::with_id("quit", "Quit").build(app_handle)?;

    let menu = MenuBuilder::new(app_handle)
        .items(&[&dashboard, &start_break, &pause, &quit])
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
                "pause" => {
                    let app = app.clone();
                    tauri::async_runtime::spawn(async move {
                        let state = app.state::<AppState>();
                        let is_now_paused = {
                            let mut paused = state.monitoring_paused.write().await;
                            *paused = !*paused;
                            *paused
                        };
                        tracing::info!("Monitoring paused toggled to: {}", is_now_paused);

                        let mut activity = state.current_activity.write().await;
                        if is_now_paused {
                            activity.state = ActivityState::Paused;
                        } else {
                            activity.state = ActivityState::Active;
                        }

                        let payload = ActivityChangedPayload {
                            state: activity.state.clone(),
                            idle_seconds: activity.idle_seconds,
                            active_session_seconds: activity.active_session_seconds,
                        };
                        let _ = app.emit("activity://changed", payload);
                    });
                }
                "start_break" => {
                    let app = app.clone();
                    tauri::async_runtime::spawn(async move {
                        let state = app.state::<AppState>();
                        tracing::info!("Started break from tray");

                        let _ = crate::application::break_service::BreakService::record_break_session(
                            &state.db,
                            None,
                            "short",
                            "tray",
                            0,
                            "started"
                        ).await;

                        let mut activity = state.current_activity.write().await;
                        activity.state = ActivityState::Breaking;

                        let payload = ActivityChangedPayload {
                            state: ActivityState::Breaking,
                            idle_seconds: activity.idle_seconds,
                            active_session_seconds: activity.active_session_seconds,
                        };
                        let _ = app.emit("activity://changed", payload);

                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.unminimize();
                            let _ = window.show();
                            let _ = window.set_focus();
                        }
                    });
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
