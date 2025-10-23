import OSS from 'ali-oss';
import type { Bucket } from './type';

/**
 * 创建 OSS 客户端
 * @param bucket 存储桶配置
 * @returns OSS 客户端实例
 */
export function createOSSClient(bucket: Bucket): OSS | null {
  // 检查是否是 OSS 存储桶
  if (!bucket.endpoint || !bucket.endpoint.includes('aliyuncs.com')) {
    return null;
  }

  // 从 endpoint 中提取 region
  // 例如：https://oss-cn-shanghai.aliyuncs.com -> oss-cn-shanghai
  const region = extractRegionFromEndpoint(bucket.endpoint);

  if (!region) {
    console.error('Failed to extract region from endpoint:', bucket.endpoint);
    return null;
  }

  try {
    const client = new OSS({
      region,
      // 开启 V4 版本签名
      authorizationV4: true,
      accessKeyId: bucket.accessKey,
      accessKeySecret: bucket.secretKey,
      bucket: bucket.bucketName,
      // 如果有自定义域名，可以设置
      // endpoint: bucket.domain || undefined,
    });

    console.log('OSS client created:', {
      region,
      bucket: bucket.bucketName,
      authorizationV4: true,
    });

    return client;
  } catch (error) {
    console.error('Failed to create OSS client:', error);
    return null;
  }
}

/**
 * 从 endpoint 中提取 region
 * @param endpoint OSS endpoint
 * @returns region 字符串
 */
function extractRegionFromEndpoint(endpoint: string): string | null {
  // 移除协议前缀
  const host = endpoint
    .replace('https://', '')
    .replace('http://', '');

  // 从 "oss-cn-shanghai.aliyuncs.com" 提取 "oss-cn-shanghai"
  const parts = host.split('.');
  if (parts.length > 0 && parts[0].startsWith('oss-')) {
    return parts[0];
  }

  return null;
}

/**
 * 生成 OSS 预签名 URL
 * @param bucket 存储桶配置
 * @param key 文件路径
 * @param expiresIn 过期时间（秒），默认 3600 秒（1 小时）
 * @returns 预签名 URL
 */
export async function generateOSSPresignedUrl(
  bucket: Bucket,
  key: string,
  expiresIn: number = 3600
): Promise<string> {
  const client = createOSSClient(bucket);

  if (!client) {
    throw new Error('Failed to create OSS client');
  }

  try {
    // 使用 OSS SDK 生成预签名 URL
    const url = client.signatureUrl(key, {
      expires: expiresIn,
      method: 'GET',
    });

    console.log('Generated OSS presigned URL:', {
      key,
      expiresIn,
      url,
    });

    return url;
  } catch (error) {
    console.error('Failed to generate OSS presigned URL:', error);
    throw error;
  }
}

/**
 * 检查是否是 OSS 存储桶
 * @param bucket 存储桶配置
 * @returns 是否是 OSS 存储桶
 */
export function isOSSBucket(bucket: Bucket): boolean {
  return !!(bucket.endpoint && bucket.endpoint.includes('aliyuncs.com'));
}

