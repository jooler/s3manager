# 管理页面文件列表表格改进

## 改进内容

### 1. 添加滚动条支持

**问题**: 文件列表表格在文件较多时无法滚动

**解决方案**:
- 将文件列表容器改为 `flex-1 overflow-auto`
- 表格头部使用 `sticky top-0` 保持固定在顶部
- 当内容超过容器高度时自动显示滚动条

**代码变更**:
```svelte
<!-- 之前 -->
<div class="overflow-x-auto">
  <table class="w-full text-sm">
    <thead class="border-b border-slate-200 bg-slate-50 dark:border-slate-700 dark:bg-slate-800">

<!-- 之后 -->
<div class="flex-1 overflow-auto">
  <table class="w-full text-sm">
    <thead class="sticky top-0 border-b border-slate-200 bg-slate-50 dark:border-slate-700 dark:bg-slate-800">
```

### 2. 设置合理的边距

**问题**: 表格距离应用边缘没有合理的间距

**解决方案**:
- 外层容器已有 `p-4` 的内边距（16px）
- 文件列表容器使用 `flex-1 flex-col` 使其占满可用空间
- 整个页面使用 `h-full` 确保充分利用高度

**代码变更**:
```svelte
<!-- 之前 -->
<div class="rounded-lg border border-slate-200 dark:border-slate-700">

<!-- 之后 -->
<div class="flex flex-1 flex-col rounded-lg border border-slate-200 dark:border-slate-700">
```

### 3. 改进空状态显示

**改进**: 当没有文件时，空状态消息现在居中显示在表格区域

```svelte
<!-- 之前 -->
<div class="p-4 text-center text-slate-500 dark:text-slate-400">

<!-- 之后 -->
<div class="flex flex-1 items-center justify-center p-4 text-center text-slate-500 dark:text-slate-400">
```

### 4. 修复可访问性问题

**问题**: 页面项数选择器的 label 没有正确关联

**解决方案**:
- 为 select 元素添加 `id="page-size"`
- 为 label 元素添加 `for="page-size"`

```svelte
<!-- 之前 -->
<label class="text-sm text-slate-600 dark:text-slate-400">
  {t().manage.toolbar.pageSize}:
</label>
<select value={pageSize} ...>

<!-- 之后 -->
<label for="page-size" class="text-sm text-slate-600 dark:text-slate-400">
  {t().manage.toolbar.pageSize}:
</label>
<select id="page-size" value={pageSize} ...>
```

### 5. 修复下载功能

**问题**: 使用了不正确的 Tauri API

**解决方案**:
- 将 `await open(url)` 改为 `window.open(url, "_blank")`
- 移除了未使用的 `open` 导入

### 6. 代码清理

**移除**:
- 未使用的 `ChevronDown` 图标导入
- 未使用的 `open` 函数导入

## 布局结构

```
<div class="mx-auto flex h-full flex-col gap-4 p-4">  <!-- 外层容器，p-4 提供边距 -->
  <h1>标题</h1>
  
  <!-- 工具栏 -->
  <div class="flex items-center gap-4 ...">...</div>
  
  <!-- 多部分上传列表 -->
  <div class="rounded-lg border ...">...</div>
  
  <!-- 文件列表 -->
  <div class="flex flex-1 flex-col ...">  <!-- flex-1 占满剩余空间 -->
    <div class="border-b ...">标题</div>
    <div class="flex-1 overflow-auto">  <!-- 可滚动区域 -->
      <table>
        <thead class="sticky top-0">...</thead>  <!-- 固定表头 -->
        <tbody>...</tbody>
      </table>
    </div>
  </div>
  
  <!-- 分页控制 -->
  <div class="flex items-center justify-between ...">...</div>
</div>
```

## 样式类说明

| 类名 | 作用 |
|------|------|
| `flex-1` | 占满父容器的剩余空间 |
| `overflow-auto` | 内容超出时显示滚动条 |
| `sticky top-0` | 表头固定在顶部，滚动时保持可见 |
| `p-4` | 内边距 16px |
| `gap-4` | 元素间距 16px |
| `h-full` | 高度 100% |

## 测试建议

1. ✅ 添加大量文件（>50个）测试滚动条显示
2. ✅ 验证表头在滚动时保持固定
3. ✅ 检查边距是否合理
4. ✅ 测试空状态显示
5. ✅ 验证响应式设计
6. ✅ 测试深色模式

## 编译状态

✅ 无诊断错误
✅ 所有可访问性问题已修复

