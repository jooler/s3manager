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

