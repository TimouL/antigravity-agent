/**
 * 系统托盘服务
 *
 * 提供系统托盘功能的前端接口
 */

import { invoke } from '@tauri-apps/api/core';

export interface SystemTrayStatus {
  enabled: boolean;
  message?: string;
}

/**
 * 系统托盘服务类
 */
export class SystemTrayService {
  /**
   * 启用系统托盘功能
   */
  static async enableSystemTray(): Promise<SystemTrayStatus> {
    try {
      const result = await invoke<string>('enable_system_tray');
      return {
        enabled: true,
        message: result
      };
    } catch (error) {
      return {
        enabled: false,
        message: `启用系统托盘失败: ${error}`
      };
    }
  }

  /**
   * 禁用系统托盘功能
   */
  static async disableSystemTray(): Promise<SystemTrayStatus> {
    try {
      const result = await invoke<string>('disable_system_tray');
      return {
        enabled: false,
        message: result
      };
    } catch (error) {
      return {
        enabled: false,
        message: `禁用系统托盘失败: ${error}`
      };
    }
  }

  /**
   * 最小化窗口到系统托盘
   */
  static async minimizeToTray(): Promise<string> {
    return await invoke<string>('minimize_to_tray');
  }

  /**
   * 从系统托盘恢复窗口
   */
  static async restoreFromTray(): Promise<string> {
    return await invoke<string>('restore_from_tray');
  }

  /**
   * 检查系统托盘是否启用
   */
  static async isSystemTrayEnabled(): Promise<boolean> {
    return await invoke<boolean>('is_system_tray_enabled');
  }

  /**
   * 切换系统托盘状态
   */
  static async toggleSystemTray(enabled: boolean): Promise<SystemTrayStatus> {
    if (enabled) {
      return this.enableSystemTray();
    } else {
      return this.disableSystemTray();
    }
  }

  /**
   * 保存系统托盘状态到持久化存储
   */
  static async saveSystemTrayState(enabled: boolean): Promise<string> {
    try {
      const result = await invoke<string>('save_system_tray_state', { enabled });
      return result;
    } catch (error) {
      throw new Error(`保存系统托盘状态失败: ${error}`);
    }
  }

  /**
   * 获取持久化的系统托盘状态
   */
  static async getSystemTrayState(): Promise<boolean> {
    try {
      return await invoke<boolean>('get_system_tray_state');
    } catch (error) {
      console.warn('获取系统托盘状态失败，使用默认值:', error);
      return true; // 默认启用
    }
  }

  /**
   * 启用系统托盘并保存状态
   */
  static async enableSystemTrayWithSave(): Promise<SystemTrayStatus> {
    try {
      const status = await this.enableSystemTray();
      if (status.enabled) {
        await this.saveSystemTrayState(true);
      }
      return status;
    } catch (error) {
      return {
        enabled: false,
        message: `启用系统托盘失败: ${error}`
      };
    }
  }

  /**
   * 禁用系统托盘并保存状态
   */
  static async disableSystemTrayWithSave(): Promise<SystemTrayStatus> {
    try {
      const status = await this.disableSystemTray();
      if (!status.enabled) {
        await this.saveSystemTrayState(false);
      }
      return status;
    } catch (error) {
      return {
        enabled: false,
        message: `禁用系统托盘失败: ${error}`
      };
    }
  }
}