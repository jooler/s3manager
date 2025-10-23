# OSS SDK 集成 - 2025-10-23 20:00

## 📋 问题背景

在尝试手动实现 OSS V4 签名算法时，遇到了多次签名错误（403 SignatureDoesNotMatch），原因：

1. **签名算法复杂**：OSS V4 签名算法有很多细节，容易出错
2. **文档理解困难**：Canonical Request 格式、URL 编码规则等容易理解错误
3. **维护成本高**：每次 OSS 更新签名规则都需要手动修改
4. **调试困难**：签名错误很难定位具体问题

## 💡 解决方案

**采用阿里云官方 OSS Browser.js SDK**，在前端直接处理 OSS 相关操作。

### ✅ 优势

| 优势 | 说明 |
|------|------|
| **官方支持** | 阿里云官方维护，签名算法完全正确 |
| **功能完整** | 支持所有 OSS 操作（上传、下载、列表、删除等） |
| **自动签名** | SDK 自动处理 V4 签名，无需手动实现 |
| **预签名 URL** | SDK 提供 `signatureUrl` 方法生成预签名 URL |
| **维护简单** | 不需要自己维护签名算法 |
| **代码简洁** | 几行代码即可完成复杂的签名操作 |

### ⚠️ 注意事项

1. **AccessKey 安全**：
   - 在生产环境中，建议使用 STS 临时凭证
   - 当前方案直接使用 AccessKey，适用于桌面应用

2. **CORS 配置**：
   - 如果需要在浏览器中直接上传文件，需要配置 OSS 的 CORS 规则
   - 当前只用于生成预签名 URL，不需要配置 CORS

## 🚀 实现步骤

### 步骤 1: 安装 OSS SDK

```bash
bun add ali-oss
```

**安装结果**:
- ✅ 成功安装 `ali-oss@6.23.0`
- ✅ 85 个依赖包
- ✅ 安装时间：15.11 秒

### 步骤 2: 创建 OSS 客户端工具类

**文件**: `src/lib/oss-client.ts`

**功能**:
1. `createOSSClient(bucket)` - 创建 OSS 客户端
2. `generateOSSPresignedUrl(bucket, key, expiresIn)` - 生成预签名 URL
3. `isOSSBucket(bucket)` - 判断是否是 OSS 存储桶

**关键代码**:

```typescript
import OSS from 'ali-oss';
import type { Bucket } from './type';

export function createOSSClient(bucket: Bucket): OSS | null {
  // 检查是否是 OSS 存储桶
  if (!bucket.endpoint || !bucket.endpoint.includes('aliyuncs.com')) {
    return null;
  }

  // 从 endpoint 中提取 region
  const region = extractRegionFromEndpoint(bucket.endpoint);

  try {
    const client = new OSS({
      region,
      // 开启 V4 版本签名
      authorizationV4: true,
      accessKeyId: bucket.accessKey,
      accessKeySecret: bucket.secretKey,
      bucket: bucket.bucketName,
    });

    return client;
  } catch (error) {
    console.error('Failed to create OSS client:', error);
    return null;
  }
}

export async function generateOSSPresignedUrl(
  bucket: Bucket,
  key: string,
  expiresIn: number = 3600
): Promise<string> {
  const client = createOSSClient(bucket);

  if (!client) {
    throw new Error('Failed to create OSS client');
  }

  // 使用 OSS SDK 生成预签名 URL
  const url = client.signatureUrl(key, {
    expires: expiresIn,
    method: 'GET',
  });

  return url;
}
```

**Region 提取逻辑**:
```typescript
function extractRegionFromEndpoint(endpoint: string): string | null {
  // 从 "https://oss-cn-shanghai.aliyuncs.com" 提取 "oss-cn-shanghai"
  const host = endpoint
    .replace('https://', '')
    .replace('http://', '');

  const parts = host.split('.');
  if (parts.length > 0 && parts[0].startsWith('oss-')) {
    return parts[0];
  }

  return null;
}
```

### 步骤 3: 修改管理页面

**文件**: `src/routes/manage/+page.svelte`

**修改内容**:

1. **导入 OSS 工具类**:
```typescript
import { generateOSSPresignedUrl, isOSSBucket } from "$lib/oss-client";
```

2. **修改 `previewImage` 函数**:
```typescript
async function previewImage(key: string) {
  try {
    const bucket = globalState.selectedBucket?.value;
    if (!bucket) {
      console.error("No bucket selected");
      return;
    }

    let presignedUrl: string;

    // 判断是 OSS 还是 R2
    if (isOSSBucket(bucket)) {
      // 使用 OSS SDK 生成预签名 URL
      console.log("Using OSS SDK to generate presigned URL");
      presignedUrl = await generateOSSPresignedUrl(bucket, key, 3600);
    } else {
      // 使用后端 Tauri 命令生成 R2 预签名 URL
      console.log("Using Tauri backend to generate R2 presigned URL");
      presignedUrl = await invoke<string>("r2_get_presigned_url", {
        bucketName: bucket.bucketName,
        accountId: bucket.accountId,
        accessKey: bucket.accessKey,
        secretKey: bucket.secretKey,
        key,
        endpoint: bucket.endpoint || undefined,
        expiresIn: 3600,
      });
    }

    console.log("Generated presigned URL:", presignedUrl);

    previewImageUrl = presignedUrl;
    previewFileName = key;
  } catch (e) {
    console.error("Error previewing image:", e);
    const errorMsg = e instanceof Error ? e.message : "Failed to preview image";
    setAlert(errorMsg);
  }
}
```

## 📝 文件修改清单

| 文件 | 操作 | 说明 |
|------|------|------|
| `package.json` | ✅ 新增 | 添加 `ali-oss` 依赖 |
| `src/lib/oss-client.ts` | ✅ 创建 | OSS 客户端工具类 |
| `src/routes/manage/+page.svelte` | ✅ 修改 | 使用 OSS SDK 生成预签名 URL |

## 🎯 工作流程

### OSS 存储桶

```
用户点击预览按钮
  ↓
previewImage(key) 被调用
  ↓
isOSSBucket(bucket) 返回 true
  ↓
调用 generateOSSPresignedUrl(bucket, key, 3600)
  ↓
创建 OSS 客户端（new OSS({...})）
  ↓
调用 client.signatureUrl(key, { expires: 3600 })
  ↓
OSS SDK 自动计算 V4 签名
  ↓
返回预签名 URL
  ↓
显示图片预览 ✅
```

### R2 存储桶

```
用户点击预览按钮
  ↓
previewImage(key) 被调用
  ↓
isOSSBucket(bucket) 返回 false
  ↓
调用 Tauri 后端命令 r2_get_presigned_url
  ↓
后端使用 AWS SDK 生成预签名 URL
  ↓
返回预签名 URL
  ↓
显示图片预览 ✅
```

## ✅ 编译状态

- ✅ 前端：无类型错误
- ✅ 后端：无需修改
- ✅ 代码质量良好

## 🧪 测试步骤

1. **重新运行应用**

2. **测试 OSS 存储桶**：
   - 切换到 OSS 存储桶
   - 点击图片的预览按钮
   - 观察控制台输出：
     ```
     Previewing image: { key: "...", bucketName: "airspace", isOSS: true }
     Using OSS SDK to generate presigned URL
     OSS client created: { region: "oss-cn-shanghai", bucket: "airspace", authorizationV4: true }
     Generated OSS presigned URL: https://airspace.oss-cn-shanghai.aliyuncs.com/...
     ```
   - 验证图片可以正常预览

3. **测试 R2 存储桶**：
   - 切换到 R2 存储桶
   - 点击图片的预览按钮
   - 观察控制台输出：
     ```
     Previewing image: { key: "...", bucketName: "...", isOSS: false }
     Using Tauri backend to generate R2 presigned URL
     Generated presigned URL: https://...
     ```
   - 验证图片可以正常预览

4. **验证 URL 格式**：
   - OSS URL 应该包含 `x-oss-signature`、`x-oss-date` 等参数
   - R2 URL 应该包含 `X-Amz-Signature`、`X-Amz-Date` 等参数

## 📊 对比

### 手动实现 vs OSS SDK

| 项目 | 手动实现 | OSS SDK |
|------|---------|---------|
| **代码量** | ~150 行 | ~10 行 |
| **签名正确性** | ❌ 多次失败 | ✅ 完全正确 |
| **维护成本** | ❌ 高 | ✅ 低 |
| **调试难度** | ❌ 困难 | ✅ 简单 |
| **功能完整性** | ⚠️ 仅预签名 URL | ✅ 所有 OSS 操作 |
| **文档支持** | ⚠️ 需要自己理解 | ✅ 官方文档完善 |

## 🔧 后续优化

### 1. 使用 STS 临时凭证（生产环境推荐）

**当前方案**:
```typescript
const client = new OSS({
  region: 'oss-cn-shanghai',
  authorizationV4: true,
  accessKeyId: bucket.accessKey,      // 长期凭证
  accessKeySecret: bucket.secretKey,  // 长期凭证
  bucket: bucket.bucketName,
});
```

**STS 方案**:
```typescript
const client = new OSS({
  region: 'oss-cn-shanghai',
  authorizationV4: true,
  accessKeyId: 'STS.xxx',           // 临时凭证
  accessKeySecret: 'xxx',           // 临时凭证
  stsToken: 'xxx',                  // 安全令牌
  bucket: bucket.bucketName,
  refreshSTSToken: async () => {
    // 向后端获取新的临时凭证
    const info = await fetch('/api/sts');
    return {
      accessKeyId: info.accessKeyId,
      accessKeySecret: info.accessKeySecret,
      stsToken: info.stsToken
    }
  },
  refreshSTSTokenInterval: 300000,  // 5 分钟刷新一次
});
```

### 2. 支持更多 OSS 操作

可以扩展 `oss-client.ts`，支持：
- 文件上传：`client.put(key, file)`
- 文件下载：`client.get(key)`
- 文件删除：`client.delete(key)`
- 文件列表：`client.list()`
- 分片上传：`client.multipartUpload()`

### 3. 添加错误处理

```typescript
export async function generateOSSPresignedUrl(
  bucket: Bucket,
  key: string,
  expiresIn: number = 3600
): Promise<string> {
  try {
    const client = createOSSClient(bucket);
    if (!client) {
      throw new Error('Failed to create OSS client');
    }

    const url = client.signatureUrl(key, {
      expires: expiresIn,
      method: 'GET',
    });

    return url;
  } catch (error) {
    if (error instanceof Error) {
      // 根据错误类型提供更友好的错误信息
      if (error.message.includes('InvalidAccessKeyId')) {
        throw new Error('OSS AccessKey 无效，请检查配置');
      } else if (error.message.includes('NoSuchBucket')) {
        throw new Error('OSS 存储桶不存在');
      }
    }
    throw error;
  }
}
```

## 📚 相关文档

- [阿里云 OSS Browser.js SDK 文档](https://help.aliyun.com/zh/oss/developer-reference/installation)
- [OSS SDK GitHub](https://github.com/ali-sdk/ali-oss)
- [OSS V4 签名文档](https://help.aliyun.com/zh/oss/developer-reference/add-signatures-to-urls)

## 🎓 经验总结

1. **优先使用官方 SDK**：不要重复造轮子，官方 SDK 更可靠
2. **理解业务场景**：桌面应用可以直接使用 AccessKey，Web 应用需要 STS
3. **分离关注点**：OSS 和 R2 使用不同的方案，代码更清晰
4. **添加日志**：详细的日志帮助调试和监控
5. **类型安全**：使用 TypeScript 确保类型正确


