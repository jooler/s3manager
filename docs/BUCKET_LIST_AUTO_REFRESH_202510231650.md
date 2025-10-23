# 设置页面添加存储桶后自动刷新左侧导航

## 需求

在设置页面中添加新的存储桶后，左侧导航中要立刻更新数据。

## 解决方案

### 1. 添加刷新信号到 GlobalState

**文件**: `src/lib/type.ts`

**变更**:
```typescript
export interface GlobalState {
  // ... 其他字段
  bucketsRefreshSignal: number;
}
```

用于触发 BucketList 组件的重新加载。

### 2. 初始化刷新信号

**文件**: `src/lib/store.svelte.ts`

**变更**:
```typescript
export let globalState: GlobalState = $state({
  // ... 其他字段
  bucketsRefreshSignal: 0,
});
```

### 3. 添加刷新函数

**文件**: `src/lib/store.svelte.ts`

**新增函数**:
```typescript
export function refreshBuckets() {
  globalState.bucketsRefreshSignal++;
}
```

每次调用时，将信号值加 1，触发监听该信号的 effect。

### 4. 更新 BucketList 组件

**文件**: `src/lib/components/BucketList.svelte`

**变更**:
```typescript
// 监听存储桶刷新信号
$effect(() => {
  // 当 bucketsRefreshSignal 变化时，重新加载存储桶列表
  globalState.bucketsRefreshSignal;
  loadBuckets();
});
```

当 `bucketsRefreshSignal` 变化时，自动调用 `loadBuckets()` 重新加载存储桶列表。

### 5. 更新 AddBucket 组件

**文件**: `src/lib/components/AddBucket.svelte`

**变更**:
1. 导入 `refreshBuckets` 函数
2. 在 `saveBucket()` 函数中调用 `refreshBuckets()`

```typescript
async function saveBucket() {
  await db.buckets.put({
    ...bucket,
  });

  refreshBuckets();  // 触发刷新信号
  closeModal();
}
```

## 工作流程

### 添加新存储桶流程

```
用户在设置页面点击"添加新存储桶"
  ↓
AddBucket 模态框打开
  ↓
用户填写存储桶信息并点击"保存"
  ↓
saveBucket() 被调用
  ↓
存储桶数据保存到数据库
  ↓
refreshBuckets() 被调用
  ↓
globalState.bucketsRefreshSignal 加 1
  ↓
BucketList 组件的 $effect 被触发
  ↓
loadBuckets() 重新加载存储桶列表
  ↓
左侧导航立即显示新的存储桶
  ↓
模态框关闭
```

### 编辑存储桶流程

同样的流程，但是编辑现有存储桶时也会触发刷新。

## 文件修改清单

| 文件 | 操作 | 说明 |
|------|------|------|
| `src/lib/type.ts` | 修改 | 添加 bucketsRefreshSignal 字段 |
| `src/lib/store.svelte.ts` | 修改 | 初始化信号，添加 refreshBuckets 函数 |
| `src/lib/components/BucketList.svelte` | 修改 | 添加 $effect 监听刷新信号 |
| `src/lib/components/AddBucket.svelte` | 修改 | 导入并调用 refreshBuckets 函数 |

## 编译状态

✅ 无诊断错误
✅ 所有导入正确
✅ 代码质量良好

## 特点

- ✅ **实时更新**：添加存储桶后立即更新左侧导航
- ✅ **简洁设计**：使用信号机制，代码清晰易维护
- ✅ **自动初始化**：新添加的存储桶会自动成为激活的存储桶（如果是第一个）
- ✅ **兼容现有逻辑**：不影响其他功能
- ✅ **可扩展**：可以轻松添加其他需要刷新的操作

## 测试建议

1. ✅ 在设置页面添加新存储桶
2. ✅ 验证左侧导航立即显示新存储桶
3. ✅ 验证新存储桶显示在列表中
4. ✅ 验证新存储桶可以被点击选中
5. ✅ 验证管理页面能正确加载新存储桶的文件
6. ✅ 编辑现有存储桶，验证左侧导航更新
7. ✅ 删除存储桶，验证左侧导航更新
8. ✅ 测试折叠状态下的显示
9. ✅ 测试深色模式

## 用户体验改进

- **即时反馈**：用户添加存储桶后立即看到结果
- **无需刷新**：不需要手动刷新页面或重启应用
- **一致的状态**：左侧导航和设置页面始终同步
- **流畅的工作流**：用户可以快速添加多个存储桶并切换

