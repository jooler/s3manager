# 存储桶切换竞态条件修复 - 2025-10-23 19:40

## 📋 问题描述

用户报告：
1. 切换到 OSS 存储桶后，显示的是上一个 R2 存储桶的数据
2. 首先获取 OSS 存储桶的数据，之后又再次获取了上一个 R2 存储桶的数据
3. 修改每页数量后重新获取的数据是正确的

## 🔍 根本原因分析

### 问题 1: 状态更新顺序错误

**BucketList.svelte 的 selectBucket 函数（修改前）**:

```typescript
async function selectBucket(bucketId: number | undefined) {
  if (!bucketId) return;
  
  // ❌ 第 1 步：更新 activeSelectedBucketId → 立即触发 $effect
  globalState.activeSelectedBucketId = bucketId;

  // 第 2 步：保存到数据库（异步操作）
  globalState.appSetting.lastActiveBucketId = bucketId;
  const settings = await db.appSettings.get(1);
  if (settings) {
    await db.appSettings.update(1, { lastActiveBucketId: bucketId });
  }

  // ❌ 第 3 步：更新 selectedBucket（太晚了！）
  const bucket = buckets.find((b) => b.id === bucketId);
  if (bucket) {
    globalState.selectedBucket = {
      value: bucket,
      label: bucket.bucketName,
    };
  }
}
```

**时间线**:
```
T1: globalState.activeSelectedBucketId = newId (OSS)
    ↓ 触发 manage/+page.svelte 的 $effect
T2: $effect 检查条件通过，调用 loadData()
T3: loadData() 读取 globalState.selectedBucket
    ↓ 此时 selectedBucket 还是旧值（R2）！
T4: 使用 R2 的凭证请求数据
T5: globalState.selectedBucket = newBucket (OSS)
    ↓ 太晚了，loadData() 已经在执行了
```

**结果**: 虽然切换到了 OSS 存储桶，但实际加载的是 R2 的数据。

### 问题 2: 重复触发 loadData()

**manage/+page.svelte（修改前）**:

```typescript
onMount(() => {
  loadData();  // ← 第 1 次调用
});

$effect(() => {
  if (globalState.activeSelectedBucketId) {
    loadData();  // ← 第 2 次调用
  }
});
```

**时间线**:
```
T1: 组件挂载
T2: onMount 调用 loadData() → 加载初始存储桶（可能是 R2）
T3: BucketList 组件初始化，设置 activeSelectedBucketId
T4: $effect 触发，再次调用 loadData()
```

**结果**: 数据被加载两次，第二次可能覆盖第一次的正确数据。

### 问题 3: 没有防止重复加载

如果 `loadData()` 正在执行时，又触发了新的 `loadData()`，会导致：
- 两个请求同时进行
- 后完成的请求覆盖先完成的请求
- 数据不一致

## ✅ 解决方案

### 1. 修复状态更新顺序（BucketList.svelte）

**关键改动**: 先更新 `selectedBucket`，再更新 `activeSelectedBucketId`

```typescript
async function selectBucket(bucketId: number | undefined) {
  if (!bucketId) return;
  
  console.log("Selecting bucket:", bucketId);
  
  // ✅ 第 1 步：先找到存储桶对象
  const bucket = buckets.find((b) => b.id === bucketId);
  if (!bucket) {
    console.error("Bucket not found:", bucketId);
    return;
  }

  console.log("Found bucket:", {
    id: bucket.id,
    name: bucket.bucketName,
    endpoint: bucket.endpoint,
    isOSS: bucket.endpoint?.includes("aliyuncs.com"),
  });

  // ✅ 第 2 步：先更新 selectedBucket（同步操作）
  globalState.selectedBucket = {
    value: bucket,
    label: bucket.bucketName,
  };

  // ✅ 第 3 步：然后更新 activeSelectedBucketId（触发 $effect）
  globalState.activeSelectedBucketId = bucketId;

  // 第 4 步：最后保存到数据库（异步操作，不影响 UI）
  globalState.appSetting.lastActiveBucketId = bucketId;
  const settings = await db.appSettings.get(1);
  if (settings) {
    await db.appSettings.update(1, { lastActiveBucketId: bucketId });
  }
}
```

**新的时间线**:
```
T1: globalState.selectedBucket = newBucket (OSS)
    ↓ selectedBucket 已更新
T2: globalState.activeSelectedBucketId = newId (OSS)
    ↓ 触发 $effect
T3: $effect 检查条件通过，调用 loadData()
T4: loadData() 读取 globalState.selectedBucket
    ↓ 此时 selectedBucket 已经是新值（OSS）✅
T5: 使用 OSS 的凭证请求数据 ✅
```

### 2. 移除 onMount，避免重复加载（manage/+page.svelte）

**修改前**:
```typescript
onMount(() => {
  loadData();
});

$effect(() => {
  if (globalState.activeSelectedBucketId) {
    loadData();
  }
});
```

**修改后**:
```typescript
// ✅ 移除 onMount，只使用 $effect
$effect(() => {
  if (
    globalState.activeSelectedBucketId && 
    globalState.selectedBucket &&
    globalState.activeSelectedBucketId !== lastLoadedBucketId
  ) {
    // 加载数据
    loadData();
  }
});
```

### 3. 添加防重复加载机制

**添加 lastLoadedBucketId 状态**:
```typescript
let lastLoadedBucketId: number | undefined = $state(undefined);
```

**在 $effect 中检查**:
```typescript
$effect(() => {
  console.log("$effect triggered:", {
    activeSelectedBucketId: globalState.activeSelectedBucketId,
    selectedBucket: globalState.selectedBucket?.value.bucketName,
    lastLoadedBucketId,
    willLoad: globalState.activeSelectedBucketId && 
              globalState.selectedBucket &&
              globalState.activeSelectedBucketId !== lastLoadedBucketId,
  });
  
  if (
    globalState.activeSelectedBucketId && 
    globalState.selectedBucket &&
    globalState.activeSelectedBucketId !== lastLoadedBucketId  // ← 防止重复加载
  ) {
    console.log("✅ Bucket changed, loading data for:", {
      bucketId: globalState.activeSelectedBucketId,
      bucketName: globalState.selectedBucket.value.bucketName,
      previousBucketId: lastLoadedBucketId,
    });
    
    // 先清空数据
    files = [];
    multipartUploads = [];
    totalCount = 0;
    
    // 重置分页
    currentPage = 1;
    continuationToken = undefined;
    nextContinuationToken = undefined;
    
    // 记录当前加载的存储桶 ID
    lastLoadedBucketId = globalState.activeSelectedBucketId;
    
    // 加载数据
    loadData();
  }
});
```

### 4. 在 loadData 中添加防重入检查

```typescript
async function loadData() {
  if (!globalState.selectedBucket) {
    setAlert(t().common.noBucketWarning);
    return;
  }

  // ✅ 防止重复加载
  if (loading) {
    console.log("Already loading, skipping duplicate request");
    return;
  }

  loading = true;
  error = null;

  try {
    const bucket = globalState.selectedBucket.value;

    console.log("Loading data for bucket:", {
      bucketId: bucket.id,
      bucketName: bucket.bucketName,
      endpoint: bucket.endpoint,
      isOSS: bucket.endpoint?.includes("aliyuncs.com"),
      currentPage,
      pageSize,
      stackTrace: new Error().stack?.split('\n').slice(2, 4).join('\n'),
    });

    // 加载文件...
  } catch (e) {
    error = e instanceof Error ? e.message : "Failed to load data";
    console.error("Error loading data:", e);
  } finally {
    loading = false;
  }
}
```

### 5. 先清空数据再加载

```typescript
if (globalState.activeSelectedBucketId !== lastLoadedBucketId) {
  // ✅ 先清空数据，避免显示旧数据
  files = [];
  multipartUploads = [];
  totalCount = 0;
  
  // 重置分页
  currentPage = 1;
  continuationToken = undefined;
  nextContinuationToken = undefined;
  
  // 然后加载新数据
  loadData();
}
```

## 📝 文件修改清单

| 文件 | 操作 | 说明 |
|------|------|------|
| `src/lib/components/BucketList.svelte` | ✅ 修改 | 修复状态更新顺序，添加日志 |
| `src/routes/manage/+page.svelte` | ✅ 修改 | 移除 onMount，添加防重复机制，先清空数据 |

## 🎯 修复效果

### 修复前
```
用户点击 OSS 存储桶
  ↓
activeSelectedBucketId 更新 → 触发 $effect
  ↓
loadData() 读取 selectedBucket（还是 R2）
  ↓
使用 R2 凭证加载数据
  ↓
selectedBucket 更新为 OSS（太晚了）
  ↓
显示 R2 的数据 ❌
```

### 修复后
```
用户点击 OSS 存储桶
  ↓
selectedBucket 更新为 OSS
  ↓
activeSelectedBucketId 更新 → 触发 $effect
  ↓
检查 lastLoadedBucketId，确认需要加载
  ↓
先清空数据（files = []）
  ↓
loadData() 读取 selectedBucket（已经是 OSS）
  ↓
使用 OSS 凭证加载数据
  ↓
显示 OSS 的数据 ✅
```

## 🧪 测试步骤

1. **打开开发者工具**（F12）→ Console 标签

2. **切换到 OSS 存储桶**，观察控制台输出：
   ```
   Selecting bucket: 2
   Found bucket: {
     id: 2,
     name: "airspace",
     endpoint: "https://oss-cn-shanghai.aliyuncs.com",
     isOSS: true
   }
   $effect triggered: {
     activeSelectedBucketId: 2,
     selectedBucket: "airspace",
     lastLoadedBucketId: 1,
     willLoad: true
   }
   ✅ Bucket changed, loading data for: {
     bucketId: 2,
     bucketName: "airspace",
     previousBucketId: 1
   }
   Loading data for bucket: {
     bucketId: 2,
     bucketName: "airspace",
     endpoint: "https://oss-cn-shanghai.aliyuncs.com",
     isOSS: true,
     currentPage: 1,
     pageSize: 10
   }
   ```

3. **验证数据正确性**：
   - 文件列表应该显示 OSS 存储桶的文件
   - 不应该显示 R2 存储桶的文件
   - 不应该有第二次加载

4. **切换回 R2 存储桶**，验证反向切换也正常

5. **快速切换存储桶**，验证防重复机制生效

## 🔧 调试日志说明

### BucketList.svelte
- `Selecting bucket: X` - 开始选择存储桶
- `Found bucket: {...}` - 找到存储桶对象，显示详细信息

### manage/+page.svelte
- `$effect triggered: {...}` - $effect 被触发，显示所有相关状态
- `✅ Bucket changed, loading data for: {...}` - 确认需要加载，显示新旧存储桶 ID
- `Loading data for bucket: {...}` - 开始加载数据，显示存储桶详情和调用栈
- `Already loading, skipping duplicate request` - 防重入机制生效

## 📚 相关概念

### Svelte 5 响应式系统

**$state**: 创建响应式状态
```typescript
let count = $state(0);
```

**$effect**: 监听状态变化并执行副作用
```typescript
$effect(() => {
  console.log(count);  // 当 count 变化时执行
});
```

**执行顺序**:
1. 状态更新（同步）
2. 触发 $effect（同步）
3. $effect 内的代码执行（同步）
4. 异步操作（如果有）

### 竞态条件（Race Condition）

当多个操作的执行顺序不确定时，可能导致意外的结果。

**示例**:
```typescript
// 操作 A
globalState.activeSelectedBucketId = newId;  // 触发 $effect

// 操作 B（在 $effect 中）
loadData();  // 读取 globalState.selectedBucket

// 操作 C
globalState.selectedBucket = newBucket;  // 太晚了
```

**解决方案**:
- 确保依赖的状态在触发副作用之前已更新
- 使用标志位防止重复执行
- 添加状态一致性检查

## 🎓 经验总结

1. **状态更新顺序很重要**：在触发副作用之前，确保所有依赖的状态都已更新

2. **避免重复触发**：使用标志位（如 `lastLoadedBucketId`）防止重复执行

3. **先清空再加载**：切换数据源时，先清空旧数据，避免显示错误的数据

4. **添加防重入检查**：使用 `loading` 标志防止并发请求

5. **详细的日志**：添加日志帮助调试复杂的状态变化


