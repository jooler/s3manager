# 阿里云 OSS 兼容性分析与实施方案

## 📋 执行摘要

**结论**: ✅ **可以兼容支持阿里云 OSS**

当前应用已经使用 AWS SDK for S3，具有很好的 S3 兼容性。阿里云 OSS 提供 S3 兼容 API，因此可以通过以下改动支持阿里云 OSS：

1. 添加 `endpoint` 字段到存储桶配置
2. 修改 Rust 后端以支持自定义 endpoint
3. 更新前端 UI 以支持存储桶类型选择

---

## 🔍 当前应用分析

### 1. 存储桶配置字段

**当前 Bucket 接口** (`src/lib/type.ts`):
```typescript
export interface Bucket {
  id?: number;
  type: "r2" | "s3";           // ✅ 已支持类型区分
  bucketName: string;           // ✅ 存储桶名称
  accountId: string;            // ⚠️ 用于 R2 endpoint
  accessKey: string;            // ✅ 访问密钥
  secretKey: string;            // ✅ 密钥
  customDomain: string;         // ✅ 自定义域名
  s3Api?: string;               // ✅ S3 API URL
  [key: string]: string | number | undefined;
}
```

**问题**: 
- `accountId` 字段对 OSS 不适用
- 缺少通用的 `endpoint` 字段

### 2. S3 操作支持

**当前支持的操作**:
- ✅ `list_objects_v2()` - 列表对象
- ✅ `put_object()` - 上传对象
- ✅ `delete_object()` - 删除对象
- ✅ `create_multipart_upload()` - 创建分段上传
- ✅ `upload_part()` - 上传分段
- ✅ `complete_multipart_upload()` - 完成分段上传
- ✅ `abort_multipart_upload()` - 中止分段上传
- ✅ `list_multipart_uploads()` - 列表分段上传

**兼容性**: 所有操作都是标准 S3 API，阿里云 OSS 完全支持

### 3. Endpoint 配置

**当前实现** (`src-tauri/src/r2.rs`):
```rust
let mut config_loader = ConfigLoader::default()
    .region(Region::new("auto"))
    .endpoint_url(format!("https://{}.r2.cloudflarestorage.com", account_id))
    .timeout_config(timeout_config)
    .credentials_provider(credentials);
```

**问题**: Endpoint 硬编码为 R2 格式

---

## 🎯 实施方案

### 方案 1: 最小改动方案（推荐）

#### 1.1 修改 Bucket 接口

**文件**: `src/lib/type.ts`

```typescript
export interface Bucket {
  id?: number;
  type: "r2" | "s3" | "oss";    // 添加 oss 类型
  bucketName: string;
  accountId: string;             // 对 OSS 可为空或用于其他用途
  accessKey: string;
  secretKey: string;
  customDomain: string;
  s3Api?: string;
  endpoint?: string;             // 新增：自定义 endpoint
  region?: string;               // 新增：区域信息
}
```

#### 1.2 修改 Rust 后端

**文件**: `src-tauri/src/r2.rs`

修改 `R2Client::new()` 方法：

```rust
pub async fn new(
    bucket_name: &str,
    account_id: &str,
    access_key: &str,
    secret_key: &str,
    domain: Option<&str>,
    endpoint: Option<&str>,      // 新增参数
) -> Result<Self, String> {
    let credentials = Credentials::new(access_key, secret_key, None, None, "R2Uploader");
    
    let timeout_config = TimeoutConfig::builder()
        .connect_timeout(Duration::from_secs(30))
        .read_timeout(Duration::from_secs(30))
        .build();

    let endpoint_url = if let Some(ep) = endpoint {
        ep.to_string()
    } else {
        format!("https://{}.r2.cloudflarestorage.com", account_id)
    };

    let mut config_loader = ConfigLoader::default()
        .region(Region::new("auto"))
        .endpoint_url(endpoint_url)
        .timeout_config(timeout_config)
        .credentials_provider(credentials);

    if let Some(proxy_connector) = create_proxy_connector() {
        config_loader =
            config_loader.http_client(HyperClientBuilder::new().build(proxy_connector));
    }

    let config = config_loader.load().await;

    Ok(Self {
        client: Client::new(&config),
        bucket_name: bucket_name.to_string(),
        domain: domain.unwrap_or("").to_string(),
    })
}
```

#### 1.3 更新所有 Tauri 命令

所有命令需要添加 `endpoint` 参数：

```rust
#[tauri::command]
pub async fn r2_ping(
    bucket_name: &str,
    account_id: &str,
    access_key: &str,
    secret_key: &str,
    endpoint: Option<&str>,      // 新增
) -> Result<(), String> {
    let client = R2Client::new(
        bucket_name, 
        account_id, 
        access_key, 
        secret_key, 
        None,
        endpoint                  // 传递
    ).await?;
    client.ping().await
}
```

#### 1.4 更新前端调用

**文件**: `src/lib/components/AddBucket.svelte`

```typescript
async function checkBucket() {
    isChecking = true;
    errorMessage = "";
    try {
        await invoke("r2_ping", {
            bucketName: bucket.bucketName,
            accountId: bucket.accountId,
            accessKey: bucket.accessKey,
            secretKey: bucket.secretKey,
            endpoint: bucket.endpoint,  // 新增
        });
        checkResult = true;
        setAlert("success");
    } catch (e) {
        checkResult = false;
        errorMessage = e as string;
    } finally {
        isChecking = false;
    }
}
```

#### 1.5 更新 UI 表单

添加 endpoint 输入字段：

```typescript
const inputConfigs = $state([
    // ... 现有字段
    {
        id: "endpoint",
        label: t().addBucket.labels.endpoint,
        focused: false,
        required: false,
        placeholder: "https://oss-cn-hangzhou.aliyuncs.com",
    },
    // ...
]);
```

---

## 📊 阿里云 OSS 配置示例

### OSS Endpoint 列表

| 区域 | Endpoint |
|------|----------|
| 华东1（杭州） | https://oss-cn-hangzhou.aliyuncs.com |
| 华东2（上海） | https://oss-cn-shanghai.aliyuncs.com |
| 华北1（青岛） | https://oss-cn-qingdao.aliyuncs.com |
| 华北2（北京） | https://oss-cn-beijing.aliyuncs.com |
| 华北3（张家口） | https://oss-cn-zhangjiakou.aliyuncs.com |
| 华北5（呼和浩特） | https://oss-cn-huhehaote.aliyuncs.com |
| 华南1（深圳） | https://oss-cn-shenzhen.aliyuncs.com |
| 西南1（成都） | https://oss-cn-chengdu.aliyuncs.com |

### 用户配置步骤

1. 获取 Access Key ID 和 Access Key Secret
2. 选择存储桶类型为 "OSS"
3. 输入 Bucket 名称
4. 输入 Access Key ID（作为 accessKey）
5. 输入 Access Key Secret（作为 secretKey）
6. 选择对应的 Endpoint
7. 点击"检查"验证连接
8. 保存配置

---

## ✅ 兼容性检查清单

| 功能 | R2 | OSS | 状态 |
|------|----|----|------|
| 列表对象 | ✅ | ✅ | 完全兼容 |
| 上传对象 | ✅ | ✅ | 完全兼容 |
| 删除对象 | ✅ | ✅ | 完全兼容 |
| 分段上传 | ✅ | ✅ | 完全兼容 |
| 自定义域名 | ✅ | ✅ | 完全兼容 |
| 代理支持 | ✅ | ✅ | 完全兼容 |

---

## 📝 实施步骤

### 第一阶段：后端修改
1. 修改 `Bucket` 接口，添加 `endpoint` 字段
2. 修改 `R2Client::new()` 支持自定义 endpoint
3. 更新所有 Tauri 命令添加 endpoint 参数

### 第二阶段：前端修改
1. 更新 `AddBucket.svelte` 添加 endpoint 输入
2. 更新所有 invoke 调用传递 endpoint
3. 添加 i18n 翻译

### 第三阶段：测试
1. 测试 R2 连接（确保向后兼容）
2. 测试 OSS 连接
3. 测试所有操作（上传、下载、删除等）

---

## 🔐 安全考虑

- ✅ Endpoint 存储在本地数据库，不上传到服务器
- ✅ 凭证（Access Key）使用相同的加密机制
- ✅ 支持代理连接
- ✅ 支持自定义域名

---

## 📈 未来扩展

该方案可轻松扩展支持其他 S3 兼容存储：
- MinIO
- DigitalOcean Spaces
- Wasabi
- Backblaze B2
- 等等

只需在 `type` 字段中添加新类型即可。


