# 阿里云 OSS 兼容性实施指南

## 📋 概述

本指南提供了将阿里云 OSS 支持集成到 R2Uploader 的完整实施步骤。

---

## 🔧 实施步骤

### 步骤 1: 修改 Bucket 类型定义

**文件**: `src/lib/type.ts`

**变更**:
```typescript
export interface Bucket {
  id?: number;
  type: "r2" | "s3" | "oss";    // 添加 "oss" 类型
  bucketName: string;
  accountId: string;             // 对 OSS 可为空
  accessKey: string;
  secretKey: string;
  customDomain: string;
  s3Api?: string;
  endpoint?: string;             // 新增：S3 兼容 endpoint
  region?: string;               // 新增：区域信息（可选）
  [key: string]: string | number | undefined;
}
```

### 步骤 2: 修改 Rust 后端 - R2Client

**文件**: `src-tauri/src/r2.rs`

**修改 R2Client::new() 方法**:

```rust
impl R2Client {
    pub async fn new(
        bucket_name: &str,
        account_id: &str,
        access_key: &str,
        secret_key: &str,
        domain: Option<&str>,
        endpoint: Option<&str>,  // 新增参数
    ) -> Result<Self, String> {
        println!("new r2 client...");
        std::env::set_var("AWS_REQUEST_CHECKSUM_CALCULATION", "WHEN_REQUIRED");

        let credentials = Credentials::new(access_key, secret_key, None, None, "R2Uploader");

        let timeout_config = TimeoutConfig::builder()
            .connect_timeout(Duration::from_secs(30))
            .read_timeout(Duration::from_secs(30))
            .build();

        // 构建 endpoint URL
        let endpoint_url = if let Some(ep) = endpoint {
            // 使用自定义 endpoint（用于 OSS 或其他 S3 兼容服务）
            ep.to_string()
        } else {
            // 默认使用 R2 endpoint
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
}
```

### 步骤 3: 更新所有 Tauri 命令

**文件**: `src-tauri/src/r2.rs`

**更新 r2_ping 命令**:
```rust
#[tauri::command]
pub async fn r2_ping(
    bucket_name: &str,
    account_id: &str,
    access_key: &str,
    secret_key: &str,
    endpoint: Option<&str>,  // 新增
) -> Result<(), String> {
    let client = R2Client::new(
        bucket_name,
        account_id,
        access_key,
        secret_key,
        None,
        endpoint,  // 传递
    ).await?;
    client.ping().await
}
```

**更新 r2_upload 命令**:
```rust
#[tauri::command]
pub async fn r2_upload(
    app: AppHandle,
    bucket_name: &str,
    account_id: &str,
    access_key: &str,
    secret_key: &str,
    domain: Option<&str>,
    endpoint: Option<&str>,  // 新增
    files: Vec<File>,
) -> Result<(), String> {
    let client = Arc::new(
        R2Client::new(
            bucket_name,
            account_id,
            access_key,
            secret_key,
            domain,
            endpoint,  // 传递
        ).await?
    );
    // ... 其余代码
}
```

**更新其他命令** (r2_list_objects, r2_list_multipart_uploads, r2_delete_object, r2_abort_multipart_upload_cmd):

```rust
#[tauri::command]
pub async fn r2_list_objects(
    bucket_name: &str,
    account_id: &str,
    access_key: &str,
    secret_key: &str,
    endpoint: Option<&str>,  // 新增
    max_keys: u32,
    continuation_token: Option<String>,
) -> Result<S3ObjectListResponse, String> {
    let client = R2Client::new(
        bucket_name,
        account_id,
        access_key,
        secret_key,
        None,
        endpoint,  // 传递
    ).await?;
    client.list_objects(max_keys, continuation_token.as_deref()).await
}
```

### 步骤 4: 更新前端 - AddBucket 组件

**文件**: `src/lib/components/AddBucket.svelte`

**添加 endpoint 字段到 bucket 对象**:
```typescript
let bucket: Bucket = $state({
    type: "r2",
    bucketName: "",
    accountId: "",
    accessKey: "",
    secretKey: "",
    customDomain: "",
    s3Api: "",
    endpoint: "",  // 新增
});
```

**添加 endpoint 输入配置**:
```typescript
const inputConfigs = $state([
    {
        id: "s3Api",
        label: t().addBucket.labels.s3Api,
        focused: false,
        required: false,
        error: false,
    },
    // ... 其他字段
    {
        id: "endpoint",
        label: t().addBucket.labels.endpoint,
        focused: false,
        required: false,
        placeholder: "https://oss-cn-hangzhou.aliyuncs.com",
    },
]);
```

**更新 checkBucket 函数**:
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
        console.error(e);
    } finally {
        isChecking = false;
    }
}
```

### 步骤 5: 更新前端 - 管理页面

**文件**: `src/routes/manage/+page.svelte`

**更新 loadData 函数**:
```typescript
async function loadData() {
    if (!globalState.selectedBucket) {
        setAlert(t().common.noBucketWarning);
        return;
    }

    loading = true;
    error = null;

    try {
        const bucket = globalState.selectedBucket.value;
        
        // Load files
        const filesResponse = await invoke("r2_list_objects", {
            bucketName: bucket.bucketName,
            accountId: bucket.accountId,
            accessKey: bucket.accessKey,
            secretKey: bucket.secretKey,
            endpoint: bucket.endpoint,  // 新增
            maxKeys: pageSize,
            continuationToken: currentPage === 1 ? undefined : continuationToken,
        });
        // ... 其余代码
    }
}
```

**更新其他操作函数** (deleteFile, abortUpload):
```typescript
async function deleteFile(key: string) {
    if (!confirm(t().manage.files.deleteConfirm)) return;
    if (!globalState.selectedBucket) return;

    try {
        const bucket = globalState.selectedBucket.value;
        await invoke("r2_delete_object", {
            bucketName: bucket.bucketName,
            accountId: bucket.accountId,
            accessKey: bucket.accessKey,
            secretKey: bucket.secretKey,
            endpoint: bucket.endpoint,  // 新增
            key,
        });
        // ... 其余代码
    }
}
```

### 步骤 6: 更新前端 - 上传页面

**文件**: `src/routes/+page.svelte`

**更新 upload 函数**:
```typescript
async function upload() {
    if (!globalState.selectedBucket) {
        setAlert(t().common.noBucketWarning);
        return;
    }

    const bucket = globalState.selectedBucket.value;
    await invoke("r2_upload", {
        bucketName: bucket.bucketName,
        accountId: bucket.accountId,
        accessKey: bucket.accessKey,
        secretKey: bucket.secretKey,
        domain: bucket.customDomain,
        endpoint: bucket.endpoint,  // 新增
        files: filesToUpload,
    });
}
```

### 步骤 7: 添加国际化翻译

**文件**: `src/lib/i18n.svelte.ts`

**英文翻译**:
```typescript
export let en = $state({
    addBucket: {
        // ... 现有字段
        labels: {
            s3Api: "S3 API",
            bucketName: "Bucket Name",
            accountId: "Account ID",
            accessKey: "Access Key",
            secretKey: "Secret Key",
            customDomain: "Custom Domain, e.g. https://example.com",
            endpoint: "Endpoint (for S3-compatible services)",  // 新增
        },
    },
});
```

**中文翻译**:
```typescript
export let zh = $state({
    addBucket: {
        // ... 现有字段
        labels: {
            s3Api: "S3 API",
            bucketName: "Bucket 名称",
            accountId: "Account ID",
            accessKey: "Access Key",
            secretKey: "Secret Key",
            customDomain: "自定义域名，例如 https://example.com",
            endpoint: "Endpoint（用于 S3 兼容服务）",  // 新增
        },
    },
});
```

---

## 🧪 测试清单

- [ ] R2 连接测试（确保向后兼容）
- [ ] OSS 连接测试
- [ ] 文件上传测试
- [ ] 文件列表测试
- [ ] 文件删除测试
- [ ] 分段上传测试
- [ ] 自定义域名测试
- [ ] 代理连接测试
- [ ] 深色模式测试
- [ ] 国际化测试

---

## 📝 用户文档

### 添加阿里云 OSS 存储桶

1. 在设置页面点击"添加新存储桶"
2. 选择存储桶类型为 "OSS"
3. 输入 Bucket 名称
4. 输入 Access Key ID
5. 输入 Access Key Secret
6. 选择对应的 Endpoint（例如：https://oss-cn-hangzhou.aliyuncs.com）
7. 点击"检查"验证连接
8. 保存配置


