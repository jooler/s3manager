# OSS 预签名 URL 编码修复 - 2025-10-23 19:50

## 📋 问题描述

用户报告：OSS 预签名 URL 返回 403 错误。

对比官方应用和当前应用生成的 URL：

**官方应用（正确）**:
```
https://airspace.oss-cn-shanghai.aliyuncs.com/Nextspace_strapi/20230221_Friedensglocke_Fichtelberg_ZH_CN_5510489151_UHD_34e4838183.jpg?x-oss-credential=...
```

**当前应用（错误）**:
```
https://airspace.oss-cn-shanghai.aliyuncs.com/Nextspace_strapi%2F20230221_Friedensglocke_Fichtelberg_ZH_CN_5510489151_UHD_34e4838183.jpg?x-oss-credential=...
```

## 🔍 问题分析

### 关键区别

| 项目 | 官方应用 | 当前应用 |
|------|---------|---------|
| **路径** | `/Nextspace_strapi/20230221_...` | `/Nextspace_strapi%2F20230221_...` |
| **路径分隔符** | `/` | `%2F`（URL 编码） |
| **OSS 解析** | 子目录 `Nextspace_strapi` 中的文件 | 文件名包含 `%2F` 的文件 |
| **结果** | ✅ 找到文件 | ❌ 404/403（文件不存在） |

### 根本原因

**错误的代码**（第 709 行）:
```rust
let canonical_uri = format!("/{}", urlencoding::encode(key));
```

**问题**:
- `urlencoding::encode(key)` 会编码 **所有** 特殊字符
- 包括路径分隔符 `/`
- 导致 `/` 被编码为 `%2F`

**示例**:
```rust
let key = "Nextspace_strapi/20230221_Friedensglocke.jpg";
let encoded = urlencoding::encode(key);
// 结果: "Nextspace_strapi%2F20230221_Friedensglocke.jpg"
//                        ^^^
//                        路径分隔符被编码了！
```

### OSS 的期望

对于路径 `Nextspace_strapi/20230221_Friedensglocke.jpg`：

**Canonical URI 应该是**:
```
/Nextspace_strapi/20230221_Friedensglocke.jpg
```

**不应该是**:
```
/Nextspace_strapi%2F20230221_Friedensglocke.jpg
```

**原因**:
- OSS 使用 `/` 作为目录分隔符
- `/` 不应该被 URL 编码
- 只有文件名中的特殊字符才需要编码

### URL 编码规则

**需要编码的字符**:
- 空格 → `%20`
- 中文字符 → `%E4%B8%AD%E6%96%87`
- 特殊字符（`!`, `@`, `#`, `$`, `%`, `^`, `&`, `*`, `(`, `)`, `+`, `=`, `[`, `]`, `{`, `}`, `|`, `\`, `:`, `;`, `"`, `'`, `<`, `>`, `,`, `?`）

**不应该编码的字符**:
- 路径分隔符 `/`
- 字母数字 `a-z`, `A-Z`, `0-9`
- 安全字符 `-`, `_`, `.`, `~`

## ✅ 解决方案

### 修改前

```rust
// 构建 URL（使用 virtual-hosted-style）
let host = format!("{}.{}", self.bucket_name, endpoint_host);
let canonical_uri = format!("/{}", urlencoding::encode(key));
```

**问题**:
- 整个 key 被编码
- 路径分隔符 `/` 被编码为 `%2F`

### 修改后

```rust
// 构建 URL（使用 virtual-hosted-style）
let host = format!("{}.{}", self.bucket_name, endpoint_host);

// 构建 canonical URI
// 注意：路径分隔符 / 不应该被编码，只编码每个路径段
let canonical_uri = if key.contains('/') {
    // 如果 key 包含路径分隔符，分别编码每个段
    let segments: Vec<String> = key
        .split('/')
        .map(|segment| urlencoding::encode(segment).to_string())
        .collect();
    format!("/{}", segments.join("/"))
} else {
    // 如果没有路径分隔符，直接编码
    format!("/{}", urlencoding::encode(key))
};
```

**改进**:
- 按 `/` 分割路径
- 分别编码每个路径段
- 用 `/` 重新连接
- 保留路径分隔符

### 示例

#### 示例 1: 带子目录的文件

**输入**:
```rust
key = "Nextspace_strapi/20230221_Friedensglocke.jpg"
```

**处理过程**:
```rust
// 1. 分割路径
segments = ["Nextspace_strapi", "20230221_Friedensglocke.jpg"]

// 2. 编码每个段
encoded_segments = [
    "Nextspace_strapi",  // 没有特殊字符，不变
    "20230221_Friedensglocke.jpg"  // 没有特殊字符，不变
]

// 3. 用 / 连接
canonical_uri = "/Nextspace_strapi/20230221_Friedensglocke.jpg"
```

**结果**: ✅ 正确

#### 示例 2: 文件名包含空格

**输入**:
```rust
key = "folder/my file.jpg"
```

**处理过程**:
```rust
// 1. 分割路径
segments = ["folder", "my file.jpg"]

// 2. 编码每个段
encoded_segments = [
    "folder",      // 没有特殊字符，不变
    "my%20file.jpg"  // 空格被编码为 %20
]

// 3. 用 / 连接
canonical_uri = "/folder/my%20file.jpg"
```

**结果**: ✅ 正确

#### 示例 3: 文件名包含中文

**输入**:
```rust
key = "images/图片.jpg"
```

**处理过程**:
```rust
// 1. 分割路径
segments = ["images", "图片.jpg"]

// 2. 编码每个段
encoded_segments = [
    "images",                    // 没有特殊字符，不变
    "%E5%9B%BE%E7%89%87.jpg"    // 中文被编码
]

// 3. 用 / 连接
canonical_uri = "/images/%E5%9B%BE%E7%89%87.jpg"
```

**结果**: ✅ 正确

#### 示例 4: 多级目录

**输入**:
```rust
key = "a/b/c/file.jpg"
```

**处理过程**:
```rust
// 1. 分割路径
segments = ["a", "b", "c", "file.jpg"]

// 2. 编码每个段
encoded_segments = ["a", "b", "c", "file.jpg"]

// 3. 用 / 连接
canonical_uri = "/a/b/c/file.jpg"
```

**结果**: ✅ 正确

## 📝 文件修改清单

| 文件 | 操作 | 说明 |
|------|------|------|
| `src-tauri/src/r2.rs` | ✅ 修改 | 修复 canonical URI 的 URL 编码逻辑 |

## ✅ 编译状态

- ✅ 后端：编译成功（54.58秒）
- ⚠️ 警告：`account_id` 字段未使用（可忽略）
- ✅ 代码质量良好

## 🧪 测试步骤

1. **重新运行应用**

2. **切换到 OSS 存储桶**

3. **点击预览按钮**，查看生成的 URL：
   ```
   https://airspace.oss-cn-shanghai.aliyuncs.com/Nextspace_strapi/20230221_Friedensglocke_Fichtelberg_ZH_CN_5510489151_UHD_34e4838183.jpg?x-oss-credential=...
   ```

4. **验证**：
   - ✅ 路径中的 `/` 没有被编码
   - ✅ 文件名中的特殊字符（如果有）被正确编码
   - ✅ 点击 URL 可以正常下载/预览图片
   - ✅ 不再返回 403 错误

5. **测试不同类型的文件名**：
   - 简单文件名：`image.jpg`
   - 带子目录：`folder/image.jpg`
   - 多级目录：`a/b/c/image.jpg`
   - 包含空格：`folder/my image.jpg`
   - 包含中文：`images/图片.jpg`

## 🎯 修复效果

### 修复前

```
key = "Nextspace_strapi/20230221_Friedensglocke.jpg"
  ↓
urlencoding::encode(key)
  ↓
"Nextspace_strapi%2F20230221_Friedensglocke.jpg"
  ↓
canonical_uri = "/Nextspace_strapi%2F20230221_Friedensglocke.jpg"
  ↓
OSS 查找文件名为 "Nextspace_strapi%2F20230221_Friedensglocke.jpg" 的文件
  ↓
找不到文件 → 403 ❌
```

### 修复后

```
key = "Nextspace_strapi/20230221_Friedensglocke.jpg"
  ↓
split('/') → ["Nextspace_strapi", "20230221_Friedensglocke.jpg"]
  ↓
encode each segment → ["Nextspace_strapi", "20230221_Friedensglocke.jpg"]
  ↓
join('/') → "Nextspace_strapi/20230221_Friedensglocke.jpg"
  ↓
canonical_uri = "/Nextspace_strapi/20230221_Friedensglocke.jpg"
  ↓
OSS 查找 "Nextspace_strapi" 目录下的 "20230221_Friedensglocke.jpg" 文件
  ↓
找到文件 → 200 ✅
```

## 🔧 技术细节

### URL 编码（Percent Encoding）

**RFC 3986** 定义了 URL 编码规则：

**保留字符（Reserved Characters）**:
```
: / ? # [ ] @ ! $ & ' ( ) * + , ; =
```

**非保留字符（Unreserved Characters）**:
```
A-Z a-z 0-9 - _ . ~
```

**编码规则**:
- 非保留字符不需要编码
- 保留字符在特定上下文中不需要编码（如路径中的 `/`）
- 其他字符需要编码为 `%XX` 格式

### OSS 路径规则

**OSS 对象存储的路径**:
- 使用 `/` 作为目录分隔符
- `/` 是路径的一部分，不应该被编码
- 文件名中的特殊字符需要编码

**示例**:
```
正确: /folder/file.jpg
错误: /folder%2Ffile.jpg  ← OSS 会认为这是一个文件名，而不是路径
```

### Canonical Request 中的 URI

**AWS/OSS 签名算法要求**:
- Canonical URI 必须是 URL 编码的
- 但路径分隔符 `/` 不应该被编码
- 每个路径段应该单独编码

**示例**:
```
原始路径: /folder/my file.jpg
Canonical URI: /folder/my%20file.jpg
              ↑      ↑
              |      |
              |      空格被编码
              路径分隔符不编码
```

## 📚 相关文档

- [RFC 3986 - Uniform Resource Identifier (URI): Generic Syntax](https://tools.ietf.org/html/rfc3986)
- [阿里云 OSS - 对象命名规范](https://help.aliyun.com/document_detail/31827.html)
- [AWS Signature Version 4 - Canonical Request](https://docs.aws.amazon.com/general/latest/gr/sigv4-create-canonical-request.html)

## 🎓 经验总结

1. **URL 编码要分段处理**：路径分隔符不应该被编码

2. **理解 API 的期望**：不同的 API 对 URL 编码的要求可能不同

3. **测试不同场景**：简单文件名、子目录、特殊字符等

4. **参考官方实现**：对比官方应用生成的 URL 格式

5. **阅读文档**：仔细阅读 API 文档中关于 URL 编码的说明


