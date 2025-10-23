# Cloudflare R2 配置指南

## 连接设置

1. 获取 Cloudflare R2 S3 API URL：
   - 进入 R2 存储桶的设置页面
   - 在"存储桶详细信息"中找到"S3 API"部分
   - 复制 S3 API URL，格式如下：
   ```
   https://[accountId].r2.cloudflarestorage.com/[bucketName]
   ```
   程序将自动从此 URL 中提取 `Bucket Name` 和 `Account ID`。

2. 创建 API 令牌，步骤如下：
   - 访问 [Cloudflare R2 API 令牌页面](https://dash.cloudflare.com/?to=/:account/r2/api-tokens)
   - 创建新的令牌，确保至少具有对象的读写权限
   - 复制"访问密钥 ID"并粘贴到"Access Key"字段
   - 复制"机密访问密钥"并粘贴到"Secret Key"字段

3. （可选）配置自定义域名：
   - 如果您为 R2 存储桶配置了自定义域名，请在"Custom Domain"字段中输入
   - 上传文件后生成的分享链接将使用此域名

## 验证与保存

1. 点击"Check"按钮验证存储桶连接
2. 连接测试通过后，点击"Save"保存配置
