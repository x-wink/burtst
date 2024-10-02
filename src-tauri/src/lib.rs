// Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
use rdev::{listen, simulate, Button, EventType, Key};
#[derive(serde::Deserialize)]
struct Config {
    mouse: u8,
    keyboard: u32,
    interval: u64,
    start: u32,
    stop: u32,
}
static mut RUNNING: bool = false;
static mut CONFIG: Config = Config {
    mouse: 0,
    keyboard: 0,
    interval: 100,
    start: 9,
    stop: 10,
};
static UNKNOW_BUTTON: Button = Button::Unknown(0);
static UNKNOW_KEY: Key = Key::Unknown(0);

fn num2button(num: u8) -> Button {
    let button_map = [(1, Button::Left), (2, Button::Right), (3, Button::Middle)];
    button_map
        .iter()
        .find(|&&(k, _)| k == num)
        .map_or(UNKNOW_BUTTON, |&(_, button)| button)
}
fn num2key(num: u32) -> Key {
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

    key_map
        .iter()
        .find(|&&(k, _)| k == num)
        .map_or(UNKNOW_KEY, |&(_, key)| key)
}
fn send(event_type: &EventType) {
    let delay = core::time::Duration::from_millis(20);
    match simulate(event_type) {
        Ok(()) => (),
        Err(error) => {
            stop();
            println!("{:?}按键失败: {:?}", event_type, error.to_string());
        }
    }
    // Let ths OS catchup (at least MacOS)
    std::thread::sleep(delay);
}
fn click(button: Button) {
    send(&EventType::ButtonPress(button));
    send(&EventType::ButtonRelease(button));
}
fn press(key: Key) {
    send(&EventType::KeyPress(key));
    send(&EventType::KeyRelease(key));
}
fn hotkey() {
    // FIXME 捕获到键盘事件就会导致闪退
    std::thread::spawn(|| {
        if let Err(error) = listen(|event| unsafe {
            let event_type = event.event_type;
            let start_key = num2key(CONFIG.start);
            let stop_key = num2key(CONFIG.stop);
            if start_key != UNKNOW_KEY && event_type == EventType::KeyPress(start_key) {
                println!("热键开始");
                start();
            } else if start_key != UNKNOW_KEY && event_type == EventType::KeyPress(stop_key) {
                println!("热键结束");
                stop();
            }
        }) {
            println!("监听热键失败: {:?}", error);
        }
    });
}

#[tauri::command]
fn setup(config: Config) {
    unsafe {
        CONFIG = config;
    }
}

#[tauri::command]
async fn start() {
    println!("start");
    unsafe {
        RUNNING = true;
        loop {
            if !RUNNING {
                break;
            }
            let button = num2button(CONFIG.mouse);
            if button != UNKNOW_BUTTON {
                click(button)
            }
            let key = num2key(CONFIG.keyboard);
            if key != UNKNOW_KEY {
                press(key)
            }
            tokio::time::sleep(tokio::time::Duration::from_millis(CONFIG.interval)).await;
        }
    }
}

#[tauri::command]
fn stop() {
    print!("stop");
    unsafe {
        RUNNING = false;
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // hotkey();
    tauri::Builder::default()
        .plugin(tauri_plugin_process::init())
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_updater::Builder::new().build())
        .invoke_handler(tauri::generate_handler![setup, start, stop])
        .run(tauri::generate_context!())
        .expect("启动失败");
}
