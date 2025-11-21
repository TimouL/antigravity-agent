/// 数据库字段常量
pub mod database {
    /// 认证状态
    pub const AUTH_STATUS: &str = "antigravityAuthStatus";

    /// 用户头像 URL
    pub const PROFILE_URL: &str = "antigravity.profileUrl";

    /// 用户设置
    pub const USER_SETTINGS: &str = "antigravityUserSettings.allUserSettings";

    /// 新手引导
    pub const ONBOARDING: &str = "antigravityOnboarding";

    /// Google 相关数据
    pub const GOOGLE_DATA: &str = "google.antigravity";

    /// 命令模型配置
    pub const COMMAND_CONFIGS: &str = "antigravity_allowed_command_model_configs";

    /// Agent 状态同步
    pub const AGENT_STATE: &str = "jetskiStateSync.agentManagerInitState";

    /// 聊天会话存储
    pub const CHAT_SESSION: &str = "chat.ChatSessionStore.index";

    /// 新存储标记
    pub const NEW_STORAGE_MARKER: &str = "__$__isNewStorageMarker";

    /// 目标存储标记
    pub const TARGET_STORAGE_MARKER: &str = "__$__targetStorageMarker";

    /// 所有需要备份的字段列表
    pub const ALL_KEYS: &[&str] = &[
        AUTH_STATUS,
        PROFILE_URL,
        USER_SETTINGS,
        ONBOARDING,
        GOOGLE_DATA,
        COMMAND_CONFIGS,
        AGENT_STATE,
        CHAT_SESSION,
        NEW_STORAGE_MARKER,
    ];

    /// 需要清除的字段列表（不包含会话数据）
    pub const DELETE_KEYS: &[&str] = &[
        AUTH_STATUS,
        PROFILE_URL,
        USER_SETTINGS,
        ONBOARDING,
        GOOGLE_DATA,
        COMMAND_CONFIGS,
    ];
}

/// 路径常量
pub mod paths {
    /// 配置目录名称
    pub const CONFIG_DIR_NAME: &str = ".antigravity-agent";

    /// 窗口状态文件
    pub const WINDOW_STATE_FILE: &str = "window_state.json";
}

/// 窗口状态限制
pub mod window_limits {}

/// 进程管理常量
pub mod process {}
