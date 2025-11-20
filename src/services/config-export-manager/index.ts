/**
 * 配置导出管理器模块
 * 统一的配置文件导入导出功能入口
 */

// 主要类导出
export { ConfigExportManager, createConfigExportManager, defaultConfigExportManager } from './config-export-manager';

// 功能模块导出
export { ConfigImportManager } from './config-import-manager';
export { ConfigExportHandler } from './config-export-handler';
export { ConfigEncryptionProvider, defaultEncryptionProvider } from './encryption-provider';

// 类型定义导出
export type {
  EncryptedConfigData,
  ConfigImportResult,
  ConfigExportResult,
  ConfigImportOptions,
  ConfigExportOptions,
  ConfigOperationStatus,
  ConfigOperationProgress,
  ConfigValidationResult,
  ConfigProgressCallback,
  ConfigValidator,
  EncryptionProvider
} from './types';

// 兼容性导出（保持向后兼容）
export { ConfigExportManager as ConfigManager } from './config-export-manager';