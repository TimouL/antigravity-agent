//! 平台支持命令
//! 负责获取平台信息、安装位置验证等跨平台操作

use serde_json::Value;

/// 获取平台信息
#[tauri::command]
pub async fn get_platform_info() -> Result<Value, String> {
    let os_type = std::env::consts::OS;
    let arch = std::env::consts::ARCH;
    let family = std::env::consts::FAMILY;

    let antigravity_available = crate::platform_utils::is_antigravity_available();
    let antigravity_paths = crate::platform_utils::get_all_antigravity_db_paths();

    Ok(serde_json::json!({
        "os": os_type,
        "arch": arch,
        "family": family,
        "antigravity_available": antigravity_available,
        "antigravity_paths": antigravity_paths.iter().map(|p| p.to_string_lossy()).collect::<Vec<_>>(),
        "config_dir": dirs::config_dir().map(|p| p.to_string_lossy().to_string()),
        "data_dir": dirs::data_dir().map(|p| p.to_string_lossy().to_string()),
        "home_dir": dirs::home_dir().map(|p| p.to_string_lossy().to_string())
    }))
}

/// 查找 Antigravity 安装位置
#[tauri::command]
pub async fn find_antigravity_installations() -> Result<Vec<String>, String> {
    let paths = crate::platform_utils::find_antigravity_installations();
    Ok(paths
        .iter()
        .map(|p| p.to_string_lossy().to_string())
        .collect())
}

/// 验证 Antigravity 路径
#[tauri::command]
pub async fn validate_antigravity_path(path: String) -> Result<bool, String> {
    let path_buf = std::path::PathBuf::from(&path);
    let db_path = path_buf.join("state.vscdb");
    Ok(db_path.exists() && db_path.is_file())
}

/// 解析 Antigravity 可执行路径（仅 Windows）
#[tauri::command]
pub async fn resolve_antigravity_path() -> Result<Option<String>, String> {
    if !cfg!(windows) {
        return Ok(None);
    }

    Ok(crate::platform_utils::resolve_antigravity_exe_windows()
        .map(|p| p.to_string_lossy().to_string()))
}

/// 保存用户选择的 Antigravity 路径（仅 Windows）
#[tauri::command]
pub async fn save_antigravity_path(path: String) -> Result<(), String> {
    if !cfg!(windows) {
        return Ok(());
    }

    let path_buf = std::path::PathBuf::from(path);
    crate::platform_utils::persist_antigravity_path(&path_buf)
}

/// 检查 Antigravity 进程是否运行（仅 Windows）
#[tauri::command]
pub async fn is_antigravity_running() -> Result<bool, String> {
    if !cfg!(windows) {
        return Ok(false);
    }

    Ok(crate::platform_utils::is_antigravity_process_running())
}

// 命令函数将在后续步骤中移动到这里
