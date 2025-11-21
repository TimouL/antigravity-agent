//! 进程管理命令
//! 负责 Antigravity 进程的启动、关闭、重启等操作

/// 关闭 Antigravity 进程
#[tauri::command]
pub async fn kill_antigravity() -> Result<String, String> {
    crate::platform_utils::kill_antigravity_processes()
}

/// 启动 Antigravity 应用
#[tauri::command]
pub async fn start_antigravity() -> Result<String, String> {
    crate::antigravity_starter::start_antigravity()
}

/// 备份并重启 Antigravity
#[tauri::command]
pub async fn backup_and_restart_antigravity() -> Result<String, String> {
    println!("🔄 开始执行 backup_and_restart_antigravity 命令");

    // 1. 关闭进程 (如果存在)
    println!("🛑 步骤1: 检查并关闭 Antigravity 进程");
    let kill_result = match crate::platform_utils::kill_antigravity_processes() {
        Ok(result) => {
            if result.contains("not found") || result.contains("未找到") {
                println!("ℹ️ Antigravity 进程未运行，跳过关闭步骤");
                "Antigravity 进程未运行".to_string()
            } else {
                println!("✅ 进程关闭结果: {}", result);
                result
            }
        }
        Err(e) => {
            if e.contains("not found") || e.contains("未找到") {
                println!("ℹ️ Antigravity 进程未运行，跳过关闭步骤");
                "Antigravity 进程未运行".to_string()
            } else {
                return Err(format!("关闭进程时发生错误: {}", e));
            }
        }
    };

    // 等待一秒确保进程完全关闭
    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;

    // 2. 备份当前账户信息（使用统一的智能备份函数）
    println!("💾 步骤2: 备份当前账户信息");

    // 获取邮箱
    let app_data = crate::platform_utils::get_antigravity_db_path()
        .ok_or_else(|| "未找到Antigravity数据库路径".to_string())?;

    let conn = crate::Connection::open(&app_data).map_err(|e| format!("连接数据库失败: {}", e))?;

    // 获取认证信息来提取邮箱
    let auth_str: String = conn
        .query_row(
            "SELECT value FROM ItemTable WHERE key = 'antigravityAuthStatus'",
            [],
            |row| row.get(0),
        )
        .map_err(|e| format!("查询认证信息失败: {}", e))?;

    drop(conn);

    let auth_data: serde_json::Value =
        serde_json::from_str(&auth_str).map_err(|e| format!("解析认证信息失败: {}", e))?;

    let email = auth_data
        .get("email")
        .and_then(|v| v.as_str())
        .ok_or_else(|| "认证信息中未找到邮箱".to_string())?;

    println!("📧 获取到的邮箱: {}", email);

    // 调用通用智能备份函数
    let (backup_name, is_overwrite) =
        crate::antigravity_backup::smart_backup_antigravity_account(email)?;
    let backup_action = if is_overwrite { "更新" } else { "创建" };
    println!("✅ 备份完成 ({}): {}", backup_action, backup_name);

    // 3. 清除 Antigravity 所有数据 (彻底注销)
    println!("🗑️ 步骤3: 清除所有 Antigravity 数据 (彻底注销)");
    match crate::antigravity_cleanup::clear_all_antigravity_data().await {
        Ok(result) => {
            println!("✅ 清除完成: {}", result);
        }
        Err(e) => {
            println!("⚠️ 清除失败: {}", e);
            return Err(format!("清除数据失败: {}", e));
        }
    }

    // 等待一秒确保操作完成
    tokio::time::sleep(tokio::time::Duration::from_millis(1000)).await;

    // 4. 重新启动进程
    println!("🚀 步骤4: 重新启动 Antigravity");
    let start_result = crate::antigravity_starter::start_antigravity();
    let start_message = match start_result {
        Ok(result) => {
            println!("✅ 启动结果: {}", result);
            result
        }
        Err(e) => {
            println!("⚠️ 启动失败: {}", e);
            format!("启动失败: {}", e)
        }
    };

    let final_message = format!(
        "{} -> 已{}备份: {} -> 已清除账户数据 -> {}",
        kill_result, backup_action, backup_name, start_message
    );
    println!("🎉 所有操作完成: {}", final_message);

    Ok(final_message)
}

// 命令函数将在后续步骤中移动到这里
