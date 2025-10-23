# 存储桶类型选择器实现 - 完整总结

## 📋 需求

在设置界面，修改"添加存储桶"按钮的行为：
- 原来：直接弹出添加表单
- 现在：弹出下拉菜单，包含 Cloudflare R2 和阿里云 OSS 两个条目
- 点击选择后触发不同的新增表单

## ✅ 实现完成

### 1️⃣ 类型定义修改

**文件**: `src/lib/type.ts`

**变更**:
```typescript
export interface Bucket {
  id?: number;
  type: "r2" | "s3" | "oss";  // 添加 "oss" 类型
  bucketName: string;
  accountId: string;
  accessKey: string;
  secretKey: string;
  customDomain: string;
  s3Api?: string;
  endpoint?: string;           // 新增：S3 兼容 endpoint
  region?: string;             // 新增：区域信息（可选）
  [key: string]: string | number | undefined;
}
```

### 2️⃣ 新建存储桶类型选择器组件

**文件**: `src/lib/components/BucketTypeSelector.svelte` ✨ 新建

**功能**:
- 显示两个存储桶类型选项
- 每个选项包含名称和描述
- 点击选项触发回调函数
- 支持深色模式

**选项**:
1. **Cloudflare R2**
   - 描述：Global cloud storage with free downloads
   - 中文：全球云存储，下载免费

2. **Aliyun OSS**
   - 描述：Alibaba Cloud Object Storage Service
   - 中文：阿里云对象存储服务

### 3️⃣ AddBucket 组件重构

**文件**: `src/lib/components/AddBucket.svelte`

**主要变更**:

#### 新增状态
```typescript
let showTypeSelector = $state(false);  // 控制类型选择器显示
```

#### 新增函数
```typescript
// 处理存储桶类型选择
function handleBucketTypeSelect(type: "r2" | "oss") {
  bucket.type = type;
  showTypeSelector = false;
  resetState();
  updateInputConfigs();
}

// 根据存储桶类型更新输入字段
function updateInputConfigs() {
  if (bucket.type === "r2") {
    // R2 表单字段：s3Api, bucketName, accountId, accessKey, secretKey, customDomain
  } else if (bucket.type === "oss") {
    // OSS 表单字段：bucketName, accessKey, secretKey, endpoint, customDomain
  }
}
```

#### 修改 $effect
```typescript
$effect(() => {
  if (show) {
    if (editBucketId) {
      // 编辑模式：直接显示表单
      showTypeSelector = false;
    } else {
      // 新增模式：显示类型选择器
      showTypeSelector = true;
    }
  }
});
```

#### 修改 snippet 内容
```svelte
{#if showTypeSelector}
  <BucketTypeSelector onTypeSelect={handleBucketTypeSelect} />
{:else if showHelp}
  <!-- 帮助内容 -->
{:else}
  <!-- 表单内容 -->
{/if}
```

### 4️⃣ 国际化翻译

**文件**: `src/lib/i18n.svelte.ts`

**英文翻译**:
```typescript
addBucket: {
  title: "Add Cloudflare R2 Bucket",
  titleR2: "Add Cloudflare R2 Bucket",
  titleOSS: "Add Aliyun OSS Bucket",
  selectBucketType: "Select Storage Type",
  labels: {
    endpoint: "Endpoint (for S3-compatible services)",
  },
}
```

**中文翻译**:
```typescript
addBucket: {
  title: "添加 Cloudflare R2 存储桶",
  titleR2: "添加 Cloudflare R2 存储桶",
  titleOSS: "添加阿里云 OSS 存储桶",
  selectBucketType: "选择存储类型",
  labels: {
    endpoint: "Endpoint（用于 S3 兼容服务）",
  },
}
```

---

## 📐 工作流程

### 新增存储桶流程

```
用户点击"添加新存储桶"
  ↓
显示类型选择器
  ├─ Cloudflare R2
  └─ Aliyun OSS
  ↓
用户选择类型
  ↓
显示对应的表单
  ├─ R2 表单：s3Api, bucketName, accountId, accessKey, secretKey, customDomain
  └─ OSS 表单：bucketName, accessKey, secretKey, endpoint, customDomain
  ↓
用户填写信息
  ↓
点击"检查"验证连接
  ↓
点击"保存"保存配置
```

### 编辑存储桶流程

```
用户点击"编辑"按钮
  ↓
直接显示对应的表单（跳过类型选择）
  ↓
用户修改信息
  ↓
点击"检查"验证连接
  ↓
点击"保存"保存配置
```

---

## 🎨 UI 设计

### 类型选择器

```
┌─────────────────────────────────────┐
│ 选择存储类型                         │
├─────────────────────────────────────┤
│ ┌─────────────────────────────────┐ │
│ │ Cloudflare R2                   │ │
│ │ Global cloud storage with free  │ │
│ │ downloads                       │ │
│ └─────────────────────────────────┘ │
│                                     │
│ ┌─────────────────────────────────┐ │
│ │ Aliyun OSS                      │ │
│ │ Alibaba Cloud Object Storage    │ │
│ │ Service                         │ │
│ └─────────────────────────────────┘ │
└─────────────────────────────────────┘
```

### R2 表单

```
┌─────────────────────────────────────┐
│ 添加 Cloudflare R2 存储桶            │
├─────────────────────────────────────┤
│ S3 API                              │
│ Bucket Name                         │
│ Account ID                          │
│ Access Key                          │
│ Secret Key                          │
│ Custom Domain                       │
├─────────────────────────────────────┤
│ [取消] [检查] [保存]                 │
└─────────────────────────────────────┘
```

### OSS 表单

```
┌─────────────────────────────────────┐
│ 添加阿里云 OSS 存储桶                │
├─────────────────────────────────────┤
│ Bucket Name                         │
│ Access Key                          │
│ Secret Key                          │
│ Endpoint                            │
│ Custom Domain                       │
├─────────────────────────────────────┤
│ [取消] [检查] [保存]                 │
└─────────────────────────────────────┘
```

---

## 📝 文件修改清单

| 文件 | 操作 | 说明 |
|------|------|------|
| `src/lib/type.ts` | ✅ 修改 | 添加 oss 类型和 endpoint 字段 |
| `src/lib/components/BucketTypeSelector.svelte` | ✨ 新建 | 存储桶类型选择器组件 |
| `src/lib/components/AddBucket.svelte` | ✅ 修改 | 支持类型选择和不同的表单 |
| `src/lib/i18n.svelte.ts` | ✅ 修改 | 添加国际化翻译 |

---

## ✅ 编译状态

- ✅ 无诊断错误
- ✅ 所有导入正确
- ✅ 代码质量良好

---

## 🧪 测试清单

- [ ] 点击"添加新存储桶"显示类型选择器
- [ ] 选择 Cloudflare R2 显示 R2 表单
- [ ] 选择 Aliyun OSS 显示 OSS 表单
- [ ] R2 表单包含所有必需字段
- [ ] OSS 表单包含所有必需字段
- [ ] 编辑 R2 存储桶直接显示 R2 表单
- [ ] 编辑 OSS 存储桶直接显示 OSS 表单
- [ ] 验证连接功能正常
- [ ] 保存功能正常
- [ ] 深色模式显示正确
- [ ] 国际化翻译正确（英文/中文）
- [ ] 取消按钮关闭模态框

---

## 🚀 后续步骤

### 后端改动（需要实施）

需要修改 Rust 后端以支持自定义 endpoint：

1. 修改 `R2Client::new()` 支持自定义 endpoint
2. 更新所有 Tauri 命令添加 endpoint 参数
3. 更新前端调用传递 endpoint

### 前端改动（已完成）

- ✅ 类型定义
- ✅ UI 组件
- ✅ 国际化翻译

---

## 💡 使用场景

### 新增 R2 存储桶

1. 点击"添加新存储桶"
2. 选择"Cloudflare R2"
3. 填写 S3 API、Bucket Name、Account ID、Access Key、Secret Key
4. 点击"检查"验证
5. 点击"保存"

### 新增 OSS 存储桶

1. 点击"添加新存储桶"
2. 选择"Aliyun OSS"
3. 填写 Bucket Name、Access Key、Secret Key、Endpoint
4. 点击"检查"验证
5. 点击"保存"

---

## 📚 相关文档

- `ALIYUN_OSS_COMPATIBILITY_ANALYSIS_202510231720.md` - 兼容性分析
- `ALIYUN_OSS_IMPLEMENTATION_GUIDE_202510231730.md` - 实施指南
- `R2_VS_OSS_COMPARISON_202510231740.md` - R2 vs OSS 对比


