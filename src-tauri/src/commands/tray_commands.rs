//! 系统托盘命令
//! 负责系统托盘的启用、禁用、最小化、恢复等操作

/// 启用系统托盘
#[tauri::command]
pub async fn enable_system_tray() -> Result<String, String> {
    if let Some(manager) = crate::system_tray::SystemTrayManager::get_global() {
        // 安全的锁获取，避免毒化锁 panic
        match manager.lock() {
            Ok(mut manager) => match manager.enable() {
                Ok(_) => Ok("系统托盘功能已启用".to_string()),
                Err(e) => Err(format!("启用系统托盘失败: {}", e)),
            },
            Err(_) => Err("系统托盘管理器不可用（可能正在维护中）".to_string()),
        }
    } else {
        Err("系统托盘未初始化".to_string())
    }
}

/// 禁用系统托盘
#[tauri::command]
pub async fn disable_system_tray() -> Result<String, String> {
    if let Some(manager) = crate::system_tray::SystemTrayManager::get_global() {
        // 安全的锁获取，避免毒化锁 panic
        match manager.lock() {
            Ok(mut manager) => match manager.disable() {
                Ok(_) => Ok("系统托盘功能已禁用".to_string()),
                Err(e) => Err(format!("禁用系统托盘失败: {}", e)),
            },
            Err(_) => Err("系统托盘管理器不可用（可能正在维护中）".to_string()),
        }
    } else {
        Err("系统托盘未初始化".to_string())
    }
}

/// 最小化到托盘
#[tauri::command]
pub async fn minimize_to_tray() -> Result<String, String> {
    if let Some(manager) = crate::system_tray::SystemTrayManager::get_global() {
        // 使用可变锁获取，避免死锁
        match manager.lock() {
            Ok(mut manager) => match manager.minimize_to_tray() {
                Ok(_) => Ok("窗口已最小化到系统托盘".to_string()),
                Err(e) => Err(format!("最小化到托盘失败: {}", e)),
            },
            Err(_) => Err("系统托盘管理器不可用（可能正在维护中）".to_string()),
        }
    } else {
        Err("系统托盘未初始化".to_string())
    }
}

/// 从托盘恢复窗口
#[tauri::command]
pub async fn restore_from_tray() -> Result<String, String> {
    if let Some(manager) = crate::system_tray::SystemTrayManager::get_global() {
        // 使用可变锁获取，避免死锁
        match manager.lock() {
            Ok(mut manager) => match manager.restore_from_tray() {
                Ok(_) => Ok("窗口已从系统托盘恢复".to_string()),
                Err(e) => Err(format!("从托盘恢复失败: {}", e)),
            },
            Err(_) => Err("系统托盘管理器不可用（可能正在维护中）".to_string()),
        }
    } else {
        Err("系统托盘未初始化".to_string())
    }
}

/// 检查系统托盘是否启用
#[tauri::command]
pub async fn is_system_tray_enabled() -> Result<bool, String> {
    if let Some(manager) = crate::system_tray::SystemTrayManager::get_global() {
        // 安全的锁获取，避免毒化锁 panic
        match manager.lock() {
            Ok(manager) => Ok(manager.is_enabled()),
            Err(_) => {
                // 锁中毒时返回默认值，但记录错误
                eprintln!("⚠️ 系统托盘管理器锁中毒，返回默认状态");
                Ok(false)
            }
        }
    } else {
        Ok(false)
    }
}

/// 保存系统托盘状态
#[tauri::command]
pub async fn save_system_tray_state(enabled: bool) -> Result<String, String> {
    match crate::window_state_manager::save_system_tray_state(enabled).await {
        Ok(_) => Ok("系统托盘状态已保存".to_string()),
        Err(e) => Err(format!("保存系统托盘状态失败: {}", e)),
    }
}

/// 获取系统托盘状态
#[tauri::command]
pub async fn get_system_tray_state() -> Result<bool, String> {
    crate::window_state_manager::get_system_tray_state().await
}

// 命令函数将在后续步骤中移动到这里
