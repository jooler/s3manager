# 存储桶持久化和 UI 改进

## 需求

1. 记住上一次激活的存储桶，应用加载后默认激活上次激活的存储桶
2. 如果不存在保存的上次激活的存储桶数据，默认激活第一个存储桶
3. 确保管理页面在加载后能够读取到正确的数据
4. 左侧导航顶部的折叠按钮移到右侧 Tab 切换栏的最左侧
5. 左侧导航处于折叠状态时，存储桶列表条目只展示存储桶名称的第一个字符（大写）

## 实现方案

### 1. 扩展 AppSettings 类型

**文件**: `src/lib/type.ts`

**新增字段**:
```typescript
lastActiveBucketId: number | undefined
```

用于保存上次激活的存储桶 ID。

### 2. 更新 GlobalState 初始化

**文件**: `src/lib/store.svelte.ts`

**变更**:
- 添加 `lastActiveBucketId: undefined` 到初始化对象

### 3. 改进 BucketList 组件

**文件**: `src/lib/components/BucketList.svelte`

**功能**:
- 应用启动时，优先使用上次激活的存储桶
- 如果上次激活的不存在，使用默认存储桶
- 如果默认存储桶也不存在，使用第一个存储桶
- 初始化时同时更新 `selectedBucket` 以保持兼容性

**初始化逻辑**:
```typescript
// 优先级：上次激活 > 默认 > 第一个
if (globalState.appSetting.lastActiveBucketId) {
  bucketId = lastActiveBucket?.id;
}
if (!bucketId && globalState.appSetting.defaultBucketId) {
  bucketId = defaultBucket?.id;
}
if (!bucketId) {
  bucketId = buckets[0].id;
}
```

**切换存储桶时**:
- 保存当前激活的存储桶 ID 到 `lastActiveBucketId`
- 更新数据库中的设置

**折叠状态显示**:
```typescript
{#if globalState.appSetting.sidebarCollapsed}
  <div class="flex-1 truncate text-center font-medium">
    {bucket.bucketName.charAt(0).toUpperCase()}
  </div>
{:else}
  <div class="flex-1 truncate font-medium">{bucket.bucketName}</div>
{/if}
```

### 4. 修改 TabNavigation 组件

**文件**: `src/lib/components/TabNavigation.svelte`

**变更**:
- 添加折叠按钮到最左侧
- 按钮显示当前折叠状态
- 点击按钮切换 `globalState.appSetting.sidebarCollapsed`

**按钮样式**:
```typescript
<button
  onclick={() =>
    (globalState.appSetting.sidebarCollapsed =
      !globalState.appSetting.sidebarCollapsed)}
  class="flex items-center justify-center rounded-lg p-2 text-slate-600 transition-colors hover:bg-slate-100 dark:text-slate-400 dark:hover:bg-slate-700/50"
>
  {#if globalState.appSetting.sidebarCollapsed}
    <PanelRightClose class="size-5" />
  {:else}
    <PanelRightOpen class="size-5" />
  {/if}
</button>
```

### 5. 简化 Sidebar 组件

**文件**: `src/lib/components/Sidebar.svelte`

**变更**:
- 移除折叠按钮（已移到 TabNavigation）
- 移除相关导入
- 简化顶部区域结构

**新结构**:
```
导航栏 (flex flex-col)
├─ 顶部区域 (border-b)
│  └─ "存储桶" 标签（展开时显示）
│
├─ 中间区域 (flex-1 overflow-y-auto)
│  └─ BucketList 组件
│
└─ 底部区域 (border-t)
   └─ 设置链接
```

## 工作流程

### 应用启动流程

1. 应用启动
2. `initAppSettings()` 从数据库加载设置
3. Sidebar 组件挂载
4. BucketList 组件挂载
5. BucketList 加载存储桶列表
6. 根据优先级设置激活的存储桶：
   - 上次激活的存储桶 ID
   - 默认存储桶 ID
   - 第一个存储桶
7. 更新 `activeSelectedBucketId` 和 `selectedBucket`
8. 管理页面监听到 `activeSelectedBucketId` 变化，自动加载文件列表

### 用户切换存储桶流程

1. 用户点击左侧存储桶条目
2. `selectBucket()` 被调用
3. 更新 `activeSelectedBucketId`
4. 保存 `lastActiveBucketId` 到数据库
5. 更新 `selectedBucket`
6. 管理页面监听到变化，自动刷新文件列表

### 折叠/展开流程

1. 用户点击 Tab 栏左侧的折叠按钮
2. `sidebarCollapsed` 状态切换
3. Sidebar 宽度变化
4. BucketList 显示方式变化：
   - 展开：显示完整存储桶名称
   - 折叠：显示首字母（大写）

## 文件修改清单

| 文件 | 操作 | 说明 |
|------|------|------|
| `src/lib/type.ts` | 修改 | 添加 lastActiveBucketId 字段 |
| `src/lib/store.svelte.ts` | 修改 | 初始化 lastActiveBucketId |
| `src/lib/components/BucketList.svelte` | 修改 | 改进初始化逻辑，支持折叠显示 |
| `src/lib/components/TabNavigation.svelte` | 修改 | 添加折叠按钮 |
| `src/lib/components/Sidebar.svelte` | 修改 | 移除折叠按钮，简化结构 |

## 编译状态

✅ 无诊断错误
✅ 所有导入正确
✅ 代码质量良好

## 测试建议

1. ✅ 应用启动时，验证激活的存储桶是否正确
2. ✅ 切换存储桶后，关闭应用再打开，验证是否恢复上次激活的存储桶
3. ✅ 验证管理页面在启动时是否能正确加载文件列表
4. ✅ 点击 Tab 栏左侧的折叠按钮，验证 Sidebar 是否正确折叠/展开
5. ✅ 折叠状态下，验证存储桶列表是否只显示首字母
6. ✅ 展开状态下，验证存储桶列表是否显示完整名称
7. ✅ 验证激活的存储桶是否始终高亮显示
8. ✅ 测试深色模式
9. ✅ 测试移动端显示

## 优势

- ✅ **用户体验**：应用记住用户的选择，下次启动时自动恢复
- ✅ **快速访问**：无需每次都重新选择存储桶
- ✅ **空间节省**：折叠状态下只显示首字母，节省空间
- ✅ **清晰的 UI**：折叠按钮在 Tab 栏中更加显眼
- ✅ **一致的交互**：所有导航控制都在顶部

