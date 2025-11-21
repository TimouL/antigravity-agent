use std::fs;
use std::path::{Path, PathBuf};
use std::process::Command;

use serde::{Deserialize, Serialize};
use sysinfo::System;

use crate::constants::paths;

/// 获取Antigravity应用数据目录（跨平台）
pub fn get_antigravity_data_dir() -> Option<PathBuf> {
    match std::env::consts::OS {
        "windows" => {
            // Windows: %APPDATA%\Antigravity\User\globalStorage\
            dirs::config_dir()
                .map(|path| path.join("Antigravity").join("User").join("globalStorage"))
        }
        "macos" => {
            // macOS: 基于 product.json 中的 dataFolderName: ".antigravity" 配置
            // ~/Library/Application Support/Antigravity/User/globalStorage/
            dirs::data_dir().map(|path| path.join("Antigravity").join("User").join("globalStorage"))
        }
        "linux" => {
            // Linux: 基于 product.json 中的 dataFolderName: ".antigravity" 配置
            // 优先使用 ~/.config/Antigravity/User/globalStorage/，备用 ~/.local/share/Antigravity/User/globalStorage/
            dirs::config_dir() // 优先：~/.config
                .map(|path| path.join("Antigravity").join("User").join("globalStorage"))
                .or_else(|| {
                    // 备用：~/.local/share
                    dirs::data_dir()
                        .map(|path| path.join("Antigravity").join("User").join("globalStorage"))
                })
        }
        _ => {
            // 其他系统：尝试使用数据目录
            dirs::data_dir().map(|path| path.join("Antigravity").join("User").join("globalStorage"))
        }
    }
}

fn ensure_config_dir() -> Result<PathBuf, String> {
    let config_dir = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(paths::CONFIG_DIR_NAME);

    fs::create_dir_all(&config_dir).map_err(|e| format!("创建配置目录失败: {e}"))?;

    Ok(config_dir)
}

fn config_file_path() -> Result<PathBuf, String> {
    Ok(ensure_config_dir()?.join("config.json"))
}

#[derive(Debug, Serialize, Deserialize, Default)]
struct AgentConfig {
    #[serde(rename = "antigravityPath")]
    antigravity_path: Option<String>,
}

fn load_agent_config() -> Result<AgentConfig, String> {
    let path = config_file_path()?;
    if !path.exists() {
        return Ok(AgentConfig::default());
    }

    let content = fs::read_to_string(&path).map_err(|e| format!("读取配置失败: {e}"))?;

    serde_json::from_str(&content).map_err(|e| format!("解析配置失败: {e}"))
}

fn save_agent_config(config: &AgentConfig) -> Result<(), String> {
    let path = config_file_path()?;
    let content =
        serde_json::to_string_pretty(config).map_err(|e| format!("序列化配置失败: {e}"))?;
    fs::write(&path, content).map_err(|e| format!("写入配置失败: {e}"))
}

fn validate_antigravity_exe(path: &Path) -> bool {
    path.is_file()
}

fn load_persisted_antigravity_path() -> Option<PathBuf> {
    if !cfg!(windows) {
        return None;
    }

    load_agent_config().ok().and_then(|cfg| {
        cfg.antigravity_path
            .as_deref()
            .map(PathBuf::from)
            .filter(|p| validate_antigravity_exe(p))
    })
}

pub fn persist_antigravity_path(path: &Path) -> Result<(), String> {
    if !cfg!(windows) {
        return Ok(());
    }

    if !validate_antigravity_exe(path) {
        return Err("无效的 Antigravity 可执行文件路径".to_string());
    }

    let mut config = load_agent_config().unwrap_or_default();
    config.antigravity_path = Some(path.to_string_lossy().to_string());
    save_agent_config(&config)
}

/// 获取Antigravity状态数据库文件路径
pub fn get_antigravity_db_path() -> Option<PathBuf> {
    get_antigravity_data_dir().map(|dir| dir.join("state.vscdb"))
}

/// 检查Antigravity是否安装并运行
pub fn is_antigravity_available() -> bool {
    get_antigravity_db_path()
        .map(|path| path.exists())
        .unwrap_or(false)
}

/// 搜索可能的Antigravity安装位置
pub fn find_antigravity_installations() -> Vec<PathBuf> {
    let mut possible_paths = Vec::new();

    // 用户数据目录
    if let Some(user_data) = dirs::data_dir() {
        possible_paths.push(user_data.join("Antigravity"));
    }

    // 配置目录
    if let Some(config_dir) = dirs::config_dir() {
        possible_paths.push(config_dir.join("Antigravity"));
    }

    possible_paths
}

/// 获取 Windows 平台下 Antigravity 的可能安装路径
fn get_antigravity_windows_paths() -> Vec<PathBuf> {
    let mut antigravity_paths = Vec::new();

    // 基于用户主目录构建可能的路径
    if let Some(home) = dirs::home_dir() {
        // C:\Users\{用户名}\AppData\Local\Programs\Antigravity\Antigravity.exe (最常见)
        antigravity_paths.push(home.join(r"AppData\Local\Programs\Antigravity\Antigravity.exe"));
        // C:\Users\{用户名}\AppData\Roaming\Local\Programs\Antigravity\Antigravity.exe
        antigravity_paths
            .push(home.join(r"AppData\Roaming\Local\Programs\Antigravity\Antigravity.exe"));
    }

    // 使用 data_local_dir (通常是 C:\Users\{用户名}\AppData\Local)
    if let Some(local_data) = dirs::data_local_dir() {
        antigravity_paths.push(local_data.join(r"Programs\Antigravity\Antigravity.exe"));
    }

    // 其他可能的位置
    antigravity_paths.push(PathBuf::from(
        r"C:\Program Files\Antigravity\Antigravity.exe",
    ));
    antigravity_paths.push(PathBuf::from(
        r"C:\Program Files (x86)\Antigravity\Antigravity.exe",
    ));

    antigravity_paths
}

pub fn find_running_antigravity_exes() -> Vec<PathBuf> {
    if !cfg!(windows) {
        return Vec::new();
    }

    let mut system = System::new();
    system.refresh_processes();

    let mut paths = Vec::new();
    for process in system.processes_by_name("Antigravity.exe") {
        if let Some(exe) = process.exe() {
            let path = exe.to_path_buf();
            if validate_antigravity_exe(&path) {
                paths.push(path);
            }
        }
    }

    for process in system.processes_by_name("Antigravity") {
        if let Some(exe) = process.exe() {
            let path = exe.to_path_buf();
            if validate_antigravity_exe(&path) {
                paths.push(path);
            }
        }
    }

    paths
}

pub fn resolve_antigravity_exe_windows() -> Option<PathBuf> {
    if !cfg!(windows) {
        return None;
    }

    if let Some(persisted) = load_persisted_antigravity_path() {
        return Some(persisted);
    }

    for path in get_antigravity_windows_paths() {
        if validate_antigravity_exe(&path) {
            let _ = persist_antigravity_path(&path);
            return Some(path);
        }
    }

    for path in find_running_antigravity_exes() {
        let _ = persist_antigravity_path(&path);
        return Some(path);
    }

    None
}

pub fn is_antigravity_process_running() -> bool {
    if !cfg!(windows) {
        return false;
    }

    !find_running_antigravity_exes().is_empty()
}

/// 获取所有可能的Antigravity数据库路径
pub fn get_all_antigravity_db_paths() -> Vec<PathBuf> {
    let mut db_paths = Vec::new();

    // 主要路径
    if let Some(main_path) = get_antigravity_db_path() {
        db_paths.push(main_path);
    }

    // 搜索其他可能的位置
    for install_dir in find_antigravity_installations() {
        if install_dir.exists() {
            // 递归搜索state.vscdb文件
            if let Ok(entries) = std::fs::read_dir(&install_dir) {
                for entry in entries.flatten() {
                    let path = entry.path();
                    if path.is_file() && path.file_name().is_some_and(|name| name == "state.vscdb")
                    {
                        db_paths.push(path);
                    }
                }
            }
        }
    }

    db_paths
}

/// 关闭Antigravity进程
pub fn kill_antigravity_processes() -> Result<String, String> {
    match std::env::consts::OS {
        "windows" => {
            // Windows: 尝试多种可能的进程名
            let process_names = vec!["Antigravity.exe", "Antigravity"];
            let mut last_error = String::new();

            for process_name in process_names {
                let output = Command::new("taskkill")
                    .args(["/F", "/IM", process_name])
                    .output()
                    .map_err(|e| format!("执行taskkill命令失败: {}", e))?;

                if output.status.success() {
                    return Ok(format!("已成功关闭Antigravity进程 ({})", process_name));
                } else {
                    last_error = format!(
                        "关闭进程 {} 失败: {:?}",
                        process_name,
                        String::from_utf8_lossy(&output.stderr)
                    );
                }
            }

            Err(last_error)
        }
        "macos" | "linux" => {
            // macOS/Linux: 使用pkill命令，尝试多种进程名模式
            let process_patterns = vec!["Antigravity", "antigravity"];
            let mut last_error = String::new();

            for pattern in process_patterns {
                let output = Command::new("pkill")
                    .args(["-f", pattern])
                    .output()
                    .map_err(|e| format!("执行pkill命令失败: {}", e))?;

                if output.status.success() {
                    return Ok(format!("已成功关闭Antigravity进程 (模式: {})", pattern));
                } else {
                    last_error = format!(
                        "关闭进程失败 (模式: {}): {:?}",
                        pattern,
                        String::from_utf8_lossy(&output.stderr)
                    );
                }
            }

            Err(last_error)
        }
        _ => Err("不支持的操作系统".to_string()),
    }
}
