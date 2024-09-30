// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[derive(serde::Deserialize)]
struct Config {
    mouse: u8,
    keyboard: u32,
    interval: u64,
}
static mut RUNNING: bool = false;
static mut CONFIG: Config = Config {
    mouse: 0,
    keyboard: 0,
    interval: 100,
};
#[tauri::command]
fn setup(config: Config) {
    stop();
    unsafe {
        CONFIG = config;
    }
}

fn num_to_button(num: u8) -> Result<Button, &'static str> {
    match num {
        1 => Ok(Button::Left),
        2 => Ok(Button::Right),
        3 => Ok(Button::Middle),
        _ => Ok(Button::Unknown(0)),
    }
}
fn num_to_key(num: u32) -> Result<Key, &'static str> {
    let key_map = [
        (1, Key::Num1),
        (2, Key::Num2),
        (3, Key::Num3),
        (4, Key::Num4),
        (5, Key::Num5),
        (6, Key::Num6),
        (7, Key::Num7),
        (8, Key::Num8),
        (9, Key::Num9),
        (10, Key::Num0),
        (11, Key::KeyA),
        (12, Key::KeyB),
        (13, Key::KeyC),
        (14, Key::KeyD),
        (15, Key::KeyE),
        (16, Key::KeyF),
        (17, Key::KeyG),
        (18, Key::KeyH),
        (19, Key::KeyI),
        (20, Key::KeyJ),
        (21, Key::KeyK),
        (22, Key::KeyL),
        (23, Key::KeyM),
        (24, Key::KeyN),
        (25, Key::KeyO),
        (26, Key::KeyP),
        (27, Key::KeyQ),
        (28, Key::KeyR),
        (29, Key::KeyS),
        (30, Key::KeyT),
        (31, Key::KeyU),
        (32, Key::KeyV),
        (33, Key::KeyW),
        (34, Key::KeyX),
        (35, Key::KeyY),
        (36, Key::KeyZ),
        (37, Key::Kp1),
        (38, Key::Kp2),
        (39, Key::Kp3),
        (40, Key::Kp4),
        (41, Key::Kp5),
        (42, Key::Kp6),
        (43, Key::Kp7),
        (44, Key::Kp8),
        (45, Key::Kp9),
        (46, Key::Kp0),
    ];

    match key_map.iter().find(|&&(k, _)| k == num) {
        Some(&(_, key)) => Ok(key),
        None => Ok(Key::Unknown(0)),
    }
}
fn mouse() {
    unsafe {
        match num_to_button(CONFIG.mouse) {
            Ok(button) => {
                if button == Button::Unknown(0) {
                    return;
                }
                simulate_mouse_click(button)
            }
            Err(err) => eprintln!("Error: {}", err),
        }
    }
}
fn keyboard() {
    unsafe {
        match num_to_key(CONFIG.keyboard) {
            Ok(key) => {
                if key == Key::Unknown(0) {
                    return;
                }
                simulate_keyboard_press(key)
            }
            Err(err) => eprintln!("Error: {}", err),
        }
    }
}

#[tauri::command]
async fn start() {
    unsafe {
        RUNNING = true;
    }
    loop {
        unsafe {
            if !RUNNING {
                break;
            }
            mouse();
            keyboard();
            tokio::time::sleep(tokio::time::Duration::from_millis(CONFIG.interval)).await;
        }
    }
}

#[tauri::command]
fn stop() {
    unsafe {
        RUNNING = false;
    }
}

use core::time;
use std::thread;

use rdev::{simulate, Button, EventType, Key};
fn send(event_type: &EventType) {
    let delay = time::Duration::from_millis(20);
    match simulate(event_type) {
        Ok(()) => (),
        Err(error) => {
            println!("We could not send {:?}: {:?}", event_type, error);
        }
    }
    // Let ths OS catchup (at least MacOS)
    thread::sleep(delay);
}
fn simulate_mouse_click(button: Button) {
    // 按下鼠标按钮
    send(&EventType::ButtonPress(button));
    // 释放鼠标按钮
    send(&EventType::ButtonRelease(button));
}
fn simulate_keyboard_press(key: Key) {
    // 按下键盘按钮
    send(&EventType::KeyPress(key));
    // 释放键盘按钮
    send(&EventType::KeyRelease(key));
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .invoke_handler(tauri::generate_handler![setup, start, stop])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
