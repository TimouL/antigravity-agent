import React, { useEffect, useState } from 'react';
import { invoke } from '@tauri-apps/api/core';
import { save } from '@tauri-apps/plugin-dialog';
import { writeTextFile } from '@tauri-apps/plugin-fs';
import { VibratingButton } from './VibratingButton';

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

interface SilentLogExportProps {
  children: React.ReactNode;
}

/**
 * 静默日志导出组件
 * 长按2秒触发日志导出，0.5秒后开始渐进式震动效果
 */
export const SilentLogExport: React.FC<SilentLogExportProps> = ({ children }) => {
  const [isLongPressActive, setIsLongPressActive] = useState(false);
  const [deviceType, setDeviceType] = useState<'mouse' | 'touch'>('mouse');

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
  const handleLogExport = async (): Promise<void> => {
    console.log('📤 开始处理日志导出...');
    try {
      // 1. 获取日志内容
      const logContent = await invoke<string>('get_log_content');
      console.log('📄 日志内容获取成功，大小:', logContent.length, '字符');

      if (!logContent || logContent.trim().length === 0) {
        console.warn('⚠️ 日志内容为空，取消导出');
        return;
      }

      // 2. 生成默认文件名
      const defaultFileName = generateFileName('antigravity-agent-logs', 'log');

      // 3. 显示文件保存对话框
      const selectedPath = await save({
        title: '保存日志文件',
        defaultPath: defaultFileName,
        filters: LOG_FILE_FILTERS
      });

      // 4. 检查用户是否选择了路径
      if (!selectedPath) {
        console.log('🚫 用户取消了日志导出');
        return;
      }

      // 5. 写入文件
      await writeTextFile(selectedPath as string, logContent);
      console.log('✅ 日志导出成功:', selectedPath);

    } catch (error) {
      console.error('❌ 日志导出失败:', error);
    }
  };

  // 震动完成时触发导出
  const handleVibrationComplete = () => {
    console.log('🎯 震动完成，准备触发导出，设备类型:', deviceType);
    handleLogExport();
  };

  // 事件处理器
  const handleMouseDown = (e: React.MouseEvent) => {
    e.preventDefault(); // 防止文本选择等默认行为
    console.log('🖱️ 鼠标按下，开始长按');
    setDeviceType('mouse');
    setIsLongPressActive(true);
  };

  const handleMouseUp = (e: React.MouseEvent) => {
    e.preventDefault();
    console.log('🖱️ 鼠标释放，结束长按');
    setIsLongPressActive(false);
  };

  const handleMouseLeave = (e: React.MouseEvent) => {
    console.log('🖱️ 鼠标离开，取消长按');
    setIsLongPressActive(false);
  };

  const handleTouchStart = (e: React.TouchEvent) => {
    e.preventDefault();
    console.log('📱 触摸开始，开始长按');
    setDeviceType('touch');
    setIsLongPressActive(true);
  };

  const handleTouchEnd = (e: React.TouchEvent) => {
    e.preventDefault();
    console.log('📱 触摸结束，结束长按');
    setIsLongPressActive(false);
  };

  return (
    <div
      onMouseDown={handleMouseDown}
      onMouseUp={handleMouseUp}
      onMouseLeave={handleMouseLeave}
      onTouchStart={handleTouchStart}
      onTouchEnd={handleTouchEnd}
      style={{
        cursor: 'pointer',
        userSelect: 'none', // 防止文本选择
        WebkitUserSelect: 'none',
        MozUserSelect: 'none',
        msUserSelect: 'none',
        display: 'inline-block'
      }}
      title="长按2秒导出日志"
    >
      <VibratingButton
        isActive={isLongPressActive}
        onComplete={handleVibrationComplete}
        totalDuration={2000}
        vibrationStartDelay={500}
      >
        {children}
      </VibratingButton>
    </div>
  );
};