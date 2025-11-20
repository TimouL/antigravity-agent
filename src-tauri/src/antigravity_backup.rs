// Antigravity 用户数据备份模块
// 负责将 Antigravity 应用数据备份到 JSON 文件

use rusqlite::{Connection, OptionalExtension};
use serde_json::Value;
use std::fs;
use std::path::PathBuf;

use crate::platform_utils;

/// 智能备份 Antigravity 账户（终极版 - 保存完整 Marker）
///
/// 备份策略：
/// 1. 保存所有关键字段的原始字符串值
/// 2. 保存完整的 __$__targetStorageMarker 对象（作为恢复时的参考）
/// 3. 保存 __$__isNewStorageMarker 状态标记
///
/// # 参数
/// - `email`: 用户邮箱
///
/// # 返回
/// - `Ok((backup_name, is_overwrite))`: 备份文件名和是否为覆盖操作
/// - `Err(message)`: 错误信息
pub fn smart_backup_antigravity_account(email: &str) -> Result<(String, bool), String> {
    println!("🔧 执行智能备份（完整 Marker 模式），邮箱: {}", email);

    let config_dir = dirs::config_dir()
        .unwrap_or_else(|| PathBuf::from("."))
        .join(".antigravity-agent")
        .join("antigravity-accounts");
    fs::create_dir_all(&config_dir).map_err(|e| e.to_string())?;

    // 简单的覆盖逻辑：每个邮箱只保留一个备份
    let backup_name = email.to_string();
    let is_overwrite = config_dir.join(format!("{}.json", backup_name)).exists();
    
    let app_data = platform_utils::get_antigravity_db_path()
        .ok_or("未找到数据库路径")?;
    
    if !app_data.exists() {
        return Err(format!("数据库文件不存在: {}", app_data.display()));
    }

    let conn = Connection::open(&app_data).map_err(|e| e.to_string())?;

    // 定义所有需要备份的关键字段
    let keys_to_backup = vec![
        "antigravityAuthStatus",
        "antigravity.profileUrl",
        "antigravityUserSettings.allUserSettings",
        "antigravityOnboarding",
        "google.antigravity",
        "antigravity_allowed_command_model_configs",
        "jetskiStateSync.agentManagerInitState",
        "chat.ChatSessionStore.index",
        "__$__isNewStorageMarker", // 关键：同步这个状态标记
    ];

    let mut data_map = serde_json::Map::new();

    // 1. 提取数据（保持原始字符串格式）
    for key in &keys_to_backup {
        let val: Option<String> = conn
            .query_row(
                "SELECT value FROM ItemTable WHERE key = ?",
                [key],
                |row| row.get(0),
            )
            .optional()
            .unwrap_or(None);
        
        if let Some(v) = val {
            println!("  📦 备份字段: {}", key);
            data_map.insert(key.to_string(), Value::String(v));
        } else {
            println!("  ℹ️ 字段不存在: {} (跳过)", key);
        }
    }

    // 2. 提取并解析 Marker（作为恢复时的参考书）
    let marker_json: Option<String> = conn
        .query_row(
            "SELECT value FROM ItemTable WHERE key = '__$__targetStorageMarker'",
            [],
            |row| row.get(0),
        )
        .optional()
        .unwrap_or(None);

    if let Some(m) = marker_json {
        // 将 Marker 解析为对象存入备份
        if let Ok(parsed_marker) = serde_json::from_str::<Value>(&m) {
            println!("  📋 备份完整 Marker（作为恢复参考）");
            data_map.insert("__$__targetStorageMarker".to_string(), parsed_marker);
        }
    }

    // 3. 添加元信息
    data_map.insert("account_email".to_string(), Value::String(email.to_string()));
    data_map.insert("backup_time".to_string(), Value::String(chrono::Local::now().to_rfc3339()));

    // 4. 写入备份文件
    let backup_file = config_dir.join(format!("{}.json", backup_name));
    let file_content = serde_json::to_string_pretty(&data_map).map_err(|e| e.to_string())?;
    fs::write(&backup_file, file_content).map_err(|e| e.to_string())?;

    let action = if is_overwrite { "覆盖" } else { "创建" };
    println!("✅ 备份成功 ({}): {}", action, backup_file.display());
    Ok((backup_name, is_overwrite))
}