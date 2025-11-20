/**
 * 配置导出管理器
 * 统一的配置文件导入导出管理接口
 */

import { ConfigImportManager } from './config-import-manager';
import { ConfigExportHandler } from './config-export-handler';
import { defaultEncryptionProvider } from './encryption-provider';
import type {
  ConfigImportResult,
  ConfigExportResult,
  ConfigProgressCallback,
  EncryptedConfigData,
  ConfigImportOptions,
  ConfigExportOptions
} from './types';

/**
 * 配置导出管理器主类
 * 提供统一的配置文件导入导出接口
 */
export class ConfigExportManager {
  private importManager: ConfigImportManager;
  private exportHandler: ConfigExportHandler;

  constructor(onProgress?: ConfigProgressCallback) {
    this.importManager = new ConfigImportManager(defaultEncryptionProvider, onProgress);
    this.exportHandler = new ConfigExportHandler(defaultEncryptionProvider);
  }

  /**
   * 导入加密配置文件
   *
   * @param options 导入选项
   * @returns 导入结果
   */
  async importEncryptedConfig(options?: ConfigImportOptions): Promise<ConfigImportResult> {
    return this.importManager.importEncryptedConfig(options);
  }

  /**
   * 解密配置文件数据
   *
   * @param encryptedData 加密数据
   * @param password 解密密码
   * @returns 解密结果
   */
  async decryptConfigData(
    encryptedData: string,
    password: string
  ): Promise<ConfigImportResult> {
    return this.importManager.decryptConfigData(encryptedData, password);
  }

  /**
   * 导出加密配置文件
   *
   * @param password 加密密码
   * @param options 导出选项
   * @returns 导出结果
   */
  async exportEncryptedConfig(
    password: string,
    options?: ConfigExportOptions
  ): Promise<ConfigExportResult> {
    return this.exportHandler.exportEncryptedConfig(password, options);
  }

  /**
   * 验证密码强度
   *
   * @param password 要验证的密码
   * @returns 验证结果
   */
  validatePassword(password: string): { isValid: boolean; message?: string } {
    return defaultEncryptionProvider.validatePassword(password);
  }

  /**
   * 生成配置文件摘要
   *
   * @param configData 配置数据
   * @returns 配置摘要信息
   */
  generateConfigSummary(configData: EncryptedConfigData): string {
    return this.exportHandler.generateConfigSummary(configData);
  }

  /**
   * 验证导出配置
   *
   * @param configData 配置数据
   * @returns 验证结果
   */
  validateExportConfig(configData: EncryptedConfigData): { isValid: boolean; errors: string[] } {
    return this.exportHandler.validateExportConfig(configData);
  }

  /**
   * 检查是否有可导出的用户数据
   *
   * @returns 是否有用户数据
   */
  async hasExportableData(): Promise<boolean> {
    try {
      const backupList = await this.getBackupList();
      return backupList.length > 0;
    } catch {
      return false;
    }
  }

  /**
   * 获取当前备份列表
   *
   * @returns 备份列表
   */
  private async getBackupList(): Promise<string[]> {
    // 动态导入 AntigravityService 以避免循环依赖
    const { AntigravityService } = await import('../antigravity-service');
    return AntigravityService.getBackupList();
  }

  /**
   * 生成默认的导出文件名
   *
   * @param prefix 文件名前缀
   * @returns 生成的文件名
   */
  generateDefaultExportFileName(prefix: string = 'antigravity_encrypted_config'): string {
    const timestamp = new Date().toISOString()
      .slice(0, 19)
      .replace(/:/g, '-');
    return `${prefix}_${timestamp}.enc`;
  }

  /**
   * 获取支持的文件类型过滤器
   *
   * @returns 文件类型过滤器数组
   */
  getSupportedFileFilters(): Array<{ name: string; extensions: string[] }> {
    return [
      {
        name: 'Antigravity 加密配置文件',
        extensions: ['enc']
      },
      {
        name: '所有文件',
        extensions: ['*']
      }
    ];
  }

  /**
   * 获取配置文件的元数据信息
   *
   * @returns 元数据对象
   */
  getConfigMetadata(): {
    version: string;
    encryptionType: string;
    supportedFormats: string[];
    maxFileSize: string;
  } {
    return {
      version: '1.1.0',
      encryptionType: 'XOR-Base64',
      supportedFormats: ['.enc'],
      maxFileSize: '10MB'
    };
  }
}

/**
 * 创建默认配置导出管理器实例
 *
 * @param onProgress 进度回调函数
 * @returns 配置导出管理器实例
 */
export function createConfigExportManager(onProgress?: ConfigProgressCallback): ConfigExportManager {
  return new ConfigExportManager(onProgress);
}

/**
 * 默认配置导出管理器实例（无进度回调）
 */
export const defaultConfigExportManager = new ConfigExportManager();