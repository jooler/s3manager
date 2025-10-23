# 存储桶切换调试 - 2025-10-23 19:30

## 📋 问题描述

用户报告：切换到 OSS 存储桶后，文件列表显示 "Failed to load data"，但实际上显示的可能是之前 R2 存储桶的数据。

## 🔍 问题分析

### 当前逻辑

#### 1. 存储桶切换流程（BucketList.svelte）

```typescript
async function selectBucket(bucketId: number | undefined) {
  if (!bucketId) return;
  
  // Step 1: 更新 activeSelectedBucketId
  globalState.activeSelectedBucketId = bucketId;

  // Step 2: 保存到数据库
  globalState.appSetting.lastActiveBucketId = bucketId;
  const settings = await db.appSettings.get(1);
  if (settings) {
    await db.appSettings.update(1, { lastActiveBucketId: bucketId });
  }

  // Step 3: 更新 selectedBucket
  const bucket = buckets.find((b) => b.id === bucketId);
  if (bucket) {
    globalState.selectedBucket = {
      value: bucket,
      label: bucket.bucketName,
    };
  }
}
```

#### 2. 管理页面监听逻辑（manage/+page.svelte）

**修改前**:
```typescript
$effect(() => {
  // 当激活的存储桶改变时，重新加载数据
  if (globalState.activeSelectedBucketId) {
    currentPage = 1;
    continuationToken = undefined;
    loadData();
  }
});
```

**问题**:
- `$effect` 监听 `activeSelectedBucketId` 变化
- 但 `loadData()` 使用 `globalState.selectedBucket`
- 可能存在时序问题：`activeSelectedBucketId` 更新了，但 `selectedBucket` 还没更新

### 潜在的竞态条件

```
时间线：
T1: globalState.activeSelectedBucketId = newId  ← $effect 触发
T2: $effect 调用 loadData()
T3: loadData() 读取 globalState.selectedBucket  ← 可能还是旧值！
T4: globalState.selectedBucket = newBucket      ← 太晚了
```

## ✅ 解决方案

### 1. 同时监听两个状态

**修改后**:
```typescript
$effect(() => {
  // 当激活的存储桶改变时，重新加载数据
  // 同时检查 selectedBucket 是否已更新，确保数据一致性
  if (globalState.activeSelectedBucketId && globalState.selectedBucket) {
    console.log("Bucket changed, loading data for:", globalState.selectedBucket.value.bucketName);
    currentPage = 1;
    continuationToken = undefined;
    loadData();
  }
});
```

**优点**:
- 确保 `selectedBucket` 已经更新
- 避免使用旧的存储桶数据

### 2. 添加调试日志

#### loadData 函数
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
    
    console.log("Loading data for bucket:", {
      bucketName: bucket.bucketName,
      endpoint: bucket.endpoint,
      isOSS: bucket.endpoint?.includes("aliyuncs.com"),
    });
    
    // Load files...
  }
}
```

#### previewImage 函数
```typescript
async function previewImage(key: string) {
  try {
    const bucket = globalState.selectedBucket?.value;
    if (!bucket) {
      console.error("No bucket selected");
      return;
    }

    console.log("Previewing image:", {
      key,
      bucketName: bucket.bucketName,
      endpoint: bucket.endpoint,
      isOSS: bucket.endpoint?.includes("aliyuncs.com"),
    });

    const presignedUrl = await invoke<string>("r2_get_presigned_url", {
      bucketName: bucket.bucketName,
      accountId: bucket.accountId,
      accessKey: bucket.accessKey,
      secretKey: bucket.secretKey,
      key,
      endpoint: bucket.endpoint || undefined,
      expiresIn: 3600,
    });

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

## 🧪 调试步骤

### 1. 打开开发者工具
- 按 F12 打开浏览器开发者工具
- 切换到 Console 标签

### 2. 切换存储桶
- 在左侧导航栏点击 OSS 存储桶
- 观察控制台输出

### 3. 检查日志

**期望看到的日志**:
```
Bucket changed, loading data for: airspace
Loading data for bucket: {
  bucketName: "airspace",
  endpoint: "https://oss-cn-shanghai.aliyuncs.com",
  isOSS: true
}
```

**如果看到错误的存储桶名称**:
- 说明 `selectedBucket` 没有正确更新
- 需要检查 `BucketList.svelte` 的 `selectBucket` 函数

**如果看到 R2 的存储桶名称**:
- 说明切换逻辑有问题
- 需要检查 `activeSelectedBucketId` 的更新

### 4. 预览图片
- 点击图片的预览按钮
- 观察控制台输出

**期望看到的日志**:
```
Previewing image: {
  key: "image.jpg",
  bucketName: "airspace",
  endpoint: "https://oss-cn-shanghai.aliyuncs.com",
  isOSS: true
}
Generated presigned URL: https://airspace.oss-cn-shanghai.aliyuncs.com/image.jpg?x-oss-credential=...
```

## 📝 文件修改清单

| 文件 | 操作 | 说明 |
|------|------|------|
| `src/routes/manage/+page.svelte` | ✅ 修改 | 修改 $effect 监听逻辑，添加调试日志 |
| `src-tauri/src/r2.rs` | ✅ 恢复 | 恢复 canonical URI（不包含 bucket 名称） |

## 🎯 预期结果

### 正常情况
1. 切换到 OSS 存储桶
2. 控制台显示正确的存储桶信息
3. 文件列表正确加载 OSS 的文件
4. 预览图片时使用正确的 OSS 存储桶信息

### 异常情况
1. 如果控制台显示错误的存储桶名称
   - 说明状态更新有问题
   - 需要检查 `BucketList.svelte`

2. 如果文件列表加载失败
   - 检查后端日志
   - 检查 OSS 凭证是否正确

3. 如果预览图片 403
   - 说明签名算法有问题
   - 需要继续调试 OSS V4 签名

## 🔧 后续优化

### 1. 统一状态管理
考虑只使用一个状态变量：
```typescript
// 方案 A: 只使用 activeSelectedBucketId
$effect(() => {
  if (globalState.activeSelectedBucketId) {
    const bucket = await db.buckets.get(globalState.activeSelectedBucketId);
    if (bucket) {
      // 使用 bucket 加载数据
    }
  }
});

// 方案 B: 只使用 selectedBucket
$effect(() => {
  if (globalState.selectedBucket) {
    // 直接使用 selectedBucket 加载数据
  }
});
```

### 2. 添加加载状态
```typescript
let switchingBucket = $state(false);

async function selectBucket(bucketId: number | undefined) {
  switchingBucket = true;
  try {
    // 切换逻辑
  } finally {
    switchingBucket = false;
  }
}
```

### 3. 添加错误处理
```typescript
$effect(() => {
  if (globalState.activeSelectedBucketId && globalState.selectedBucket) {
    // 验证 ID 是否匹配
    if (globalState.selectedBucket.value.id !== globalState.activeSelectedBucketId) {
      console.error("Bucket ID mismatch!");
      return;
    }
    loadData();
  }
});
```

## 📚 相关文档

- `BUCKET_MANAGEMENT_RESTRUCTURE_202510231620.md` - 存储桶管理重构
- `BUCKET_PERSISTENCE_AND_UI_IMPROVEMENTS_202510231630.md` - 存储桶持久化
- `OSS_V4_SIGNATURE_IMPLEMENTATION_202510231920.md` - OSS V4 签名实现


