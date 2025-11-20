/**
 * 配置加密提供者
 * 为配置文件导入导出提供加密/解密服务
 */

import type { EncryptionProvider } from './types';

/**
 * 配置加密提供者实现类
 * 使用 XOR 算法和 Unicode 安全的 Base64 编码
 */
export class ConfigEncryptionProvider implements EncryptionProvider {
  /**
   * Unicode安全的base64编码函数
   * 支持 Unicode 字符的正确编码
   */
  private unicodeBase64Encode(str: string): string {
    try {
      // 使用 TextEncoder 处理 Unicode 字符
      const encoder = new TextEncoder();
      const data = encoder.encode(str);
      let binary = '';
      for (let i = 0; i < data.byteLength; i++) {
        binary += String.fromCharCode(data[i]);
      }
      return btoa(binary);
    } catch (error) {
      // 如果 TextEncoder 失败，使用备用方法
      return btoa(unescape(encodeURIComponent(str)));
    }
  }

  /**
   * Unicode安全的base64解码函数
   * 支持 Unicode 字符的正确解码
   */
  private unicodeBase64Decode(str: string): string {
    try {
      // 使用 TextDecoder 处理 Unicode 字符
      const binary = atob(str);
      const bytes = new Uint8Array(binary.length);
      for (let i = 0; i < binary.length; i++) {
        bytes[i] = binary.charCodeAt(i);
      }
      const decoder = new TextDecoder();
      return decoder.decode(bytes);
    } catch (error) {
      // 如果 TextDecoder 失败，使用备用方法
      try {
        return decodeURIComponent(escape(atob(str)));
      } catch (fallbackError) {
        throw new Error('Base64解码失败');
      }
    }
  }

  /**
   * 使用 XOR 算法加密数据
   *
   * @param data 要加密的原始数据
   * @param password 加密密码
   * @returns Base64 编码的加密数据
   */
  encrypt(data: string, password: string): string {
    let result = '';
    for (let i = 0; i < data.length; i++) {
      result += String.fromCharCode(
        data.charCodeAt(i) ^ password.charCodeAt(i % password.length)
      );
    }
    return this.unicodeBase64Encode(result);
  }

  /**
   * 使用 XOR 算法解密数据
   *
   * @param encryptedData Base64 编码的加密数据
   * @param password 解密密码
   * @returns 解密后的原始数据
   * @throws 解密失败时抛出错误
   */
  decrypt(encryptedData: string, password: string): string {
    try {
      const data = this.unicodeBase64Decode(encryptedData);
      let result = '';
      for (let i = 0; i < data.length; i++) {
        result += String.fromCharCode(
          data.charCodeAt(i) ^ password.charCodeAt(i % password.length)
        );
      }
      return result;
    } catch (error) {
      throw new Error('解密失败：数据格式错误或密码不正确');
    }
  }

  /**
   * 验证密码强度
   *
   * @param password 要验证的密码
   * @returns 验证结果
   */
  validatePassword(password: string): { isValid: boolean; message?: string } {
    if (!password || password.length === 0) {
      return { isValid: false, message: '请输入密码' };
    }

    if (password.length < 4) {
      return { isValid: false, message: '密码长度至少为4个字符' };
    }

    return { isValid: true };
  }

  /**
   * 生成随机盐值（可选的安全增强功能）
   *
   * @param length 盐值长度，默认为16
   * @returns 随机盐值字符串
   */
  generateSalt(length: number = 16): string {
    const chars = 'ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789';
    let result = '';
    for (let i = 0; i < length; i++) {
      result += chars.charAt(Math.floor(Math.random() * chars.length));
    }
    return result;
  }
}

/**
 * 默认加密提供者实例
 */
export const defaultEncryptionProvider = new ConfigEncryptionProvider();