/**
 * 对邮箱地址进行打码处理
 * 保留@符号前的首字符和@符号后的域名
 * @param email 邮箱地址
 * @returns 打码后的邮箱
 */
export const maskUsername = (email: string): string => {
  if (!email) {
    return email;
  }

  const atIndex = email.lastIndexOf('@');
  if (atIndex <= 0 || atIndex >= email.length - 1) {
    return email; // 不是有效的邮箱格式，直接返回
  }

  const localPart = email.substring(0, atIndex);
  const domainPart = email.substring(atIndex);

  // @符号前只有1个字符，不打码
  if (localPart.length <= 2) {
    return email;
  }

  // @符号前有2个字符，显示首字符 + *
  if (localPart.length === 2) {
    return localPart.charAt(0) + '*' + domainPart;
  }

  // @符号前超过2个字符，显示首字符 + * + 尾字符
  let stars = '';
  for (let i = 0; i < localPart.length - 2; i++) {
    stars += '*';
  }
  return localPart.charAt(0) + stars + localPart.charAt(localPart.length - 1) + domainPart;
};

/**
 * 对备份文件名（邮箱）进行打码处理
 * @param backupFile 备份文件名（邮箱地址）
 * @returns 打码后的备份文件名
 */
export const maskBackupFilename = (backupFile: string): string => {
  if (!backupFile) {
    return backupFile;
  }

  // 直接对整个文件名（邮箱）进行打码
  return maskUsername(backupFile);
};
