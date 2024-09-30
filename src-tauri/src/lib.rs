// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
#[derive(serde::Deserialize)]
struct Config {
    mouse: u8,
    interval: u64,
}
static mut RUNNING: bool = false;
static mut CONFIG: Config = Config {
    mouse: 0,
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

use rdev::{simulate, Button, EventType, Key};
fn simulate_mouse_click(button: Button) {
    // 按下鼠标按钮
    if let Err(error) = simulate(&EventType::ButtonPress(button)) {
        eprintln!("Error: {:?}", error);
    }
    // 释放鼠标按钮
    if let Err(error) = simulate(&EventType::ButtonRelease(button)) {
        eprintln!("Error: {:?}", error);
    }
}
fn simulate_keyboard_press(key: Key) {
    // 按下键盘按钮
    if let Err(error) = simulate(&EventType::KeyPress(key)) {
        eprintln!("Error: {:?}", error);
    }
    // 释放键盘按钮
    if let Err(error) = simulate(&EventType::KeyRelease(key)) {
        eprintln!("Error: {:?}", error);
    }
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
