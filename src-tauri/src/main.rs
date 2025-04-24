// Prevents additional console window on Windows in release, DO NOT REMOVE!!
// #![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![cfg_attr(
    all(not(debug_assertions), target_os = "windows"),
    windows_subsystem = "windows"
)]

use enigo::{Enigo, MouseButton, MouseControllable};
use tauri_plugin_dialog::DialogExt;
use std::sync::atomic::{AtomicBool, Ordering};
use std::sync::Arc;
use std::thread;
use std::time::Duration;
use tauri::{Emitter, Manager, Window};
static CLICKING: AtomicBool = AtomicBool::new(false);

#[tauri::command]
fn start_clicking(x: i32, y: i32, interval_ms: u64, window: Window) {
    if CLICKING.swap(true, Ordering::SeqCst) {
        return; // Already clicking
    }

    let window_clone = window.clone();

    // แสดง dialog เมื่อเริ่มการคลิก


    // Spawn a new thread to handle clicking
    std::thread::spawn(move || {
        let mut enigo = Enigo::new();

        while CLICKING.load(Ordering::SeqCst) {
            // Move mouse to position
            enigo.mouse_move_to(x, y);
            // Perform click
            enigo.mouse_click(MouseButton::Left);

            // Emit event to update status
            if let Err(e) = window_clone.emit_to("main", "clicking-status", true) {
                println!("Failed to emit event: {:?}", e);
            }

            // Sleep for the specified interval
            thread::sleep(Duration::from_millis(interval_ms));
        }

        // Emit event when clicking stops
        if let Err(e) = window_clone.emit_to("main", "clicking-status", false) {
            println!("Failed to emit event: {:?}", e);
        }
    });
}

#[tauri::command]
fn stop_clicking(window: Window) {
    CLICKING.store(false, Ordering::SeqCst);

    // แสดง dialog เมื่อหยุดการคลิก
 
}

#[tauri::command]
fn get_current_mouse_position() -> (i32, i32) {
    let enigo = Enigo::new();
    let pos = enigo.mouse_location();
    (pos.0, pos.1)
}

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .setup(|app| {
            #[cfg(desktop)]
            {
                use tauri_plugin_global_shortcut::{
                    Code, GlobalShortcutExt, Modifiers, Shortcut, ShortcutState,
                };

                let ctrl_n_shortcut = Shortcut::new(Some(Modifiers::CONTROL), Code::KeyN);
                let esc_shortcut = Shortcut::new(None, Code::Escape);

                // เก็บ window reference สำหรับใช้ใน handler
                let main_window = app.get_webview_window("main");

                app.handle().plugin(
                    tauri_plugin_global_shortcut::Builder::new()
                        .with_handler(move |_app, shortcut, event| {
                            println!("{:?}", shortcut);
                            if shortcut == &esc_shortcut {
                                match event.state() {
                                    ShortcutState::Pressed => {
                                        println!("ESC Pressed!");
                                    }
                                    ShortcutState::Released => {
                                        println!("ESC Released!");

                                        // หยุดการคลิกก่อน
                                        CLICKING.store(false, Ordering::SeqCst);

                                        // แสดง dialog ถ้ามี window
                                        if let Some(window) = main_window.clone() {
                                            window
                                                .dialog()
                                                .message("หยุดการคลิกอัตโนมัติแล้ว!")
                                                .title("Auto Clicker")
                                                .show(|result| match result {
                                                    true => {
                                                        println!("Dialog closed");
                                                    }
                                                    false => {
                                                        println!("Dialog not closed");
                                                        // Emit event to update status      
                                                    },
                                                });

                                            // emit event
                                            if let Err(e) =
                                                window.emit_to("main", "clicking-status", false)
                                            {
                                                println!("Failed to emit event: {:?}", e);
                                            }
                                        }
                                    }
                                }
                            }
                        })
                        .build(),
                )?;

                app.global_shortcut().register(esc_shortcut)?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            start_clicking,
            stop_clicking,
            get_current_mouse_position
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
