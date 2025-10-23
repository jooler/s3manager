<script lang="ts">
  import Plyr from "plyr";
  import "plyr/dist/plyr.css";
  import { X } from "lucide-svelte";

  interface Props {
    imageUrl: string;
    fileName: string;
    onClose: () => void;
  }

  let { imageUrl, fileName, onClose }: Props = $props();

  let player: Plyr | null = null;
  let videoContainer: HTMLElement;

  $effect(() => {
    if (imageUrl && videoContainer) {
      initializePlayer();
    }
  });

  function initializePlayer() {
    if (player) {
      player.destroy();
    }

    const video = document.createElement("video");
    video.src = imageUrl;
    video.controls = true;
    video.preload = "metadata";

    videoContainer.innerHTML = "";
    videoContainer.appendChild(video);

    player = new Plyr(video, {
      controls: [
        "play-large",
        "play",
        "progress",
        "current-time",
        "mute",
        "volume",
        "captions",
        "settings",
        "pip",
        "airplay",
        "fullscreen",
      ],
      tooltip: "center",
      invertTime: false,
    });
  }

  function handleClose() {
    if (player) {
      player.destroy();
    }
    onClose();
  }

  function handleBackdropClick(e: MouseEvent) {
    if (e.target === e.currentTarget) {
      handleClose();
    }
  }

  $effect(() => {
    return () => {
      if (player) {
        player.destroy();
      }
    };
  });
</script>

<div
  class="fixed inset-0 z-50 flex items-center justify-center bg-black/80 backdrop-blur-sm"
  onclick={handleBackdropClick}
>
  <div class="relative w-full max-w-4xl mx-4">
    <!-- 关闭按钮 -->
    <button
      onclick={(e) => {
        e.stopPropagation();
        handleClose();
      }}
      class="absolute -top-10 right-0 text-white hover:text-gray-300 transition-colors p-2"
      title="关闭预览"
    >
      <X size={24} />
    </button>

    <!-- 视频容器 -->
    <div
      bind:this={videoContainer}
      onclick={(e) => e.stopPropagation()}
      class="bg-black rounded-lg overflow-hidden shadow-2xl"
    >
      <!-- 视频元素将通过 JavaScript 动态创建 -->
    </div>

    <!-- 文件名 -->
    <div class="text-center mt-4 text-white text-sm truncate">
      {fileName}
    </div>
  </div>
</div>

<style>
  :global(.plyr) {
    width: 100%;
    height: auto;
    aspect-ratio: 16/9;
  }

  :global(.plyr__video-wrapper) {
    background: black;
  }
</style>