//! 日志装饰器工具
//! 提供命令执行的自动日志记录功能

/// 异步命令日志记录宏
/// 自动记录命令开始、结束、执行时间和结果
#[macro_export]
macro_rules! log_async_command {
    ($command_name:expr, $future:expr) => {{
        let start_time = std::time::Instant::now();
        log::info!("🔧 开始执行命令: {}", $command_name);

        match $future.await {
            Ok(result) => {
                let duration = start_time.elapsed();
                log::info!("✅ 命令完成: {} (耗时: {:?})", $command_name, duration);
                Ok(result)
            }
            Err(e) => {
                let duration = start_time.elapsed();
                log::error!(
                    "❌ 命令失败: {} - 错误: {} (耗时: {:?})",
                    $command_name,
                    e,
                    duration
                );
                Err(e)
            }
        }
    }};
}

/// 记录系统启动信息
pub fn log_system_info() {
    log::info!("🚀 启动 Antigravity Agent v{}", env!("CARGO_PKG_VERSION"));
    log::info!(
        "🖥️ 系统信息: {} {}",
        std::env::consts::OS,
        std::env::consts::ARCH
    );

    log::info!("📁 配置目录已初始化");
    log::info!("📁 日志系统已启用");
}

/// 记录数据库操作
pub fn log_database_operation(operation: &str, table: Option<&str>, success: bool) {
    if let Some(table) = table {
        if success {
            log::info!("🗄️ 数据库操作成功: {} - 表: {}", operation, table);
        } else {
            log::error!("❌ 数据库操作失败: {} - 表: {}", operation, table);
        }
    } else if success {
        log::info!("🗄️ 数据库操作成功: {}", operation);
    } else {
        log::error!("❌ 数据库操作失败: {}", operation);
    }
}