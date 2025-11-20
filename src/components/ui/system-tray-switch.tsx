import React from 'react';
import * as Switch from '@radix-ui/react-switch';
import { Monitor, Moon } from 'lucide-react';
import { SystemTrayService } from '../../services/system-tray-service';

interface SystemTraySwitchProps {
  checked: boolean;
  onCheckedChange: (checked: boolean) => void;
  disabled?: boolean;
  showStatus: (message: string, isError?: boolean) => void;
}

/**
 * 系统托盘开关组件
 *
 * 当启用时，关闭按钮会变成最小化到系统托盘
 * 当禁用时，恢复正常关闭行为
 */
const SystemTraySwitch: React.FC<SystemTraySwitchProps> = ({
  checked,
  onCheckedChange,
  disabled = false,
  showStatus
}) => {
  const [isChanging, setIsChanging] = React.useState(false);

  const handleCheckedChange = async (newChecked: boolean) => {
    if (isChanging) return;

    setIsChanging(true);
    showStatus(newChecked ? '正在启用系统托盘...' : '正在禁用系统托盘...');

    try {
      let result;
      if (newChecked) {
        result = await SystemTrayService.enableSystemTrayWithSave();
      } else {
        result = await SystemTrayService.disableSystemTrayWithSave();
      }

      if (result.enabled === newChecked) {
        showStatus(result.message || `系统托盘已${newChecked ? '启用' : '禁用'}`);
        // 通知父组件状态已更改
        onCheckedChange(newChecked);
      } else {
        showStatus(result.message || '操作失败', true);
        // 不调用 onCheckedChange，保持原状态
      }
    } catch (error) {
      const errorMessage = error instanceof Error ? error.message : '未知错误';
      showStatus(`操作失败: ${errorMessage}`, true);
      // 不调用 onCheckedChange，保持原状态
    } finally {
      setIsChanging(false);
    }
  };

  return (
    <div className="flex items-center gap-3">
      <div className="flex items-center gap-2">
        <Monitor className="w-4 h-4 text-gray-600 dark:text-gray-400" />
        <span className="text-sm font-medium text-gray-700 dark:text-gray-300">
          系统托盘
        </span>
      </div>

      <Switch.Root
        className="SwitchRoot"
        checked={checked}
        onCheckedChange={handleCheckedChange}
        disabled={disabled || isChanging}
      >
        <Switch.Thumb className="SwitchThumb" />
      </Switch.Root>

      <div className="flex items-center gap-1">
        {checked ? (
          <>
            <Moon className="w-3 h-3 text-blue-600 dark:text-blue-400" />
            <span className="text-xs text-gray-600 dark:text-gray-400">
              最小化
            </span>
          </>
        ) : (
          <span className="text-xs text-gray-500 dark:text-gray-500">
              关闭
            </span>
        )}
      </div>

      <style>{`
        .SwitchRoot {
          width: 42px;
          height: 24px;
          background-color: ${checked ? '#3b82f6' : '#e5e7eb'};
          border-radius: 9999px;
          position: relative;
          transition: all 100ms ease;
          cursor: pointer;
          border: none;
          outline: none;
        }

        .SwitchRoot:hover {
          background-color: ${checked ? '#2563eb' : '#d1d5db'};
        }

        .SwitchRoot:focus-visible {
          box-shadow: 0 0 0 2px rgba(59, 130, 246, 0.5);
        }

        .SwitchRoot:disabled {
          opacity: 0.5;
          cursor: not-allowed;
        }

        .SwitchThumb {
          display: block;
          width: 18px;
          height: 18px;
          background-color: white;
          border-radius: 9999px;
          transition: transform 100ms ease;
          transform: ${checked ? 'translateX(18px)' : 'translateX(2px)'};
          box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
        }
      `}</style>
    </div>
  );
};

export default SystemTraySwitch;