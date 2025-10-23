# OSS V4 签名算法实现 - 预签名 URL 修复

## 📋 问题分析

### 错误的 URL（使用 AWS S3 签名）
```
https://oss-cn-shanghai.aliyuncs.com/airspace/Team_Brand_46b7734ee5.png?
x-id=GetObject&
X-Amz-Algorithm=AWS4-HMAC-SHA256&
X-Amz-Credential=LTAI5tRcn582YUNAgVGppY4u/20251023/auto/s3/aws4_request&
X-Amz-Date=20251023T170450Z&
X-Amz-Expires=3600&
X-Amz-SignedHeaders=host&
X-Amz-Signature=...
```

**问题**:
1. 使用了 AWS S3 的签名参数（`X-Amz-*`）
2. Region 是 `auto` 而不是 `cn-shanghai`
3. Credential 格式是 `aws4_request` 而不是 `aliyun_v4_request`
4. 返回 403 错误

### 正确的 URL（使用 OSS V4 签名）
```
https://airspace.oss-cn-shanghai.aliyuncs.com/20230411-EuropeFromISS_ZH-CN0722816540_UHD.jpg?
x-oss-credential=LTAI5tRcn582YUNAgVGppY4u%2F20251023%2Fcn-shanghai%2Foss%2Faliyun_v4_request&
x-oss-date=20251023T171004Z&
x-oss-expires=3600&
x-oss-signature-version=OSS4-HMAC-SHA256&
x-oss-signature=a2c00a38c60c2bd3ab2954e51b05cf6240666c742383f8340738decbe480f0df
```

**特点**:
1. 使用 OSS 专用的签名参数（`x-oss-*`）
2. Region 是正确的 `cn-shanghai`
3. Credential 格式是 `aliyun_v4_request`
4. 签名算法是 `OSS4-HMAC-SHA256`

## 🔍 关键区别

| 项目 | AWS S3 | 阿里云 OSS |
|------|--------|-----------|
| **签名版本参数** | `X-Amz-Algorithm` | `x-oss-signature-version` |
| **签名版本值** | `AWS4-HMAC-SHA256` | `OSS4-HMAC-SHA256` |
| **凭证参数** | `X-Amz-Credential` | `x-oss-credential` |
| **凭证格式** | `access_key/date/region/s3/aws4_request` | `access_key/date/region/oss/aliyun_v4_request` |
| **日期参数** | `X-Amz-Date` | `x-oss-date` |
| **过期时间参数** | `X-Amz-Expires` | `x-oss-expires` |
| **签名参数** | `X-Amz-Signature` | `x-oss-signature` |
| **签名密钥前缀** | `AWS4` | `aliyun_v4` |

## ✅ 解决方案

### 1️⃣ 添加依赖

在 `Cargo.toml` 中添加：

```toml
hmac = "0.12"
sha2 = "0.10"
hex = "0.4"
chrono = "0.4"
urlencoding = "2.1"
```

### 2️⃣ 实现 OSS V4 签名算法

创建 `generate_oss_presigned_url` 方法，实现完整的 OSS V4 签名流程：

```rust
fn generate_oss_presigned_url(&self, key: &str, expires_in: u64) -> Result<String, String> {
    // 1. 提取 region
    let region = extract_region_from_endpoint(endpoint);
    
    // 2. 获取当前时间
    let now = Utc::now();
    let date_stamp = now.format("%Y%m%d").to_string();
    let date_time = now.format("%Y%m%dT%H%M%SZ").to_string();
    
    // 3. 构建 credential
    let credential = format!("{}/{}/{}/oss/aliyun_v4_request", access_key, date_stamp, region);
    
    // 4. 构建 canonical request
    let canonical_request = build_canonical_request(...);
    
    // 5. 构建 string to sign
    let string_to_sign = build_string_to_sign(...);
    
    // 6. 计算签名
    let signature = calculate_signature(...);
    
    // 7. 构建最终 URL
    let final_url = build_final_url(...);
    
    Ok(final_url)
}
```

### 3️⃣ 签名计算流程

#### Step 1: 构建 Canonical Request
```
GET
/{url_encoded_key}
{canonical_query_string}
host:{bucket}.{endpoint}

host
UNSIGNED-PAYLOAD
```

#### Step 2: 计算 Canonical Request Hash
```rust
let canonical_request_hash = SHA256(canonical_request);
```

#### Step 3: 构建 String to Sign
```
OSS4-HMAC-SHA256
{date_time}
{date_stamp}/{region}/oss/aliyun_v4_request
{canonical_request_hash}
```

#### Step 4: 计算签名密钥
```rust
k_date = HMAC_SHA256("aliyun_v4" + secret_key, date_stamp)
k_region = HMAC_SHA256(k_date, region)
k_service = HMAC_SHA256(k_region, "oss")
k_signing = HMAC_SHA256(k_service, "aliyun_v4_request")
```

#### Step 5: 计算最终签名
```rust
signature = HMAC_SHA256(k_signing, string_to_sign)
signature_hex = hex_encode(signature)
```

### 4️⃣ 修改 get_presigned_url 方法

```rust
pub async fn get_presigned_url(&self, key: &str, expires_in: u64) -> Result<String, String> {
    let is_oss = self.endpoint.as_ref().map_or(false, |ep| ep.contains("aliyuncs.com"));

    if is_oss {
        // OSS 使用自定义的签名算法
        self.generate_oss_presigned_url(key, expires_in)
    } else {
        // R2 使用 AWS SDK 的预签名 URL
        // ... AWS SDK 代码 ...
    }
}
```

## 📊 URL 格式对比

### R2 预签名 URL
```
https://bucket.account-id.r2.cloudflarestorage.com/image.jpg?
X-Amz-Algorithm=AWS4-HMAC-SHA256&
X-Amz-Credential=...&
X-Amz-Date=...&
X-Amz-Expires=3600&
X-Amz-SignedHeaders=host&
X-Amz-Signature=...
```

### OSS 预签名 URL
```
https://bucket.oss-cn-shanghai.aliyuncs.com/image.jpg?
x-oss-credential=...&
x-oss-date=...&
x-oss-expires=3600&
x-oss-signature-version=OSS4-HMAC-SHA256&
x-oss-signature=...
```

## 📝 文件修改清单

| 文件 | 操作 | 说明 |
|------|------|------|
| `src-tauri/Cargo.toml` | ✅ 修改 | 添加加密和编码依赖 |
| `src-tauri/src/r2.rs` | ✅ 修改 | 添加 imports |
| `src-tauri/src/r2.rs` | ✅ 新增 | `generate_oss_presigned_url` 方法 |
| `src-tauri/src/r2.rs` | ✅ 修改 | 简化 `get_presigned_url` 方法 |

## ✅ 编译状态

- ✅ 后端：编译成功
- ⚠️ 警告：`account_id` 字段未使用（可忽略）
- ✅ 代码质量良好

## 🧪 测试清单

### R2 测试
- [ ] 文件列表加载
- [ ] 上传文件
- [ ] 删除文件
- [ ] 预览图片（应该正常工作）
- [ ] 验证预签名 URL 使用 AWS S3 签名

### OSS 测试
- [ ] 文件列表加载
- [ ] 上传文件
- [ ] 删除文件
- [ ] 预览图片（应该正常工作）
- [ ] 验证预签名 URL 使用 OSS V4 签名
- [ ] 验证 URL 参数包含 `x-oss-*`
- [ ] 验证 credential 包含 `aliyun_v4_request`
- [ ] 验证 region 正确（如 `cn-shanghai`）

## 🔧 实现细节

### Region 提取
```rust
// 从 "oss-cn-shanghai.aliyuncs.com" 提取 "cn-shanghai"
let region = endpoint_host
    .split('.')
    .next()
    .and_then(|s| s.strip_prefix("oss-"))
    .unwrap_or("auto");
```

### URL 编码
```rust
// 对 key 进行 URL 编码
let canonical_uri = format!("/{}", urlencoding::encode(key));
```

### 查询参数排序
```rust
// 按字母顺序排序查询参数
let mut query_params = vec![
    ("x-oss-credential", ...),
    ("x-oss-date", ...),
    ("x-oss-expires", ...),
    ("x-oss-signature-version", ...),
];
query_params.sort_by(|a, b| a.0.cmp(&b.0));
```

### HMAC 链式计算
```rust
type HmacSha256 = Hmac<Sha256>;

let k_date = HmacSha256::new_from_slice(format!("aliyun_v4{}", secret_key).as_bytes())
    .chain_update(date_stamp.as_bytes())
    .finalize()
    .into_bytes();

let k_region = HmacSha256::new_from_slice(&k_date)
    .chain_update(region.as_bytes())
    .finalize()
    .into_bytes();

// ... 继续链式计算 ...
```

## ✨ 特点总结

- ✅ **完整实现 OSS V4 签名算法**
- ✅ **自动检测服务类型**：根据 endpoint 判断是 R2 还是 OSS
- ✅ **正确的签名参数**：使用 `x-oss-*` 而不是 `X-Amz-*`
- ✅ **正确的 region**：从 endpoint 自动提取
- ✅ **正确的 credential 格式**：使用 `aliyun_v4_request`
- ✅ **Virtual-Hosted-Style URL**：使用 `bucket.endpoint` 格式
- ✅ **不影响其他操作**：只针对预签名 URL 做特殊处理

## 🎯 为什么不能使用 AWS SDK？

1. **签名参数不同**：OSS 使用 `x-oss-*`，AWS 使用 `X-Amz-*`
2. **签名算法不同**：OSS 使用 `OSS4-HMAC-SHA256`，AWS 使用 `AWS4-HMAC-SHA256`
3. **Credential 格式不同**：OSS 使用 `aliyun_v4_request`，AWS 使用 `aws4_request`
4. **签名密钥前缀不同**：OSS 使用 `aliyun_v4`，AWS 使用 `AWS4`

虽然 OSS 声称兼容 S3 API，但在预签名 URL 方面，它使用的是自己的签名算法，与 AWS S3 不兼容。

## 📚 参考资料

- 阿里云 OSS 文档：https://help.aliyun.com/zh/oss/
- OSS S3 兼容 API：https://help.aliyun.com/zh/oss/developer-reference/
- AWS S3 签名 V4：https://docs.aws.amazon.com/AmazonS3/latest/API/sig-v4-authenticating-requests.html


