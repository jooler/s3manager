# OSS 预签名 URL 修复 - 独立处理方案

## 📋 问题回顾

1. **修改前**: 文件列表功能正常工作
2. **修改后**: 为了支持 OSS 预签名 URL，修改了全局 S3 客户端配置（`force_path_style(true)`）
3. **新问题**: OSS 文件列表加载失败（"Failed to load data"）

## 🔍 根本原因

**错误的方案**: 在全局 S3 客户端配置中强制使用 Path-Style
- ❌ 影响了所有 S3 操作（list_objects, upload, delete 等）
- ❌ 破坏了原本正常工作的功能
- ❌ R2 和 OSS 的其他操作可能不兼容 Path-Style

**正确的方案**: 只在生成预签名 URL 时针对 OSS 使用 Path-Style
- ✅ 不影响其他操作
- ✅ 保持原有功能正常工作
- ✅ 只针对预签名 URL 做特殊处理

## ✅ 解决方案

### 核心思路

1. **恢复原始配置**: 将 S3 客户端配置恢复到修改前的状态
2. **独立处理预签名 URL**: 在 `get_presigned_url` 方法中判断是否是 OSS
3. **按需创建客户端**: 为 OSS 预签名 URL 创建临时的 path-style 客户端

### 实现步骤

#### 1️⃣ 修改 R2Client 结构体

添加必要的字段以便在生成预签名 URL 时使用：

```rust
#[derive(Clone)]
pub struct R2Client {
    client: Client,
    bucket_name: String,
    domain: String,
    endpoint: Option<String>,      // 新增：存储 endpoint
    access_key: String,            // 新增：存储 access_key
    secret_key: String,            // 新增：存储 secret_key
    account_id: String,            // 新增：存储 account_id
}
```

#### 2️⃣ 恢复原始的 new_with_endpoint 方法

```rust
pub async fn new_with_endpoint(
    bucket_name: &str,
    account_id: &str,
    access_key: &str,
    secret_key: &str,
    domain: Option<&str>,
    endpoint: Option<&str>,
) -> Result<Self, String> {
    // ... 配置代码 ...

    let config = config_loader.load().await;

    Ok(Self {
        client: Client::new(&config),  // 使用默认配置，不强制 path-style
        bucket_name: bucket_name.to_string(),
        domain: domain.unwrap_or("").to_string(),
        endpoint: endpoint.map(|s| s.to_string()),
        access_key: access_key.to_string(),
        secret_key: secret_key.to_string(),
        account_id: account_id.to_string(),
    })
}
```

#### 3️⃣ 修改 get_presigned_url 方法

根据是否是 OSS 来使用不同的 URL 生成方案：

```rust
pub async fn get_presigned_url(&self, key: &str, expires_in: u64) -> Result<String, String> {
    // 判断是否是 OSS（通过 endpoint 是否包含 "aliyuncs.com"）
    let is_oss = self.endpoint.as_ref().map_or(false, |ep| ep.contains("aliyuncs.com"));

    if is_oss {
        // OSS 需要使用 path-style，创建一个专门的客户端
        let credentials = Credentials::new(
            &self.access_key,
            &self.secret_key,
            None,
            None,
            "R2Uploader",
        );

        let endpoint_url = self.endpoint.as_ref().unwrap().clone();

        let config = ConfigLoader::default()
            .region(Region::new("auto"))
            .endpoint_url(endpoint_url)
            .credentials_provider(credentials)
            .load()
            .await;

        // 为 OSS 创建使用 path-style 的客户端配置
        let s3_config = aws_sdk_s3::config::Builder::from(&config)
            .force_path_style(true)
            .build();

        let oss_client = Client::from_conf(s3_config);

        let presigning_config = aws_sdk_s3::presigning::PresigningConfig::builder()
            .expires_in(std::time::Duration::from_secs(expires_in))
            .build()
            .map_err(|e| e.to_string())?;

        let presigned_request = oss_client
            .get_object()
            .bucket(&self.bucket_name)
            .key(key)
            .presigned(presigning_config)
            .await
            .map_err(|e| e.to_string())?;

        Ok(presigned_request.uri().to_string())
    } else {
        // R2 使用默认的 virtual-hosted-style
        let presigning_config = aws_sdk_s3::presigning::PresigningConfig::builder()
            .expires_in(std::time::Duration::from_secs(expires_in))
            .build()
            .map_err(|e| e.to_string())?;

        let presigned_request = self
            .client
            .get_object()
            .bucket(&self.bucket_name)
            .key(key)
            .presigned(presigning_config)
            .await
            .map_err(|e| e.to_string())?;

        Ok(presigned_request.uri().to_string())
    }
}
```

## 📊 方案对比

| 方案 | 优点 | 缺点 | 结果 |
|------|------|------|------|
| **全局 Path-Style** | 简单 | 破坏其他功能 | ❌ 不可行 |
| **独立处理预签名 URL** | 不影响其他功能 | 稍微复杂 | ✅ 推荐 |

## 🔄 工作流程

### R2 预签名 URL
```
用户点击预览
    ↓
调用 get_presigned_url()
    ↓
检测到不是 OSS
    ↓
使用默认客户端（virtual-hosted-style）
    ↓
生成预签名 URL
    ↓
返回 URL
```

### OSS 预签名 URL
```
用户点击预览
    ↓
调用 get_presigned_url()
    ↓
检测到是 OSS（endpoint 包含 "aliyuncs.com"）
    ↓
创建临时 path-style 客户端
    ↓
使用临时客户端生成预签名 URL
    ↓
返回 URL
```

### 其他操作（list_objects, upload, delete）
```
调用相应方法
    ↓
使用默认客户端（virtual-hosted-style）
    ↓
执行操作
    ↓
返回结果
```

## 📝 文件修改清单

| 文件 | 操作 | 说明 |
|------|------|------|
| `src-tauri/src/r2.rs` | ✅ 修改 | R2Client 结构体添加字段 |
| `src-tauri/src/r2.rs` | ✅ 修改 | 恢复 new_with_endpoint 方法 |
| `src-tauri/src/r2.rs` | ✅ 修改 | 修改 get_presigned_url 方法 |

## ✅ 编译状态

- ✅ 后端：编译成功
- ⚠️ 警告：`account_id` 字段未使用（可忽略）
- ✅ 代码质量良好

## 🧪 测试清单

### R2 测试
- [ ] 文件列表加载（应该正常工作）
- [ ] 上传文件（应该正常工作）
- [ ] 删除文件（应该正常工作）
- [ ] 预览图片（应该正常工作）
- [ ] 验证预签名 URL 格式（virtual-hosted-style）

### OSS 测试
- [ ] 文件列表加载（应该恢复正常）
- [ ] 上传文件（应该正常工作）
- [ ] 删除文件（应该正常工作）
- [ ] 预览图片（应该正常工作）
- [ ] 验证预签名 URL 格式（path-style）

## 📚 URL 格式示例

### R2 预签名 URL（Virtual-Hosted-Style）
```
https://bucket.account-id.r2.cloudflarestorage.com/image.jpg?
X-Amz-Algorithm=AWS4-HMAC-SHA256&
X-Amz-Credential=...&
X-Amz-Signature=...
```

### OSS 预签名 URL（Path-Style）
```
https://oss-cn-shanghai.aliyuncs.com/bucket/image.jpg?
X-Amz-Algorithm=AWS4-HMAC-SHA256&
X-Amz-Credential=...&
X-Amz-Signature=...
```

## ✨ 特点总结

- ✅ **不影响现有功能**：文件列表、上传、删除等操作保持不变
- ✅ **针对性修复**：只针对预签名 URL 做特殊处理
- ✅ **自动检测**：根据 endpoint 自动判断是 R2 还是 OSS
- ✅ **向后兼容**：不破坏现有代码
- ✅ **易于维护**：逻辑清晰，易于理解

## 🎯 关键点

1. **不要修改全局配置**：全局配置会影响所有操作
2. **按需创建客户端**：只在需要时创建特殊配置的客户端
3. **自动检测服务类型**：通过 endpoint 判断是 R2 还是 OSS
4. **保持原有功能**：确保修改不影响现有功能


