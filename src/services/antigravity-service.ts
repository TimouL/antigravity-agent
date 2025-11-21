import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';

/**
 * Antigravity 服务 - 处理 Antigravity 相关操作
 */
export class AntigravityService {
  /**
   * 备份并重启Antigravity
   */
  static async backupAndRestartAntigravity(
    onStatusUpdate: (message: string, isError?: boolean) => void
  ): Promise<void> {
    try {
      console.log('🚀 开始执行备份并重启 Antigravity 流程');
      onStatusUpdate('正在关闭 Antigravity 进程...');

      console.log('📞 调用后端 backup_and_restart_antigravity 命令');
      const result = await invoke('backup_and_restart_antigravity') as string;
      console.log('✅ 后端命令执行成功，结果:', result);

      onStatusUpdate(result);

    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : String(error);
      console.error('❌ 备份并重启失败:', errorMessage);
      console.error('❌ 完整错误对象:', error);
      throw new Error(`备份并重启失败: ${errorMessage}`);
    }
  }

  static async ensureAntigravityPath(
    onStatusUpdate: (message: string, isError?: boolean) => void
  ): Promise<boolean> {
    const isWindows = navigator.userAgent.includes('Windows');
    if (!isWindows) {
      return true;
    }

    try {
      const running = await invoke<boolean>('is_antigravity_running');
      if (!running) {
        onStatusUpdate('未检测到 Antigravity 进程，请先启动应用后再登录新账户', true);
        return false;
      }

      const resolved = await invoke<string | null>('resolve_antigravity_path');
      if (resolved) {
        return true;
      }

      onStatusUpdate('未找到 Antigravity 安装路径，请选择可执行文件');
      const selected = await open({
        title: '请选择 Antigravity 可执行文件',
        multiple: false,
        filters: [
          { name: '可执行文件', extensions: ['exe'] },
          { name: '所有文件', extensions: ['*'] }
        ]
      });

      if (!selected || Array.isArray(selected)) {
        onStatusUpdate('未选择可执行文件，已取消登录新账户', true);
        return false;
      }

      await invoke('save_antigravity_path', { path: selected });
      onStatusUpdate('已更新 Antigravity 安装路径');
      return true;
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : String(error);
      onStatusUpdate(`检查/更新 Antigravity 路径失败: ${errorMessage}`, true);
      return false;
    }
  }

  /**
   * 获取备份列表
   */
  static async getBackupList(): Promise<string[]> {
    try {
      const backupList = await invoke('list_backups') as string[];
      if (!backupList) {
        throw new Error('无法获取备份列表');
      }
      return backupList;
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : String(error);
      throw new Error(`获取备份列表失败: ${errorMessage}`);
    }
  }
}