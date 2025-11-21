//! 日志相关命令
//! 提供日志导出和管理功能

use dirs;
use regex::Regex;
use std::fs;

// 日志文件大小限制 (10MB)
const MAX_LOG_FILE_SIZE: u64 = 10 * 1024 * 1024;

/// 过滤日志中的敏感信息
/// 移除密码、token、API密钥等敏感数据
fn filter_sensitive_info(log_content: &str) -> String {
    let mut filtered_content = log_content.to_string();

    // 简单字符串替换 - 避免复杂的正则表达式
    let search_terms = vec![
        // 密码相关
        "password",
        "pwd",
        "passwd",
        // Token相关
        "token",
        "access_token",
        "refresh_token",
        // API密钥相关
        "api_key",
        "secret_key",
        "private_key",
        // 认证信息
        "authorization",
    ];

    // 过滤包含敏感信息的行
    let lines: Vec<&str> = filtered_content.lines().collect();
    let mut filtered_lines = Vec::new();

    for line in lines {
        let lower_line = line.to_lowercase();
        let contains_sensitive = search_terms.iter().any(|term| {
            lower_line.contains(&term.to_lowercase())
                && (lower_line.contains(":") || lower_line.contains("="))
        });

        if contains_sensitive {
            // 如果包含敏感信息，替换为过滤标记
            filtered_lines.push("🔒 [SENSITIVE_DATA_FILTERED]");
        } else {
            filtered_lines.push(line);
        }
    }

    filtered_content = filtered_lines.join("\n");

    // 过滤文件路径
    let lines: Vec<&str> = filtered_content.lines().collect();
    let mut filtered_lines = Vec::new();

    for line in lines {
        let contains_path = line.contains('\\')
            || line.contains('/')
            || line.contains("Users\\")
            || line.contains("AppData\\")
            || line.contains("Program Files")
            || line.contains("ProgramData")
            || line.contains(":/")
            || line.contains(":/\\");

        let contains_log_dir = line.contains("antigravity-agent")
            && (line.contains("logs") || line.contains("config"));

        if contains_path && contains_log_dir {
            // 如果包含系统路径信息，过滤掉
            filtered_lines.push("🔒 [PATH_INFO_FILTERED]");
        } else {
            filtered_lines.push(line);
        }
    }

    filtered_content = filtered_lines.join("\n");

    // 过滤邮箱地址
    if let Ok(email_regex) = Regex::new(r"[a-zA-Z0-9._%+-]+@[a-zA-Z0-9.-]+\.[a-zA-Z]{2,}") {
        let mut filtered_emails = 0;
        filtered_content = email_regex
            .replace_all(&filtered_content, |caps: &regex::Captures| {
                filtered_emails += 1;
                let email = &caps[0];
                if let Some(at_pos) = email.find('@') {
                    let username = &email[..at_pos];
                    let domain = &email[at_pos..];

                    let masked_username = if username.len() <= 2 {
                        "***".to_string()
                    } else if username.len() <= 3 {
                        format!("{}***", &username[..1])
                    } else {
                        format!("{}***{}", &username[..1], &username[username.len() - 1..])
                    };

                    format!("{}{}", masked_username, domain)
                } else {
                    "***EMAIL_FILTERED***".to_string()
                }
            })
            .to_string();
    }

    // 过滤IP地址
    if let Ok(ip_regex) = Regex::new(r"\b(?:\d{1,3}\.){3}\d{1,3}\b") {
        let mut filtered_ips = 0;
        filtered_content = ip_regex
            .replace_all(&filtered_content, |caps: &regex::Captures| {
                filtered_ips += 1;
                let ip = &caps[0];
                let parts: Vec<&str> = ip.split('.').collect();
                if parts.len() == 4 {
                    format!("{}.***.{}.{}", parts[0], parts[2], parts[3])
                } else {
                    "***IP_FILTERED***".to_string()
                }
            })
            .to_string();
    }

    // 统计过滤的敏感信息数量
    let filtered_count = filtered_content.matches("***FILTERED***").count();
    if filtered_count > 0 {
        log::info!("🔒 已过滤 {} 个敏感信息", filtered_count);
    }

    filtered_content
}

/// 获取日志内容用于导出
/// 只读取日志内容，不处理文件保存，让前端处理文件选择对话框
#[tauri::command]
pub async fn get_log_content() -> Result<String, String> {
    crate::log_async_command!("get_log_content", async {
        log::info!("📤 请求日志内容");

        // 获取日志目录 - 使用系统配置目录
        let log_dir = dirs::config_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .join("antigravity-agent")
            .join("logs");

        // 检查日志文件是否存在
        let log_file = log_dir.join("antigravity-agent.log");
        if !log_file.exists() {
            log::warn!("日志文件不存在");
            return Err("日志文件不存在".to_string());
        }

        log::info!("📄 找到日志文件");

        // 检查文件大小
        let metadata = fs::metadata(&log_file).map_err(|e| format!("获取文件信息失败: {}", e))?;

        let file_size = metadata.len();
        log::info!(
            "📄 日志文件大小: {} 字节 ({} MB)",
            file_size,
            file_size / (1024 * 1024)
        );

        if file_size > MAX_LOG_FILE_SIZE {
            return Err(format!(
                "日志文件过大 ({} MB)，超过限制 ({} MB)",
                file_size / (1024 * 1024),
                MAX_LOG_FILE_SIZE / (1024 * 1024)
            ));
        }

        // 读取日志内容
        let log_content =
            fs::read_to_string(&log_file).map_err(|e| format!("读取日志文件失败: {}", e))?;

        log::info!("📄 日志内容读取成功，大小: {} 字节", log_content.len());

        // 过滤敏感信息
        let filtered_content = filter_sensitive_info(&log_content);

        log::info!(
            "✅ 日志内容读取并过滤完成，处理后大小: {} 字节",
            filtered_content.len()
        );

        // 验证过滤后的内容不为空
        if filtered_content.trim().is_empty() {
            log::warn!("⚠️ 警告：过滤后的日志内容为空，可能是过滤规则过于严格");
        }

        Ok(filtered_content)
    })
}

/// 导出日志文件（保持向后兼容）
/// 此函数保留但不再使用，实际导出由前端处理
#[tauri::command]
pub async fn export_logs() -> Result<String, String> {
    crate::log_async_command!("export_logs", async {
        log::info!("📤 用户请求导出日志（兼容模式）");

        // 获取日志内容
        let log_content = get_log_content().await?;

        // 生成默认文件名（包含时间戳）
        let now = chrono::Utc::now();
        let timestamp = now.format("%Y%m%d_%H%M%S");
        let default_filename = format!("antigravity-agent-logs-{}.log", timestamp);

        // 保存到桌面
        let desktop = dirs::desktop_dir().ok_or("无法获取桌面目录")?;
        let export_path = desktop.join(default_filename);

        // 写入日志内容到桌面
        fs::write(&export_path, &log_content).map_err(|e| format!("写入文件失败: {}", e))?;

        log::info!("✅ 日志已成功导出");
        Ok("日志导出完成".to_string())
    })
}

/// 获取日志文件信息
/// 返回日志文件路径、大小等信息，用于前端显示状态
#[tauri::command]
pub async fn get_log_info() -> Result<LogInfo, String> {
    let log_dir = dirs::config_dir()
        .unwrap_or_else(|| std::path::PathBuf::from("."))
        .join("antigravity-agent")
        .join("logs");

    let log_file = log_dir.join("antigravity-agent.log");

    if log_file.exists() {
        let metadata = fs::metadata(&log_file).map_err(|e| format!("获取文件信息失败: {}", e))?;

        let modified = metadata
            .modified()
            .map_err(|e| format!("获取修改时间失败: {}", e))?;

        let modified_str = chrono::DateTime::<chrono::Utc>::from(modified)
            .format("%Y-%m-%d %H:%M:%S UTC")
            .to_string();

        Ok(LogInfo {
            exists: true,
            path: "日志文件路径".to_string(),
            size_bytes: metadata.len(),
            size_human: format_file_size(metadata.len()),
            last_modified: modified_str,
        })
    } else {
        Ok(LogInfo {
            exists: false,
            path: "日志文件路径".to_string(),
            size_bytes: 0,
            size_human: "0 B".to_string(),
            last_modified: "不存在".to_string(),
        })
    }
}

/// 清空日志文件
/// 删除当前日志文件内容，但保留文件本身
#[tauri::command]
pub async fn clear_logs() -> Result<String, String> {
    crate::log_async_command!("clear_logs", async {
        let log_dir = dirs::config_dir()
            .unwrap_or_else(|| std::path::PathBuf::from("."))
            .join("antigravity-agent")
            .join("logs");

        let log_file = log_dir.join("antigravity-agent.log");

        if log_file.exists() {
            // 备份当前日志（可选）
            let backup_path = log_dir.join("antigravity-agent.backup.log");
            if let Ok(_) = fs::copy(&log_file, &backup_path) {
                log::info!("📦 日志已备份");
            }

            // 清空日志文件
            fs::write(&log_file, "").map_err(|e| format!("清空日志文件失败: {}", e))?;

            log::info!("🗑️ 日志文件已清空");
            Ok("日志文件已清空".to_string())
        } else {
            Err("日志文件不存在".to_string())
        }
    })
}

#[derive(serde::Serialize)]
pub struct LogInfo {
    pub exists: bool,
    pub path: String,
    pub size_bytes: u64,
    pub size_human: String,
    pub last_modified: String,
}

/// 格式化文件大小显示
fn format_file_size(bytes: u64) -> String {
    const UNITS: &[&str] = &["B", "KB", "MB", "GB"];
    let mut size = bytes as f64;
    let mut unit_index = 0;

    while size >= 1024.0 && unit_index < UNITS.len() - 1 {
        size /= 1024.0;
        unit_index += 1;
    }

    if unit_index == 0 {
        format!("{} {}", bytes, UNITS[unit_index])
    } else {
        format!("{:.2} {}", size, UNITS[unit_index])
    }
}
