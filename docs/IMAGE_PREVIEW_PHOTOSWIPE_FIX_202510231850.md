# 图片预览 PhotoSwipe 初始化修复

## 📋 问题

预签名 URL 已经成功返回，但是 PhotoSwipe 没有正确显示图片。

## 🔍 根本原因

### 1. PhotoSwipe 5 API 变化
- PhotoSwipe 5 使用 `width` 和 `height` 而不是 `w` 和 `h`
- 需要先加载图片获取尺寸，然后再初始化 PhotoSwipe

### 2. 初始化时机问题
- 原代码在图片加载完成前就初始化了 PhotoSwipe
- 导致 PhotoSwipe 无法正确显示图片

## ✅ 解决方案

### 修改 ImagePreview 组件

**文件**: `src/lib/components/ImagePreview.svelte`

#### 关键改动

1. **先加载图片获取尺寸**
```typescript
const img = new Image();
img.onload = () => {
  // 图片加载完成后再初始化 PhotoSwipe
  const items = [
    {
      src: imageUrl,
      width: img.width,   // 使用 width 而不是 w
      height: img.height, // 使用 height 而不是 h
      alt: fileName,
    },
  ];
  
  // 创建 PhotoSwipe 实例
  pswp = new PhotoSwipe({
    dataSource: items,
    index: 0,
    wheelToZoom: true,
    showHideAnimationType: "fade",
    bgOpacity: 0.95,
    padding: { top: 20, bottom: 20, left: 20, right: 20 },
  });
  
  pswp.on("close", () => {
    if (onClose) onClose();
  });
  
  pswp.init();
};

img.src = imageUrl;
```

2. **错误处理**
```typescript
img.onerror = () => {
  console.error("Failed to load image:", imageUrl);
  if (onClose) {
    onClose();
  }
};
```

3. **清理资源**
```typescript
return () => {
  if (pswp) {
    pswp.destroy();
  }
};
```

## 📊 工作流程

```
用户点击预览按钮
    ↓
调用 previewImage(key)
    ↓
获取预签名 URL
    ↓
设置 previewImageUrl 和 previewFileName
    ↓
ImagePreview 组件挂载
    ↓
创建 Image 对象加载图片
    ↓
图片加载完成（img.onload）
    ↓
获取图片尺寸（img.width, img.height）
    ↓
创建 PhotoSwipe 实例
    ↓
初始化 PhotoSwipe（pswp.init()）
    ↓
显示全屏图片预览
```

## 🔧 完整代码

```svelte
<script lang="ts">
  import { onMount } from "svelte";
  import PhotoSwipe from "photoswipe";
  import "photoswipe/style.css";

  let {
    imageUrl,
    fileName,
    onClose,
  }: {
    imageUrl: string;
    fileName: string;
    onClose?: () => void;
  } = $props();

  let pswpElement: HTMLDivElement;
  let pswp: PhotoSwipe | null = null;

  onMount(() => {
    // 先加载图片获取尺寸
    const img = new Image();
    img.onload = () => {
      // 创建 PhotoSwipe 实例
      const items = [
        {
          src: imageUrl,
          width: img.width,
          height: img.height,
          alt: fileName,
        },
      ];

      const options = {
        dataSource: items,
        index: 0,
        wheelToZoom: true,
        showHideAnimationType: "fade" as const,
        bgOpacity: 0.95,
        padding: {
          top: 20,
          bottom: 20,
          left: 20,
          right: 20,
        },
      };

      pswp = new PhotoSwipe(options);

      // 监听关闭事件
      pswp.on("close", () => {
        if (onClose) {
          onClose();
        }
      });

      pswp.init();
    };

    img.onerror = () => {
      console.error("Failed to load image:", imageUrl);
      if (onClose) {
        onClose();
      }
    };

    img.src = imageUrl;

    return () => {
      if (pswp) {
        pswp.destroy();
      }
    };
  });
</script>

<div bind:this={pswpElement} class="pswp-container"></div>

<style>
  :global(.pswp) {
    --pswp-bg: rgba(0, 0, 0, 0.95);
    --pswp-root-z-index: 9999;
  }

  :global(.pswp__button) {
    color: white;
    opacity: 0.8;
    transition: opacity 0.2s;
  }

  :global(.pswp__button:hover) {
    opacity: 1;
  }

  :global(.pswp__caption__center) {
    color: rgba(255, 255, 255, 0.8);
    font-size: 14px;
  }
</style>
```

## 📝 关键点

### PhotoSwipe 5 数据格式

| 属性 | 类型 | 说明 |
|------|------|------|
| `src` | string | 图片 URL（必需） |
| `width` | number | 图片宽度（必需） |
| `height` | number | 图片高度（必需） |
| `alt` | string | 图片描述（可选） |
| `srcset` | string | 响应式图片（可选） |

### 初始化选项

| 选项 | 值 | 说明 |
|------|-----|------|
| `dataSource` | array | 图片数据数组 |
| `index` | number | 起始索引（默认 0） |
| `wheelToZoom` | boolean | 鼠标滚轮缩放 |
| `showHideAnimationType` | string | 动画类型（fade/zoom/none） |
| `bgOpacity` | number | 背景透明度（0-1） |
| `padding` | object | 内边距 |

## ✅ 编译状态

- ✅ 前端：无诊断错误
- ✅ 所有导入正确
- ✅ 代码质量良好

## 🧪 测试清单

- [ ] 点击图片预览按钮
- [ ] 验证 PhotoSwipe 正确打开
- [ ] 验证图片正确显示
- [ ] 测试缩放功能（鼠标滚轮）
- [ ] 测试平移功能（拖拽）
- [ ] 测试关闭功能（ESC 或关闭按钮）
- [ ] 测试不同图片格式（JPG, PNG, GIF, WebP）
- [ ] 测试大图片（> 5MB）
- [ ] 测试小图片（< 100KB）
- [ ] 测试深色模式
- [ ] 测试国际化（英文/中文）

## 🔄 完整流程

### 1. 用户操作
```
用户在管理页面看到图片文件
    ↓
点击预览按钮（紫色眼睛图标）
```

### 2. 前端处理
```
previewImage(key) 函数被调用
    ↓
调用 r2_get_presigned_url 命令
    ↓
传递参数：bucketName, accountId, accessKey, secretKey, key, endpoint
```

### 3. 后端处理
```
R2Client.get_presigned_url() 方法被调用
    ↓
使用 AWS SDK 生成预签名 URL
    ↓
返回包含鉴权信息的 URL
```

### 4. 前端显示
```
接收预签名 URL
    ↓
设置 previewImageUrl 和 previewFileName
    ↓
ImagePreview 组件挂载
    ↓
加载图片获取尺寸
    ↓
初始化 PhotoSwipe
    ↓
显示全屏预览
```

## 📚 相关文档

- PhotoSwipe 5 文档：https://photoswipe.com/
- PhotoSwipe 数据源：https://photoswipe.com/data-sources/
- PhotoSwipe 选项：https://photoswipe.com/options/
- PhotoSwipe 事件：https://photoswipe.com/events/

## ✨ 特点总结

- ✅ **正确的初始化顺序**：先加载图片，再初始化 PhotoSwipe
- ✅ **正确的数据格式**：使用 width/height 而不是 w/h
- ✅ **错误处理**：图片加载失败时自动关闭
- ✅ **资源清理**：组件卸载时销毁 PhotoSwipe 实例
- ✅ **事件监听**：正确监听关闭事件
- ✅ **预签名 URL**：使用标准 S3 预签名 URL 方案


