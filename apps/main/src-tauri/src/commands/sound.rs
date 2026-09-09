//! 自选提示音的文件管理：导入（复制进应用数据目录）、读取字节、删除。
//!
//! 选中的音频一律复制到 `{app_data_dir}/sounds/`，settings.json 只存文件名——
//! 否则用户之后移动或删除源文件，提示音会静默失效且无从提示。

use std::path::{Path, PathBuf};
use tauri::{AppHandle, Manager};
use tracing::info;

/// 单个提示音上限 5 MB：文件要整份经 IPC 送进 WebView 播放，
/// 再大只会拖慢首次播放，而提示音本就该是短音效。
const SOUND_MAX_BYTES: u64 = 5 * 1024 * 1024;

/// WebView2（Chromium 内核）能直接解码的容器，列表之外的文件即使复制进来也放不出声，
/// 与其让用户选完才发现没声音，不如在导入这一步就拒掉。
const ALLOWED_EXTENSIONS: &[&str] = &[
    "mp3", "wav", "ogg", "oga", "opus", "m4a", "aac", "flac", "weba", "webm",
];

/// 净化后的文件名主干最长字符数，避免用户拿超长文件名撑爆路径长度限制。
const MAX_STEM_CHARS: usize = 48;

pub(crate) fn sounds_dir(app: &AppHandle) -> Result<PathBuf, String> {
    app.path()
        .app_data_dir()
        .map(|p| p.join("sounds"))
        .map_err(|e| format!("无法获取应用数据目录: {e}"))
}

/// 把前端传来的名字限制为 sounds/ 下的单层文件名，杜绝 `../` 与绝对路径穿越。
fn resolve_sound_path(app: &AppHandle, name: &str) -> Result<PathBuf, String> {
    let is_plain_file_name = Path::new(name)
        .file_name()
        .and_then(|f| f.to_str())
        .is_some_and(|f| f == name);
    if name.is_empty() || !is_plain_file_name {
        return Err(format!("非法的提示音文件名：{name}"));
    }
    Ok(sounds_dir(app)?.join(name))
}

/// 剔除各平台文件名非法字符并截断长度；结果为空时退回固定名。
fn sanitize_stem(stem: &str) -> String {
    let cleaned: String = stem
        .chars()
        .filter(|c| {
            !c.is_control() && !matches!(c, '/' | '\\' | ':' | '*' | '?' | '"' | '<' | '>' | '|')
        })
        .take(MAX_STEM_CHARS)
        .collect();
    let trimmed = cleaned.trim().trim_matches('.').trim();
    if trimmed.is_empty() {
        "提示音".to_string()
    } else {
        trimmed.to_string()
    }
}

/// 同名文件不覆盖而是加序号：两个不同目录下的同名音效都可能被用户选进来。
fn pick_unique_file_name(dir: &Path, stem: &str, ext: &str) -> (String, PathBuf) {
    let mut name = format!("{stem}.{ext}");
    let mut n = 1u32;
    while dir.join(&name).exists() {
        n += 1;
        name = format!("{stem} ({n}).{ext}");
    }
    let path = dir.join(&name);
    (name, path)
}

/// 复制用户选中的音频到 sounds/，返回存储用的文件名。
#[tauri::command]
pub fn import_sound_file(app: AppHandle, path: String) -> Result<String, String> {
    let src = Path::new(&path);
    if !src.is_file() {
        return Err(format!("文件不存在：{path}"));
    }
    let ext = src
        .extension()
        .and_then(|e| e.to_str())
        .map(|e| e.to_ascii_lowercase())
        .unwrap_or_default();
    if !ALLOWED_EXTENSIONS.contains(&ext.as_str()) {
        return Err(format!(
            "不支持的音频格式：{}",
            if ext.is_empty() {
                "无扩展名".into()
            } else {
                format!(".{ext}")
            }
        ));
    }
    let size = src
        .metadata()
        .map_err(|e| format!("读取文件信息失败：{e}"))?
        .len();
    if size > SOUND_MAX_BYTES {
        return Err(format!("文件过大（{size} 字节），超过 5 MB 上限"));
    }

    let dir = sounds_dir(&app)?;
    std::fs::create_dir_all(&dir).map_err(|e| format!("无法创建提示音目录: {e}"))?;
    let stem = sanitize_stem(
        &src.file_stem()
            .map(|s| s.to_string_lossy().to_string())
            .unwrap_or_default(),
    );
    let (name, dest) = pick_unique_file_name(&dir, &stem, &ext);
    std::fs::copy(src, &dest).map_err(|e| format!("复制音频失败：{e}"))?;
    info!("已导入提示音「{name}」");
    Ok(name)
}

/// 读出音频原始字节交给前端建 Blob URL 播放。
#[tauri::command]
pub fn read_sound_file(app: AppHandle, name: String) -> Result<tauri::ipc::Response, String> {
    let path = resolve_sound_path(&app, &name)?;
    let bytes = std::fs::read(&path).map_err(|e| format!("读取提示音失败：{e}"))?;
    Ok(tauri::ipc::Response::new(bytes))
}

/// 删除不再被任何时机引用的提示音。文件已不在视为成功，保证前端可以无脑调用。
#[tauri::command]
pub fn delete_sound_file(app: AppHandle, name: String) -> Result<(), String> {
    let path = resolve_sound_path(&app, &name)?;
    if !path.exists() {
        return Ok(());
    }
    std::fs::remove_file(&path).map_err(|e| format!("删除提示音失败：{e}"))?;
    info!("已删除提示音「{name}」");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn sanitize_stem_strips_illegal_chars() {
        assert_eq!(sanitize_stem("a/b\\c:d*e?f\"g<h>i|j"), "abcdefghij");
        assert_eq!(sanitize_stem("  开火音效  "), "开火音效");
    }

    #[test]
    fn sanitize_stem_falls_back_when_empty() {
        assert_eq!(sanitize_stem(""), "提示音");
        assert_eq!(sanitize_stem("..."), "提示音");
        assert_eq!(sanitize_stem("///"), "提示音");
    }

    #[test]
    fn sanitize_stem_truncates_long_name() {
        let long = "字".repeat(200);
        assert_eq!(sanitize_stem(&long).chars().count(), MAX_STEM_CHARS);
    }

    #[test]
    fn pick_unique_file_name_appends_index() {
        let dir = tempfile::tempdir().unwrap();
        let (first, path) = pick_unique_file_name(dir.path(), "shot", "mp3");
        assert_eq!(first, "shot.mp3");
        std::fs::write(&path, b"x").unwrap();

        let (second, path2) = pick_unique_file_name(dir.path(), "shot", "mp3");
        assert_eq!(second, "shot (2).mp3");
        std::fs::write(&path2, b"x").unwrap();

        let (third, _) = pick_unique_file_name(dir.path(), "shot", "mp3");
        assert_eq!(third, "shot (3).mp3");
    }
}
