/**
 * 自定义 Hook：处理窗口关闭事件
 *
 * 当系统托盘启用时，关闭窗口会最小化到托盘而不是真正关闭
 */

import { useEffect } from 'react';
import { getCurrentWindow } from '@tauri-apps/api/window';
import { listen } from '@tauri-apps/api/event';
import { SystemTrayService } from '../services/system-tray-service';

// 全局变量来跟踪是否应该阻止关闭
let shouldPreventClose = false;

/**
 * 自定义 Hook：处理窗口关闭事件
 */
export const useWindowClose = () => {
  useEffect(() => {
    let unlistenFn: (() => void) | null = null;

    const setupWindowCloseHandler = async () => {
      try {
        const window = getCurrentWindow();

        // 监听窗口关闭请求事件
        unlistenFn = await listen('tauri://close-requested', async (event) => {
          console.log('🚪 收到窗口关闭请求事件');

          try {
            // 检查系统托盘是否启用
            const trayEnabled = await SystemTrayService.isSystemTrayEnabled();
            console.log('📋 系统托盘状态:', trayEnabled);

            if (trayEnabled) {
              console.log('📋 系统托盘已启用，最小化窗口而不是关闭');

              // 阻止默认的关闭行为 - 在 Tauri 2.x 中需要设置 preventDefault
              if (event.preventDefault) {
                event.preventDefault();
              }

              // 设置标志，阻止关闭
              shouldPreventClose = true;

              // 先隐藏窗口，然后最小化到系统托盘
              await window.hide();
              await SystemTrayService.minimizeToTray();

              // 发送自定义事件通知状态变化
              await window.emit('window-minimized-to-tray');

              console.log('✅ 窗口已最小化到托盘');
            } else {
              console.log('📋 系统托盘未启用，允许关闭窗口');
              shouldPreventClose = false;

              // 发送自定义事件通知状态变化
              await window.emit('window-normal-close');
            }
          } catch (error) {
            console.error('处理窗口关闭事件时出错:', error);
            // 如果出错，记录错误但仍然尝试隐藏窗口
            try {
              shouldPreventClose = true;
              await window.hide();
              console.log('📋 出错但仍尝试隐藏窗口');
            } catch (hideError) {
              console.error('隐藏窗口失败:', hideError);
            }
          }
        });

        console.log('✅ 窗口关闭处理器已设置');
      } catch (error) {
        console.error('设置窗口关闭处理器失败:', error);
      }
    };

    setupWindowCloseHandler();

    // 清理函数
    return () => {
      if (unlistenFn) {
        unlistenFn();
      }
    };
  }, []);
};