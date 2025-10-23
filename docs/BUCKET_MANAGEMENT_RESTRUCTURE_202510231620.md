# 存储桶管理逻辑重构

## 需求

1. 左侧导航栏分为上下两部分
   - 上部：存储桶列表容器（可滚动）
   - 下部：设置条目（固定在底部）
2. 上传页面移除 BucketSelector 功能
3. 管理页面根据激活的存储桶显示对应的文件
4. 通过点击左侧存储桶列表条目来切换激活的存储桶

## 实现方案

### 1. 更新 GlobalState

**文件**: `src/lib/store.svelte.ts` 和 `src/lib/type.ts`

**新增字段**:
```typescript
activeSelectedBucketId: number | undefined
```

用于跟踪当前激活的存储桶 ID。

### 2. 创建存储桶列表组件

**文件**: `src/lib/components/BucketList.svelte`

**功能**:
- 从数据库加载所有存储桶
- 显示存储桶列表
- 支持点击切换激活状态
- 激活的存储桶显示青色高亮和指示点
- 自动设置默认存储桶（如果没有激活的）

**逻辑**:
```typescript
function selectBucket(bucketId: number | undefined) {
  if (!bucketId) return;
  globalState.activeSelectedBucketId = bucketId;
  // 同时更新 selectedBucket 以保持兼容性
  const bucket = buckets.find((b) => b.id === bucketId);
  if (bucket) {
    globalState.selectedBucket = {
      value: bucket,
      label: bucket.bucketName,
    };
  }
}
```

### 3. 修改 Sidebar 组件

**文件**: `src/lib/components/Sidebar.svelte`

**结构变更**:
```
导航栏 (flex flex-col)
├─ 顶部区域 (flex-col gap-2 border-b)
│  ├─ 折叠按钮
│  └─ "存储桶" 标签（展开时显示）
│
├─ 中间区域 (flex-1 overflow-y-auto)
│  └─ BucketList 组件（展开时显示）
│
└─ 底部区域 (border-t)
   └─ 设置链接
```

**特点**:
- 存储桶列表占据所有可用空间（flex-1）
- 设置条目始终固定在底部
- 支持侧边栏折叠/展开
- 移动端在底部导航栏中显示存储桶列表

### 4. 移除上传页面的 BucketSelector

**文件**: `src/routes/+page.svelte`

**变更**:
- 移除 `<BucketSelector />` 组件
- 移除相关导入

### 5. 更新管理页面逻辑

**文件**: `src/routes/manage/+page.svelte`

**变更**:
- 将所有状态变量改为 `$state()`
- 添加 `$effect` 监听 `activeSelectedBucketId` 变化
- 当激活的存储桶改变时，重置分页并重新加载数据

**代码**:
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

### 6. 添加翻译

**文件**: `src/lib/i18n.svelte.ts`

**新增翻译**:
- `common.buckets`: "Buckets" / "存储桶"

## 工作流程

### 用户操作流程

1. **切换存储桶**
   - 用户点击左侧导航栏中的存储桶条目
   - `activeSelectedBucketId` 更新
   - `selectedBucket` 同时更新（保持兼容性）

2. **上传文件**
   - 用户进入上传页面
   - 上传组件使用 `globalState.selectedBucket` 获取当前存储桶
   - 文件上传到激活的存储桶

3. **管理文件**
   - 用户进入管理页面
   - 管理页面监听 `activeSelectedBucketId` 变化
   - 自动加载激活存储桶的文件列表

4. **查看传输**
   - 传输页面可以显示所有存储桶的传输列表
   - 或根据激活的存储桶对应切换

## 文件修改清单

| 文件 | 操作 | 说明 |
|------|------|------|
| `src/lib/store.svelte.ts` | 修改 | 添加 activeSelectedBucketId |
| `src/lib/type.ts` | 修改 | 更新 GlobalState 接口 |
| `src/lib/components/BucketList.svelte` | 新建 | 存储桶列表组件 |
| `src/lib/components/Sidebar.svelte` | 修改 | 分为上下两部分结构 |
| `src/routes/+page.svelte` | 修改 | 移除 BucketSelector |
| `src/routes/manage/+page.svelte` | 修改 | 添加 $effect 监听激活存储桶 |
| `src/lib/i18n.svelte.ts` | 修改 | 添加翻译 |

## 编译状态

✅ 无诊断错误
✅ 所有导入正确
✅ 代码质量良好

## 测试建议

1. ✅ 验证左侧导航栏显示存储桶列表
2. ✅ 点击存储桶条目切换激活状态
3. ✅ 验证激活存储桶显示高亮
4. ✅ 上传页面不显示 BucketSelector
5. ✅ 管理页面根据激活存储桶显示文件
6. ✅ 切换存储桶时管理页面自动刷新
7. ✅ 测试侧边栏折叠/展开
8. ✅ 测试移动端显示
9. ✅ 测试深色模式
10. ✅ 测试传输页面逻辑

## 后续考虑

- 可以添加存储桶搜索功能
- 可以添加存储桶排序功能
- 可以添加存储桶快速操作菜单（编辑、删除等）
- 传输页面可以添加存储桶过滤功能

