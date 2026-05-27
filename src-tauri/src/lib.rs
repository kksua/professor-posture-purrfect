use std::sync::{Arc, Mutex};
use std::thread;
use std::time::{Duration, Instant};

use tauri::{
    menu::{CheckMenuItem, Menu, MenuItem, PredefinedMenuItem},
    tray::TrayIconBuilder,
    AppHandle, Emitter, Manager,
};
use tauri_plugin_store::StoreExt;

struct AppState {
    interval_minutes: u64,
    paused: bool,
    last_shown: Instant,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            interval_minutes: 45,
            paused: false,
            last_shown: Instant::now(),
        }
    }
}

type SharedState = Arc<Mutex<AppState>>;

/// Raise the window above the macOS Dock (NSStatusWindowLevel = 25 > Dock level 20).
/// Called every time before show() so the level survives across hide/show cycles.
/// ns_window() is a direct method on WebviewWindow, gated by #[cfg(target_os = "macos")]
/// in the Tauri source — no extra trait import needed.
#[cfg(target_os = "macos")]
fn elevate_window_above_dock(window: &tauri::WebviewWindow) {
    use objc::{msg_send, sel, sel_impl};
    let Ok(ns_win_ptr) = window.ns_window() else { return; };
    let ns_window = ns_win_ptr as *mut objc::runtime::Object;
    unsafe {
        // NSStatusWindowLevel (25) sits above the Dock (20) and menu bar (24).
        // Using 25 keeps us visible above the Dock without covering system alerts.
        let _: () = msg_send![ns_window, setLevel: 25i64];
    }
}

fn show_pet_window(app: &AppHandle) {
    let Some(window) = app.get_webview_window("pet-overlay") else {
        return;
    };

    // Position at bottom-right of the primary monitor, clear of the Dock
    if let Ok(Some(monitor)) = window.primary_monitor() {
        let screen = monitor.size();
        let scale = monitor.scale_factor();
        let win_w = (320.0 * scale) as i32;
        let win_h = (300.0 * scale) as i32;
        // 20 logical px from the right edge; 30 logical px from the bottom edge
        let x = screen.width as i32 - win_w - (20.0 * scale) as i32;
        let y = screen.height as i32 - win_h - (30.0 * scale) as i32;
        let _ = window.set_position(tauri::Position::Physical(tauri::PhysicalPosition { x, y }));
    }

    // Elevate above Dock before showing
    #[cfg(target_os = "macos")]
    elevate_window_above_dock(&window);

    let _ = window.show();
    let _ = window.emit("show-pet", ());

    // Auto-hide after 15 seconds
    let window_clone = window.clone();
    thread::spawn(move || {
        thread::sleep(Duration::from_secs(15));
        let _ = window_clone.hide();
    });
}

fn rebuild_tray_menu(app: &AppHandle, state: &AppState) {
    let interval = state.interval_minutes;
    let paused = state.paused;
    let pause_text = if paused {
        "▶  Resume Reminders"
    } else {
        "⏸  Pause Reminders"
    };

    let Ok(title_item) =
        MenuItem::with_id(app, "app-title", "Professor Posture Purrfect 🐾", false, None::<&str>)
    else {
        return;
    };
    let Ok(sep1) = PredefinedMenuItem::separator(app) else {
        return;
    };
    let Ok(label_item) =
        MenuItem::with_id(app, "remind-label", "Remind me every:", false, None::<&str>)
    else {
        return;
    };
    let Ok(i30) =
        CheckMenuItem::with_id(app, "interval-30", "30 minutes", true, interval == 30, None::<&str>)
    else {
        return;
    };
    let Ok(i45) =
        CheckMenuItem::with_id(app, "interval-45", "45 minutes", true, interval == 45, None::<&str>)
    else {
        return;
    };
    let Ok(i60) =
        CheckMenuItem::with_id(app, "interval-60", "60 minutes", true, interval == 60, None::<&str>)
    else {
        return;
    };
    let Ok(sep2) = PredefinedMenuItem::separator(app) else {
        return;
    };
    let Ok(pause_item) =
        MenuItem::with_id(app, "pause-toggle", pause_text, true, None::<&str>)
    else {
        return;
    };
    let Ok(show_now) =
        MenuItem::with_id(app, "show-now", "🐱  Show Pet Now", true, None::<&str>)
    else {
        return;
    };
    let Ok(sep3) = PredefinedMenuItem::separator(app) else {
        return;
    };
    let Ok(quit) = PredefinedMenuItem::quit(app, Some("Quit")) else {
        return;
    };

    let Ok(menu) = Menu::with_items(
        app,
        &[
            &title_item,
            &sep1,
            &label_item,
            &i30,
            &i45,
            &i60,
            &sep2,
            &pause_item,
            &show_now,
            &sep3,
            &quit,
        ],
    ) else {
        return;
    };

    if let Some(tray) = app.tray_by_id("main-tray") {
        let _ = tray.set_menu(Some(menu));
    }
}

fn save_prefs(app: &AppHandle, state: &AppState) {
    if let Ok(store) = app.store("preferences.json") {
        store.set("interval_minutes", serde_json::json!(state.interval_minutes));
        store.set("paused", serde_json::json!(state.paused));
        let _ = store.save();
    }
}

#[tauri::command]
fn hide_pet(app: AppHandle) {
    if let Some(window) = app.get_webview_window("pet-overlay") {
        let _ = window.hide();
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_store::Builder::new().build())
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![hide_pet])
        .setup(|app| {
            // Hide from Dock — this is a menu bar only app
            #[cfg(target_os = "macos")]
            app.set_activation_policy(tauri::ActivationPolicy::Accessory);

            // Load saved preferences
            let store = app.store("preferences.json")?;
            let saved_interval: u64 = store
                .get("interval_minutes")
                .and_then(|v| v.as_u64())
                .unwrap_or(45);
            let saved_paused: bool = store
                .get("paused")
                .and_then(|v| v.as_bool())
                .unwrap_or(false);
            drop(store);

            let state: SharedState = Arc::new(Mutex::new(AppState {
                interval_minutes: saved_interval,
                paused: saved_paused,
                last_shown: Instant::now(),
            }));

            // Build initial tray menu
            let pause_text = if saved_paused {
                "▶  Resume Reminders"
            } else {
                "⏸  Pause Reminders"
            };

            let menu = Menu::with_items(
                app,
                &[
                    &MenuItem::with_id(
                        app,
                        "app-title",
                        "Professor Posture Purrfect 🐾",
                        false,
                        None::<&str>,
                    )?,
                    &PredefinedMenuItem::separator(app)?,
                    &MenuItem::with_id(
                        app,
                        "remind-label",
                        "Remind me every:",
                        false,
                        None::<&str>,
                    )?,
                    &CheckMenuItem::with_id(
                        app,
                        "interval-30",
                        "30 minutes",
                        true,
                        saved_interval == 30,
                        None::<&str>,
                    )?,
                    &CheckMenuItem::with_id(
                        app,
                        "interval-45",
                        "45 minutes",
                        true,
                        saved_interval == 45,
                        None::<&str>,
                    )?,
                    &CheckMenuItem::with_id(
                        app,
                        "interval-60",
                        "60 minutes",
                        true,
                        saved_interval == 60,
                        None::<&str>,
                    )?,
                    &PredefinedMenuItem::separator(app)?,
                    &MenuItem::with_id(app, "pause-toggle", pause_text, true, None::<&str>)?,
                    &MenuItem::with_id(app, "show-now", "🐱  Show Pet Now", true, None::<&str>)?,
                    &PredefinedMenuItem::separator(app)?,
                    &PredefinedMenuItem::quit(app, Some("Quit"))?,
                ],
            )?;

            // Tray icon — use bundled app icon as placeholder
            let icon = app
                .default_window_icon()
                .cloned()
                .expect("no default window icon");

            TrayIconBuilder::with_id("main-tray")
                .icon(icon)
                .menu(&menu)
                .show_menu_on_left_click(true)
                .on_menu_event({
                    let state = state.clone();
                    move |app, event| {
                        match event.id.as_ref() {
                            "interval-30" | "interval-45" | "interval-60" => {
                                let new_interval: u64 = match event.id.as_ref() {
                                    "interval-30" => 30,
                                    "interval-45" => 45,
                                    _ => 60,
                                };
                                let mut s = state.lock().unwrap();
                                s.interval_minutes = new_interval;
                                s.last_shown = Instant::now(); // reset timer on interval change
                                rebuild_tray_menu(app, &s);
                                save_prefs(app, &s);
                            }
                            "pause-toggle" => {
                                let mut s = state.lock().unwrap();
                                s.paused = !s.paused;
                                rebuild_tray_menu(app, &s);
                                save_prefs(app, &s);
                            }
                            "show-now" => {
                                show_pet_window(app);
                            }
                            _ => {}
                        }
                    }
                })
                .build(app)?;

            // Background timer thread — checks every 10 seconds
            let state_clone = state.clone();
            let app_handle = app.handle().clone();
            thread::spawn(move || loop {
                thread::sleep(Duration::from_secs(10));
                let should_show = {
                    let s = state_clone.lock().unwrap();
                    !s.paused
                        && s.last_shown.elapsed().as_secs() >= s.interval_minutes * 60
                };
                if should_show {
                    {
                        let mut s = state_clone.lock().unwrap();
                        s.last_shown = Instant::now();
                    }
                    show_pet_window(&app_handle);
                }
            });

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
