mod memory;
pub mod offsets;
mod sdk;
mod scripthub;

use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

#[derive(Serialize, Deserialize, Debug)]
pub struct ScriptItem {
    pub name: String,
    pub content: String,
}

#[derive(Serialize, Debug)]
pub struct OffsetsStatus {
    pub version: String,
    pub source: String,
    pub offset_count: usize,
    pub fflag_count: usize,
}

fn get_scripts_dir_path() -> Result<PathBuf, String> {
    let exe = std::env::current_exe().map_err(|e| format!("cannot get exe path: {e}"))?;
    let dir = exe.parent().ok_or("cannot get exe directory")?.join("scripts");
    if !dir.exists() {
        fs::create_dir_all(&dir)
            .map_err(|e| format!("cannot create scripts folder {}: {e}", dir.display()))?;
    }
    Ok(dir)
}

fn sanitize_filename(name: &str) -> String {
    let mut out = name.trim().to_string();
    if out.is_empty() {
        out = "Script.lua".into();
    }
    out = out
        .replace(['<', '>', ':', '"', '/', '\\', '|', '?', '*'], "_");
    if out.len() > 100 {
        out.truncate(100);
    }
    if !out.ends_with(".lua") && !out.ends_with(".txt") {
        out.push_str(".lua");
    }
    out
}

#[tauri::command]
fn list_saved_scripts() -> Result<Vec<ScriptItem>, String> {
    let dir = match get_scripts_dir_path() {
        Ok(p) => p,
        Err(_) => return Ok(Vec::new()),
    };
    let mut items = Vec::new();
    let entries = match fs::read_dir(&dir) {
        Ok(e) => e,
        Err(_) => return Ok(Vec::new()),
    };
    for entry in entries.flatten() {
        let path = entry.path();
        if !path.is_file() {
            continue;
        }
        let ext = path
            .extension()
            .and_then(|e| e.to_str())
            .unwrap_or("")
            .to_ascii_lowercase();
        if ext != "lua" && ext != "txt" {
            continue;
        }
        let name = entry.file_name().to_string_lossy().to_string();
        if name.len() > 260 {
            continue;
        }
        let content = fs::read_to_string(&path).unwrap_or_default();
        if content.len() > 2_000_000 {
            continue;
        }
        items.push(ScriptItem { name, content });
    }
    items.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(items)
}

#[tauri::command]
fn save_script(name: String, content: String) -> Result<(), String> {
    if name.len() > 260 {
        return Err("filename too long".into());
    }
    let dir = get_scripts_dir_path()?;
    let file = dir.join(sanitize_filename(&name));
    if file.parent().is_some_and(|p| !p.starts_with(&dir)) {
        return Err("invalid path".into());
    }
    let tmp = file.with_extension("tmp");
    fs::write(&tmp, &content).map_err(|e| format!("write failed: {e}"))?;
    fs::rename(&tmp, &file).map_err(|e| format!("save failed: {e}"))?;
    Ok(())
}

#[tauri::command]
fn delete_script(name: String) -> Result<(), String> {
    if name.contains("..") || name.contains('/') || name.contains('\\') {
        return Err("invalid name".into());
    }
    let dir = get_scripts_dir_path()?;
    let file = dir.join(&name);
    if !file.starts_with(&dir) {
        return Err("invalid path".into());
    }
    if file.is_file() {
        fs::remove_file(&file).map_err(|e| format!("delete failed: {e}"))?;
    }
    Ok(())
}

#[tauri::command]
fn http_get(url: String) -> Result<String, String> {
    if url.len() > 2048 {
        return Err("url too long".into());
    }
    if !(url.starts_with("http://") || url.starts_with("https://")) {
        return Err("invalid url scheme".into());
    }
    let body = reqwest::blocking::Client::builder()
        .user_agent("Mozilla/5.0 (Windows NT 10.0; Win64; x64) AppleWebKit/537.36")
        .timeout(std::time::Duration::from_secs(10))
        .build()
        .map_err(|e| e.to_string())?
        .get(&url)
        .header("Accept", "application/json")
        .send()
        .map_err(|e| format!("request failed: {e}"))?
        .error_for_status()
        .map_err(|e| format!("http {e}"))?
        .text()
        .map_err(|e| e.to_string())?;
    if body.len() > 5_000_000 {
        return Err("response too large".into());
    }
    Ok(body)
}

#[tauri::command]
fn open_url(url: String) -> Result<(), String> {
    if url.len() > 2048 {
        return Err("url too long".into());
    }
    if !(url.starts_with("http://") || url.starts_with("https://")) {
        return Err("invalid url".into());
    }
    #[cfg(target_os = "windows")]
    std::process::Command::new("cmd")
        .args(["/C", "start", "", &url])
        .spawn()
        .map_err(|e| format!("open failed: {e}"))?;
    #[cfg(not(target_os = "windows"))]
    std::process::Command::new("open")
        .arg(&url)
        .spawn()
        .map_err(|e| e.to_string())?;
    Ok(())
}

#[tauri::command]
fn inject() -> Result<String, String> {
    let session = sdk::Roblox::new()?;
    if !session.process().is_alive() {
        return Err("roblox closed".into());
    }
    let address = session.datamodel_address();
    if !memory::Process::is_valid_address(address) {
        return Err("resolved datamodel is invalid".into());
    }
    let line = format!("datamodel: 0x{address:X}");
    println!("{line}");
    Ok(line)
}

#[tauri::command]
fn get_game_id() -> Result<u64, String> {
    let session = sdk::Roblox::new().map_err(|_| "no valid datamodel".to_string())?;
    let game_id = session
        .datamodel()
        .get_game_id()
        .map_err(|_| "no valid game id".to_string())?;
    if !scripthub::is_valid_game_id(game_id) {
        return Err("no valid game id".into());
    }
    Ok(game_id)
}

#[tauri::command]
fn get_offset(path: String) -> Result<u64, String> {
    if path.len() > 128 {
        return Err("offset path too long".into());
    }
    offsets::get(&path).map_err(String::from)
}

#[tauri::command]
fn get_fflag(name: String) -> Result<u64, String> {
    if name.len() > 128 {
        return Err("fflag name too long".into());
    }
    offsets::fflag(&name).map_err(String::from)
}

#[tauri::command]
fn get_offsets_status() -> OffsetsStatus {
    let _ = offsets::ensure_loaded();
    OffsetsStatus {
        version: offsets::version().unwrap_or_default(),
        source: offsets::SOURCE_URL.to_string(),
        offset_count: offsets::offset_count(),
        fflag_count: offsets::fflag_count(),
    }
}

#[tauri::command]
fn refresh_offsets() -> Result<String, String> {
    offsets::refresh()?;
    Ok(format!(
        "{} ({} offsets, {} fflags)",
        offsets::version().unwrap_or_else(|| "unknown".into()),
        offsets::offset_count(),
        offsets::fflag_count()
    ))
}

#[tauri::command]
fn roblox_open() -> bool {
    memory::Process::open_roblox()
        .map(|p| p.is_alive())
        .unwrap_or(false)
}

#[tauri::command]
fn get_status() -> String {
    if roblox_open() {
        "ready".into()
    } else {
        "not_found".into()
    }
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let _ = get_scripts_dir_path();
    std::thread::spawn(|| {
        let _ = offsets::ensure_loaded();
    });
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            http_get,
            open_url,
            list_saved_scripts,
            save_script,
            delete_script,
            inject,
            get_game_id,
            get_offset,
            get_fflag,
            get_offsets_status,
            refresh_offsets,
            roblox_open,
            get_status
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
