use once_cell::sync::OnceCell;
use std::sync::{Arc, Mutex};
/// 系统托盘管理模块
///
/// 使用 Tauri 2.x 内置的系统托盘 API
use tauri::{
    image::Image,
    menu::{MenuBuilder, MenuItem},
    tray::TrayIconBuilder,
    AppHandle, Manager,
};

/// 全局系统托盘管理器实例 - 使用 OnceCell 避免未定义行为
static SYSTEM_TRAY_MANAGER: OnceCell<Arc<Mutex<SystemTrayManager>>> = OnceCell::new();

/// 系统托盘管理器
pub struct SystemTrayManager {
    is_enabled: bool,
    app_handle: Option<AppHandle>,
    tray_icon: Option<tauri::tray::TrayIcon>,
    is_minimizing: bool, // 防止重入的标志
}

impl SystemTrayManager {
    /// 创建新的系统托盘管理器
    pub fn new() -> Self {
        Self {
            is_enabled: false,
            app_handle: None,
            tray_icon: None,
            is_minimizing: false,
        }
    }

    /// 初始化全局系统托盘管理器
    pub fn initialize_global(app_handle: &AppHandle) -> Result<(), Box<dyn std::error::Error>> {
        // 检查是否已经初始化
        if SYSTEM_TRAY_MANAGER.get().is_some() {
            return Ok(());
        }

        let mut manager = SystemTrayManager::new();
        manager.app_handle = Some(app_handle.clone());

        // 创建托盘图标
        println!("📋 创建系统托盘图标");

        // 尝试读取托盘图标
        let tray_icon_path = std::path::PathBuf::from(env!("CARGO_MANIFEST_DIR"))
            .join("icons")
            .join("tray-icon.png");

        // 创建菜单项
        let show_item = MenuItem::with_id(app_handle, "show", "显示窗口", true, None::<&str>)?;
        let hide_item = MenuItem::with_id(app_handle, "hide", "隐藏窗口", true, None::<&str>)?;
        let quit_item = MenuItem::with_id(app_handle, "quit", "退出应用", true, None::<&str>)?;

        let menu = MenuBuilder::new(app_handle)
            .item(&show_item)
            .separator()
            .item(&hide_item)
            .separator()
            .item(&quit_item)
            .build()?;

        // 构建托盘图标
        let mut tray_builder = TrayIconBuilder::new()
            .menu(&menu)
            .tooltip("Antigravity Agent");

        // 如果图标文件存在，加载图标
        if tray_icon_path.exists() {
            println!("📋 尝试加载托盘图标: {}", tray_icon_path.display());
            match std::fs::read(&tray_icon_path) {
                Ok(icon_data) => {
                    // 使用 image crate 处理 PNG 图像
                    match image::load_from_memory(&icon_data) {
                        Ok(img) => {
                            let rgba_img = img.to_rgba8();
                            let (width, height) = rgba_img.dimensions();
                            let rgba_data = rgba_img.into_raw();

                            // 创建 Tauri Image
                            let tauri_image =
                                Image::new_owned(rgba_data, width as u32, height as u32);
                            tray_builder = tray_builder.icon(tauri_image);
                            println!("✅ 托盘图标加载成功，尺寸: {}x{}", width, height);
                        }
                        Err(e) => {
                            println!("⚠️ 图像处理失败: {}", e);
                        }
                    }
                }
                Err(e) => {
                    println!("⚠️ 读取图标文件失败: {}", e);
                }
            }
        } else {
            println!("⚠️ 托盘图标文件不存在，使用默认图标");
        }

        // 创建托盘图标
        match tray_builder.build(app_handle) {
            Ok(tray) => {
                manager.tray_icon = Some(tray.clone());
                println!("✅ 系统托盘图标创建成功");

                // 设置菜单事件监听
                tray.on_menu_event(move |app, event| match event.id().as_ref() {
                    "show" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.show();
                            let _ = window.set_focus();
                            println!("📋 菜单: 显示窗口");
                        }
                    }
                    "hide" => {
                        if let Some(window) = app.get_webview_window("main") {
                            let _ = window.hide();
                            println!("📋 菜单: 隐藏窗口");
                        }
                    }
                    "quit" => {
                        println!("📋 菜单: 退出应用");
                        app.exit(0);
                    }
                    _ => {
                        println!("🖱️ 未知菜单项: {:?}", event.id());
                    }
                });
            }
            Err(e) => {
                println!("⚠️ 创建系统托盘图标失败: {}", e);
            }
        }

        // 使用 OnceCell 安全地设置全局实例
        let manager_arc = Arc::new(Mutex::new(manager));
        if let Err(_) = SYSTEM_TRAY_MANAGER.set(manager_arc) {
            // 如果设置失败，说明已经被设置了，直接返回
            return Ok(());
        }

        println!("✅ 系统托盘管理器初始化成功");
        Ok(())
    }

    /// 获取全局系统托盘管理器
    pub fn get_global() -> Option<Arc<Mutex<SystemTrayManager>>> {
        SYSTEM_TRAY_MANAGER.get().map(|mgr| Arc::clone(mgr))
    }

    /// 启用系统托盘功能
    pub fn enable(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.is_enabled = true;
        println!("✅ 系统托盘功能已启用");
        Ok(())
    }

    /// 禁用系统托盘功能
    pub fn disable(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        self.is_enabled = false;
        // 移除托盘图标
        if let Some(_tray) = self.tray_icon.take() {
            println!("🔴 系统托盘图标已移除");
        }
        println!("🔴 系统托盘功能已禁用");
        Ok(())
    }

    /// 检查系统托盘是否启用
    pub fn is_enabled(&self) -> bool {
        self.is_enabled
    }

    /// 最小化窗口到系统托盘 - 使用内部可变引用避免锁竞争
    pub fn minimize_to_tray(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        // 检查是否已经在最小化过程中，防止重入
        if self.is_minimizing {
            println!("📋 已经在最小化到托盘的过程中，跳过重复调用");
            return Ok(());
        }

        if !self.is_enabled {
            return Err("系统托盘功能未启用".into());
        }

        // 设置重入保护标志
        self.is_minimizing = true;

        if let Some(app_handle) = &self.app_handle {
            if let Some(window) = app_handle.get_webview_window("main") {
                // 隐藏主窗口
                if let Err(e) = window.hide() {
                    self.is_minimizing = false; // 失败时重置标志
                    return Err(format!("隐藏窗口失败: {}", e).into());
                }
                println!("📋 窗口已最小化到系统托盘");
            }
        }

        // 清除重入保护标志
        self.is_minimizing = false;
        Ok(())
    }

    /// 从系统托盘恢复窗口 - 使用内部可变引用避免锁竞争
    pub fn restore_from_tray(&mut self) -> Result<(), Box<dyn std::error::Error>> {
        if let Some(app_handle) = &self.app_handle {
            if let Some(window) = app_handle.get_webview_window("main") {
                // 显示并聚焦主窗口
                window.show()?;
                window.set_focus()?;
                println!("📋 窗口已从系统托盘恢复");
            }
        }

        Ok(())
    }
}
