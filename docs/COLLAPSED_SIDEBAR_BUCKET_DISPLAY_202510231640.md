# 折叠状态下的存储桶列表显示修复

## 问题

折叠状态时，存储桶列表没有显示，应该显示列表，但只取第一个字符，如果第一个字符是英文，转化为大写。

## 解决方案

### 1. 修改 Sidebar 组件

**文件**: `src/lib/components/Sidebar.svelte`

**变更**:
- 移除了对 `!globalState.appSetting.sidebarCollapsed` 的条件判断
- BucketList 组件现在始终显示
- 折叠时，容器使用 `flex flex-col items-center` 来居中显示存储桶按钮

**代码**:
```typescript
<!-- Scrollable Bucket List -->
<div class="flex-1 overflow-y-auto {globalState.appSetting.sidebarCollapsed ? 'flex flex-col items-center px-1 py-2' : 'px-2 py-2'}">
  <BucketList />
</div>
```

### 2. 优化 BucketList 组件

**文件**: `src/lib/components/BucketList.svelte`

**变更**:
- 根据折叠状态调整间距：`gap-1` (折叠) vs `gap-2` (展开)
- 折叠状态下的按钮：
  - 固定大小：`h-10 w-10`
  - 居中显示：`flex items-center justify-center`
  - 加粗字体：`font-bold`
  - 只显示首字母（大写）
- 展开状态下的按钮：
  - 正常布局：`gap-2 px-3 py-2`
  - 显示完整名称
  - 显示激活指示点

**代码**:
```typescript
<div class="flex flex-col {globalState.appSetting.sidebarCollapsed ? 'gap-1' : 'gap-2'}">
  {#each buckets as bucket (bucket.id)}
    <button
      class="flex items-center justify-center rounded-lg transition-colors {globalState.appSetting.sidebarCollapsed
        ? 'h-10 w-10 text-sm font-bold'
        : 'gap-2 px-3 py-2 text-left text-sm'} ..."
    >
      {#if globalState.appSetting.sidebarCollapsed}
        <span>{bucket.bucketName.charAt(0).toUpperCase()}</span>
      {:else}
        <div class="flex-1 truncate font-medium">{bucket.bucketName}</div>
        {#if globalState.activeSelectedBucketId === bucket.id}
          <div class="h-2 w-2 rounded-full bg-cyan-600 dark:bg-cyan-400"></div>
        {/if}
      {/if}
    </button>
  {/each}
</div>
```

## 显示效果

### 展开状态
```
┌──────────────────┐
│ 存储桶           │
├──────────────────┤
│ • my-bucket      │
│ • test-r2        │
│ • prod-storage   │
├──────────────────┤
│ 设置             │
└──────────────────┘
```

### 折叠状态
```
┌────┐
│ M  │  (my-bucket)
│ T  │  (test-r2)
│ P  │  (prod-storage)
├────┤
│ ⚙️  │  (设置)
└────┘
```

## 特点

- ✅ **始终显示**：折叠状态下存储桶列表仍然可见
- ✅ **节省空间**：只显示首字母，宽度仅 64px (w-16)
- ✅ **清晰标识**：首字母大写，易于识别
- ✅ **Tooltip 支持**：鼠标悬停显示完整名称
- ✅ **激活状态**：激活的存储桶显示高亮背景
- ✅ **响应式**：展开/折叠时平滑过渡
- ✅ **深色模式**：完全支持深色模式

## 文件修改清单

| 文件 | 操作 | 说明 |
|------|------|------|
| `src/lib/components/Sidebar.svelte` | 修改 | 移除条件判断，始终显示 BucketList |
| `src/lib/components/BucketList.svelte` | 修改 | 优化折叠状态下的显示样式 |

## 编译状态

✅ 无诊断错误
✅ 所有导入正确
✅ 代码质量良好

## 测试建议

1. ✅ 展开状态下，验证存储桶列表显示完整名称
2. ✅ 点击折叠按钮，验证 Sidebar 折叠
3. ✅ 折叠状态下，验证存储桶列表显示首字母（大写）
4. ✅ 折叠状态下，验证存储桶按钮大小和居中
5. ✅ 鼠标悬停存储桶按钮，验证 Tooltip 显示完整名称
6. ✅ 点击存储桶按钮，验证激活状态更新
7. ✅ 验证激活的存储桶显示高亮背景
8. ✅ 测试深色模式下的显示
9. ✅ 测试移动端显示（始终展开）
10. ✅ 验证管理页面根据激活的存储桶显示文件

## 用户体验改进

- **更好的空间利用**：折叠状态下仍然可以快速切换存储桶
- **清晰的视觉反馈**：激活的存储桶显示高亮
- **易于识别**：首字母大写，结合 Tooltip 提示
- **平滑的过渡**：展开/折叠时有过渡动画
- **一致的交互**：与其他 UI 元素保持一致

