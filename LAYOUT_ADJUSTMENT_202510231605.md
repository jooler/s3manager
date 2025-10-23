# 布局调整 - Tab 工具栏位置和标题移除

## 调整内容

### 1. 布局结构调整

**从**: 上下布局（顶部 Tab，下方内容）
**到**: 左右布局（左侧导航，右侧顶部 Tab + 内容）

#### 新布局结构

```
应用根容器 (flex h-full)
├─ 左侧导航栏 (Sidebar)
│  └─ 设置
│
└─ 右侧主内容 (main flex flex-1 flex-col)
   ├─ Tab 工具栏 (TabNavigation)
   │  ├─ 管理 Tab
   │  ├─ 上传 Tab
   │  └─ 传输 Tab
   │
   └─ 页面内容 (flex-1 overflow-hidden)
      ├─ 管理页面
      ├─ 上传页面
      └─ 传输页面
```

### 2. 移除所有页面的 h1 标题

由于现在有 Tab 工具栏显示当前页面标题，所以移除了各页面的重复标题。

#### 修改的文件

| 文件 | 移除内容 |
|------|---------|
| `src/routes/manage/+page.svelte` | `<h1>{t().manage.title}</h1>` |
| `src/routes/+page.svelte` | `<h1>{t().common.upload}</h1>` |
| `src/routes/transfer/+page.svelte` | `<h1>{t().transfer.title}</h1>` |

## 文件修改清单

### src/routes/+layout.svelte

**变更**:
- 根容器保持 `flex h-full`（左右布局）
- main 元素改为 `flex flex-1 flex-col`（列布局）
- TabNavigation 移到 main 内部顶部
- 内容区域包装在 `flex-1 overflow-hidden` 的 div 中

**新结构**:
```svelte
<div class="flex h-full bg-slate-50 dark:bg-slate-900">
  <Sidebar />
  <main class="flex flex-1 flex-col overflow-hidden pb-18 md:pb-0">
    <TabNavigation />
    <div class="flex-1 overflow-hidden">
      {@render children()}
    </div>
  </main>
</div>
```

### src/routes/manage/+page.svelte

**移除**:
```svelte
<h1 class="text-2xl font-bold text-slate-800 dark:text-slate-200">
  {t().manage.title}
</h1>
```

### src/routes/+page.svelte

**移除**:
```svelte
<h1 class="text-2xl font-bold text-slate-800 dark:text-slate-200">
  {t().common.upload}
</h1>
```

**同时移除**:
- 未使用的导入：`import { t } from "$lib/i18n.svelte";`

### src/routes/transfer/+page.svelte

**移除**:
```svelte
<h1 class="text-2xl font-bold text-slate-800 dark:text-slate-200">
  {t().transfer.title}
</h1>
```

**同时移除**:
- 未使用的导入：`import { t } from "$lib/i18n.svelte";`

## 优势

✅ **清晰的视觉层级**：Tab 工具栏清晰显示当前页面
✅ **避免重复**：移除了重复的标题显示
✅ **更多内容空间**：页面顶部不再有标题占用空间
✅ **一致的导航体验**：所有页面都通过 Tab 切换
✅ **保持左右布局**：熟悉的应用布局结构

## 编译状态

✅ 无诊断错误
✅ 所有导入正确
✅ 代码质量良好

## 测试建议

1. ✅ 验证 Tab 工具栏在右侧顶部显示
2. ✅ 点击各个 Tab 切换页面
3. ✅ 验证当前 Tab 高亮显示
4. ✅ 确认页面内容不显示重复标题
5. ✅ 测试深色模式
6. ✅ 验证左侧导航栏只显示设置
7. ✅ 测试响应式设计

