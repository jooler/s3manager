# OSS Canonical Request 格式修复

## 📋 问题分析

### OSS 返回的错误信息

```xml
<Error>
  <Code>SignatureDoesNotMatch</Code>
  <Message>The request signature we calculated does not match the signature you provided.</Message>
  <CanonicalRequest>GET /airspace/Task_chat_CN.png x-oss-credential=...&x-oss-date=...&x-oss-expires=...&x-oss-signature-version=... UNSIGNED-PAYLOAD</CanonicalRequest>
  <StringToSign>OSS4-HMAC-SHA256 20251023T171529Z 20251023/cn-shanghai/oss/aliyun_v4_request 8e00c982696269df64eec6ea7c556c789c8f50601e28bd9cb0517eb30bad444b</StringToSign>
</Error>
```

### 问题所在

从 OSS 返回的 `CanonicalRequest` 可以看出，正确的格式应该是：

```
GET
/airspace/Task_chat_CN.png
x-oss-credential=...&x-oss-date=...&x-oss-expires=...&x-oss-signature-version=...

UNSIGNED-PAYLOAD
```

**我们之前的格式**（错误）:
```rust
let canonical_request = format!(
    "GET\n{}\n{}\n{}\n{}\nUNSIGNED-PAYLOAD",
    canonical_uri,           // /airspace/Task_chat_CN.png
    canonical_query_string,  // x-oss-credential=...
    canonical_headers,       // host:bucket.endpoint\n
    signed_headers          // host
);
```

这会生成：
```
GET
/airspace/Task_chat_CN.png
x-oss-credential=...
host:bucket.endpoint

host
UNSIGNED-PAYLOAD
```

**正确的格式**:
```rust
let canonical_request = format!(
    "GET\n{}\n{}\n\n\nUNSIGNED-PAYLOAD",
    canonical_uri,           // /airspace/Task_chat_CN.png
    canonical_query_string   // x-oss-credential=...
);
```

这会生成：
```
GET
/airspace/Task_chat_CN.png
x-oss-credential=...


UNSIGNED-PAYLOAD
```

## 🔍 关键区别

### AWS S3 Canonical Request（普通请求）
```
HTTP-Verb
Canonical-URI
Canonical-Query-String
Canonical-Headers
Signed-Headers
Hashed-Payload
```

示例：
```
GET
/image.jpg
x-amz-date=20251023T120000Z
host:bucket.s3.amazonaws.com
x-amz-date:20251023T120000Z

host;x-amz-date
UNSIGNED-PAYLOAD
```

### OSS 预签名 URL Canonical Request
```
HTTP-Verb
Canonical-URI
Canonical-Query-String


UNSIGNED-PAYLOAD
```

示例：
```
GET
/image.jpg
x-oss-credential=...&x-oss-date=...&x-oss-expires=...


UNSIGNED-PAYLOAD
```

**关键点**:
1. ❌ **不包含** Canonical-Headers
2. ❌ **不包含** Signed-Headers
3. ✅ **包含** 两个空行（表示没有 headers）
4. ✅ **包含** UNSIGNED-PAYLOAD

## ✅ 解决方案

### 修改前
```rust
// 构建 canonical request
let canonical_headers = format!("host:{}\n", host);
let signed_headers = "host";
let canonical_request = format!(
    "GET\n{}\n{}\n{}\n{}\nUNSIGNED-PAYLOAD",
    canonical_uri, canonical_query_string, canonical_headers, signed_headers
);
```

### 修改后
```rust
// 构建 canonical request
// 注意：OSS 预签名 URL 的 canonical request 格式与普通请求不同
// 格式：HTTP-Verb\nCanonical-URI\nCanonical-Query-String\n\n\nUNSIGNED-PAYLOAD
let canonical_request = format!(
    "GET\n{}\n{}\n\n\nUNSIGNED-PAYLOAD",
    canonical_uri, canonical_query_string
);
```

## 📊 Canonical Request 对比

### 错误的格式（包含 headers）
```
GET
/airspace/Task_chat_CN.png
x-oss-credential=LTAI5tRcn582YUNAgVGppY4u%2F20251023%2Fcn-shanghai%2Foss%2Faliyun_v4_request&x-oss-date=20251023T171529Z&x-oss-expires=3600&x-oss-signature-version=OSS4-HMAC-SHA256
host:airspace.oss-cn-shanghai.aliyuncs.com

host
UNSIGNED-PAYLOAD
```

### 正确的格式（不包含 headers）
```
GET
/airspace/Task_chat_CN.png
x-oss-credential=LTAI5tRcn582YUNAgVGppY4u%2F20251023%2Fcn-shanghai%2Foss%2Faliyun_v4_request&x-oss-date=20251023T171529Z&x-oss-expires=3600&x-oss-signature-version=OSS4-HMAC-SHA256


UNSIGNED-PAYLOAD
```

## 🔧 完整的签名流程

### Step 1: 构建 Canonical Request
```rust
let canonical_request = format!(
    "GET\n{}\n{}\n\n\nUNSIGNED-PAYLOAD",
    canonical_uri,          // /airspace/Task_chat_CN.png
    canonical_query_string  // x-oss-credential=...&x-oss-date=...
);
```

### Step 2: 计算 Canonical Request Hash
```rust
let canonical_request_hash = SHA256(canonical_request);
// 结果：8e00c982696269df64eec6ea7c556c789c8f50601e28bd9cb0517eb30bad444b
```

### Step 3: 构建 String to Sign
```rust
let string_to_sign = format!(
    "OSS4-HMAC-SHA256\n{}\n{}\n{}",
    date_time,                // 20251023T171529Z
    scope,                    // 20251023/cn-shanghai/oss/aliyun_v4_request
    canonical_request_hash    // 8e00c982...
);
```

结果：
```
OSS4-HMAC-SHA256
20251023T171529Z
20251023/cn-shanghai/oss/aliyun_v4_request
8e00c982696269df64eec6ea7c556c789c8f50601e28bd9cb0517eb30bad444b
```

### Step 4: 计算签名
```rust
k_date = HMAC_SHA256("aliyun_v4" + secret_key, "20251023")
k_region = HMAC_SHA256(k_date, "cn-shanghai")
k_service = HMAC_SHA256(k_region, "oss")
k_signing = HMAC_SHA256(k_service, "aliyun_v4_request")
signature = HMAC_SHA256(k_signing, string_to_sign)
```

## 📝 文件修改清单

| 文件 | 操作 | 说明 |
|------|------|------|
| `src-tauri/src/r2.rs` | ✅ 修改 | 移除 canonical headers 和 signed headers |

## ✅ 编译状态

- ✅ 后端：编译成功
- ⚠️ 警告：`account_id` 字段未使用（可忽略）
- ✅ 代码质量良好

## 🧪 测试清单

### OSS 预签名 URL 测试
- [ ] 生成预签名 URL
- [ ] 验证 URL 格式正确
- [ ] 验证 canonical request 不包含 headers
- [ ] 验证签名计算正确
- [ ] 访问 URL 不返回 403 错误
- [ ] 图片可以正常显示

## 🎯 为什么预签名 URL 不包含 Headers？

### 普通请求 vs 预签名 URL

**普通请求**:
- 签名在 `Authorization` header 中
- 需要包含所有签名的 headers
- Canonical request 包含 headers

**预签名 URL**:
- 签名在 URL 查询参数中
- 所有信息都在 URL 中
- Canonical request **不包含** headers

### 原因

1. **预签名 URL 是自包含的**：所有必要的信息（credential, date, expires）都在查询参数中
2. **不需要额外的 headers**：浏览器访问 URL 时不需要添加特殊的 headers
3. **简化验证**：OSS 服务器只需要验证 URL 中的参数，不需要检查 headers

## ✨ 特点总结

- ✅ **正确的 Canonical Request 格式**
- ✅ **不包含 headers**：预签名 URL 不需要 headers
- ✅ **两个空行**：表示没有 canonical headers 和 signed headers
- ✅ **符合 OSS 规范**：与 OSS 文档一致

## 📚 参考

### Canonical Request 格式

**AWS S3 普通请求**:
```
<HTTPMethod>\n
<CanonicalURI>\n
<CanonicalQueryString>\n
<CanonicalHeaders>\n
<SignedHeaders>\n
<HashedPayload>
```

**OSS 预签名 URL**:
```
<HTTPMethod>\n
<CanonicalURI>\n
<CanonicalQueryString>\n
\n
\n
<HashedPayload>
```

注意：OSS 预签名 URL 的 canonical request 中，canonical headers 和 signed headers 都是空的，所以有两个连续的换行符。


