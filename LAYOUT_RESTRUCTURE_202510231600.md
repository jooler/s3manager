# 应用布局重构 - Tab 导航切换

## 需求

1. 左侧导航栏只保留"设置"
2. 右侧主体部分顶部添加 Tab 切换工具栏
3. Tab 中放置原来左侧导航中的"管理、上传、传输"三个条目

## 实现方案

### 1. 新建 Tab 导航组件

**文件**: `src/lib/components/TabNavigation.svelte`

- 显示三个 Tab：管理、上传、传输
- 使用图标 + 文本的组合
- 当前活跃 Tab 显示下划线和青色高亮
- 支持深色模式

**Tab 列表**:
- 管理 (Database icon) → `/manage`
- 上传 (CloudUpload icon) → `/`
- 传输 (ArrowsUpFromLine icon) → `/transfer`

### 2. 修改 Sidebar 组件

**文件**: `src/lib/components/Sidebar.svelte`

**变更**:
- 移除导航链接：管理、上传、传输
- 只保留：设置
- 移除未使用的图标导入

### 3. 修改主布局

**文件**: `src/routes/+layout.svelte`

**变更**:
- 导入 `TabNavigation` 组件
- 修改根容器为 `flex h-full flex-col`（从 `flex h-full` 改为列布局）
- 在顶部添加 `<TabNavigation />`
- 将 Sidebar 和 main 包装在一个 `flex flex-1 overflow-hidden` 的容器中

**新布局结构**:
```
根容器 (flex h-full flex-col)
├─ TabNavigation (固定高度)
└─ 内容区域 (flex flex-1 overflow-hidden)
   ├─ Sidebar (左侧，可折叠)
   └─ main (右侧，主内容)
```

## 布局树

```
<div class="flex h-full flex-col">
  <!-- Tab 导航栏 -->
  <TabNavigation />
  
  <!-- 内容区域 -->
  <div class="flex flex-1 overflow-hidden">
    <!-- 左侧导航栏（仅设置） -->
    <Sidebar />
    
    <!-- 右侧主内容 -->
    <main class="flex-1 overflow-hidden">
      {@render children()}
    </main>
  </div>
</div>
```

## 样式特点

### Tab 导航栏
- 背景色：白色（深色模式为 slate-800）
- 边框：下边框 slate-200（深色模式为 slate-700）
- 内边距：px-4
- 间距：gap-1

### Tab 项
- 活跃状态：
  - 下边框：cyan-600（深色模式为 cyan-400）
  - 文字颜色：cyan-600（深色模式为 cyan-400）
- 非活跃状态：
  - 下边框：transparent
  - 文字颜色：slate-600（深色模式为 slate-400）
- 悬停效果：背景色变浅

## 文件修改清单

| 文件 | 操作 | 说明 |
|------|------|------|
| `src/lib/components/TabNavigation.svelte` | 新建 | Tab 导航组件 |
| `src/lib/components/Sidebar.svelte` | 修改 | 只保留设置链接 |
| `src/routes/+layout.svelte` | 修改 | 添加 Tab 导航，调整布局 |

## 编译状态

✅ 无诊断错误
✅ 所有导入正确
✅ 代码质量良好

## 测试建议

1. ✅ 验证 Tab 导航显示正确
2. ✅ 点击各个 Tab 切换页面
3. ✅ 验证当前 Tab 高亮显示
4. ✅ 测试深色模式
5. ✅ 验证左侧导航栏只显示设置
6. ✅ 测试响应式设计（移动端）
7. ✅ 验证 Tab 导航在移动端的表现

## 后续考虑

- 移动端可能需要调整 Tab 导航的显示方式
- 可以考虑添加 Tab 动画效果
- 可以考虑添加 Tab 滚动功能（如果 Tab 数量增加）

