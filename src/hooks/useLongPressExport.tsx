import { useRef } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { save } from '@tauri-apps/plugin-dialog';
import { writeTextFile, exists, readTextFile } from '@tauri-apps/plugin-fs';

// 配置常量
const LONG_PRESS_DURATION = 2000; // 长按时间（毫秒）
const VIBRATION_START_DELAY = 500; // 震动开始延迟（毫秒）
const LOG_FILE_FILTERS = [
  {
    name: '日志文件',
    extensions: ['log'] as string[]
  },
  {
    name: '所有文件',
    extensions: ['*'] as string[]
  }
];


/**
 * 长按导出Hook
 * 处理长按事件和日志导出逻辑
 */
export const useLongPressExport = () => {
  const timerRef = useRef<ReturnType<typeof setTimeout> | null>(null);
  const startTimeRef = useRef<number>(0);

  /**
   * 生成带时间戳的文件名
   */
  const generateFileName = (prefix: string, extension: string): string => {
    const now = new Date();
    const timestamp = now.toISOString()
      .slice(0, 19)
      .replace(/:/g, '-');
    return `${prefix}-${timestamp}.${extension}`;
  };

  /**
   * 处理日志导出的核心逻辑
   */
  const handleLogExport = async (deviceType: 'mouse' | 'touch'): Promise<void> => {
    try {
      console.log(`📤 正在获取日志内容（${deviceType}）...`);

      // 1. 获取日志内容
      const logContent = await invoke<string>('get_log_content');

      if (!logContent || logContent.trim().length === 0) {
        console.warn('⚠️ 日志内容为空，取消导出');
        return;
      }

      console.log(`📄 日志内容获取成功（${deviceType}），大小: ${logContent.length} 字符`);

      // 2. 生成默认文件名
      const defaultFileName = generateFileName('antigravity-agent-logs', 'log');
      console.log(`📝 默认文件名（${deviceType}）: ${defaultFileName}`);

      // 3. 显示文件保存对话框
      console.log(`💾 显示文件保存对话框（${deviceType}）...`);
      const selectedPath = await save({
        title: '保存日志文件',
        defaultPath: defaultFileName,
        filters: LOG_FILE_FILTERS
      });

      // 4. 检查用户是否选择了路径
      if (!selectedPath) {
        console.log(`🚫 用户取消了日志导出（${deviceType}）`);
        return;
      }

      // 5. 写入文件
      console.log(`💾 正在保存到: ${selectedPath}（${deviceType}）`);

      try {

        await writeTextFile(selectedPath as string, logContent);
        console.log(`✅ 日志导出成功（${deviceType}）:`, selectedPath);

      } catch (writeError) {
        console.error(`💥 文件写入失败（${deviceType}）:`, writeError);
        throw writeError;
      }

    } catch (error) {
      // 详细错误处理
      if (error instanceof Error) {
        // 检查是否是用户取消
        if (error.message.includes('用户取消') || error.message.includes('cancel') || error.message.includes('abort')) {
          console.log(`🚫 用户取消了日志导出（${deviceType}）`);
        } else if (error.message.includes('permission')) {
          console.error(`🚫 权限被拒绝（${deviceType}）:`, error.message);
        } else if (error.message.includes('space') || error.message.includes('disk')) {
          console.error(`💾 磁盘空间不足（${deviceType}）:`, error.message);
        } else if (error.message.includes('path') || error.message.includes('directory')) {
          console.error(`📁 路径错误（${deviceType}）:`, error.message);
        } else {
          console.error(`❌ 日志导出失败（${deviceType}）:`, error.message);
        }
      } else {
        console.error(`❌ 日志导出失败（${deviceType}）:`, error);
      }
    }
  };

  /**
   * 开始长按（由震动组件管理时间）
   */
  const startLongPress = (deviceType: 'mouse' | 'touch'): void => {
    console.log(`${deviceType === 'mouse' ? '🖱️ 鼠标按下' : '📱 触摸开始'}，等待震动完成...`);
    startTimeRef.current = Date.now();
  };

  /**
   * 结束长按（取消）
   */
  const endLongPress = (deviceType: 'mouse' | 'touch'): void => {
    const elapsed = Date.now() - startTimeRef.current;
    console.log(`${deviceType === 'mouse' ? '🖱️ 鼠标释放' : '📱 触摸结束'}，长按时间: ${elapsed}ms`);

    if (timerRef.current) {
      clearTimeout(timerRef.current);
      timerRef.current = null;
    }
    startTimeRef.current = 0;
  };

  /**
   * 长按取消（鼠标离开或触摸取消）
   */
  const cancelLongPress = (deviceType: 'mouse' | 'touch'): void => {
    console.log(`${deviceType === 'mouse' ? '🖱️ 鼠标离开' : '📱 触摸取消'}区域，取消长按`);

    if (timerRef.current) {
      clearTimeout(timerRef.current);
      timerRef.current = null;
    }
    startTimeRef.current = 0;
  };

  /**
   * 清理资源
   */
  const cleanup = (): void => {
    if (timerRef.current) {
      clearTimeout(timerRef.current);
      timerRef.current = null;
    }
    startTimeRef.current = 0;
  };

  return {
    startLongPress,
    endLongPress,
    cancelLongPress,
    cleanup,
    generateFileName,
    handleLogExport
  };
};