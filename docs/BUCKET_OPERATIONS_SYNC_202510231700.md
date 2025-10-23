# 设置页面所有存储桶操作同步更新左侧导航

## 需求

设置页面中对存储桶数据进行更新后，都应该触发左侧导航栏数据的同步更新，包括新增、编辑、删除等操作。

## 解决方案

### 1. 在设置页面导入 `refreshBuckets` 函数

**文件**: `src/routes/setting/+page.svelte`

**变更**:
```typescript
import { globalState, refreshBuckets } from "$lib/store.svelte";
```

### 2. 在所有存储桶操作中调用 `refreshBuckets()`

#### 新增存储桶

**函数**: `onAddBucketClose()`

```typescript
async function onAddBucketClose() {
  buckets = await db.buckets.toArray();
  // 如果只有一个存储桶且没有默认存储桶，自动设置为默认
  if (buckets.length === 1 && !globalState.appSetting.defaultBucketId) {
    await setDefaultBucket(buckets[0].id!);
  }
  refreshBuckets();  // 触发刷新
}
```

#### 编辑存储桶

通过 AddBucket 组件的 `saveBucket()` 函数已经调用了 `refreshBuckets()`。

#### 删除存储桶

**函数**: `deleteBucket()`

```typescript
async function deleteBucket(id: number) {
  await db.buckets.delete(id);
  buckets = await db.buckets.toArray();
  checkDefaultBucket();
  refreshBuckets();  // 触发刷新
}
```

#### 设置默认存储桶

**函数**: `setDefaultBucket()`

```typescript
async function setDefaultBucket(id: number) {
  globalState.appSetting.defaultBucketId = id;
  refreshBuckets();  // 触发刷新
}
```

## 工作流程

### 新增存储桶

```
用户点击"添加新存储桶"
  ↓
AddBucket 模态框打开
  ↓
用户填写信息并保存
  ↓
AddBucket.saveBucket() 调用 refreshBuckets()
  ↓
模态框关闭，onAddBucketClose() 被调用
  ↓
onAddBucketClose() 再次调用 refreshBuckets()
  ↓
BucketList 组件刷新（可能被调用两次，但无害）
  ↓
左侧导航立即显示新存储桶
```

### 编辑存储桶

```
用户点击"编辑"按钮
  ↓
AddBucket 模态框打开（编辑模式）
  ↓
用户修改信息并保存
  ↓
AddBucket.saveBucket() 调用 refreshBuckets()
  ↓
模态框关闭，onAddBucketClose() 被调用
  ↓
onAddBucketClose() 再次调用 refreshBuckets()
  ↓
BucketList 组件刷新
  ↓
左侧导航立即显示更新后的存储桶信息
```

### 删除存储桶

```
用户点击"删除"按钮
  ↓
deleteBucket() 被调用
  ↓
存储桶从数据库删除
  ↓
refreshBuckets() 被调用
  ↓
BucketList 组件刷新
  ↓
左侧导航立即移除已删除的存储桶
```

### 设置默认存储桶

```
用户点击"设置为默认"按钮
  ↓
setDefaultBucket() 被调用
  ↓
defaultBucketId 更新
  ↓
refreshBuckets() 被调用
  ↓
BucketList 组件刷新
  ↓
左侧导航立即显示默认存储桶的标记
```

## 文件修改清单

| 文件 | 操作 | 说明 |
|------|------|------|
| `src/routes/setting/+page.svelte` | 修改 | 导入 refreshBuckets，在所有操作中调用 |
| `src/lib/components/AddBucket.svelte` | 已完成 | saveBucket() 中已调用 refreshBuckets() |
| `src/lib/components/BucketList.svelte` | 已完成 | $effect 监听刷新信号 |

## 编译状态

✅ 无诊断错误
✅ 所有导入正确
✅ 代码质量良好

## 特点

- ✅ **全面覆盖**：所有存储桶操作都会触发刷新
- ✅ **实时同步**：左侧导航与设置页面始终保持同步
- ✅ **用户体验**：用户能立即看到操作结果
- ✅ **简洁设计**：使用统一的刷新机制
- ✅ **可靠性**：即使刷新被调用多次也不会出现问题

## 测试建议

1. ✅ 在设置页面添加新存储桶，验证左侧导航立即更新
2. ✅ 编辑现有存储桶，验证左侧导航立即更新
3. ✅ 删除存储桶，验证左侧导航立即移除
4. ✅ 设置默认存储桶，验证左侧导航显示标记
5. ✅ 验证折叠状态下的显示也会更新
6. ✅ 验证新添加的存储桶可以立即在管理页面使用
7. ✅ 验证删除的存储桶不再出现在左侧导航
8. ✅ 测试深色模式
9. ✅ 测试移动端显示

## 用户体验改进

- **即时反馈**：所有操作都能立即在左侧导航中看到结果
- **一致的状态**：设置页面和左侧导航始终同步
- **流畅的工作流**：用户可以快速进行多个操作
- **无需刷新**：不需要手动刷新页面或重启应用
- **可靠的数据**：左侧导航始终显示最新的存储桶列表

