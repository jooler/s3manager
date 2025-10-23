<script lang="ts">
  import { onMount, onDestroy } from "svelte";
  import { UploadCloud } from "lucide-svelte";
  import { listen, type UnlistenFn } from "@tauri-apps/api/event";
  import { globalState, setDragPaths, setIsDragging } from "$lib/store.svelte";
  import { t } from "$lib/i18n.svelte";
  import { page } from "$app/state";
  import { goto } from "$app/navigation";

  let unlistenDrop: UnlistenFn;
  let unlistenLeave: UnlistenFn;

  interface DragEvent {
    event: "string";
    id: number;
    payload: {
      paths: string[];
      position: {
        x: number;
        y: number;
      };
    };
  }

  onMount(async () => {
    unlistenDrop = await listen("tauri://drag-drop", async (event) => {
      setIsDragging(false);
      setDragPaths((event as DragEvent).payload.paths || []);
      // 如果不在首页，跳转到首页，因为首页才能上传
      if (page.route.id !== "/") {
        await goto("/");
      }
    });

    unlistenLeave = await listen("tauri://drag-leave", async (e) => {
      setIsDragging(false);
    });
  });

  onDestroy(() => {
    unlistenDrop();
    unlistenLeave();
  });
</script>

<div class="overlay inset-0" class:active={globalState.drag.isDragging}>
  <div class="flex flex-col items-center gap-4 rounded-xl p-8 text-center">
    <UploadCloud class="size-16 text-slate-300" />
    <p class="text-2xl font-medium text-slate-300">{t().fileDrag.dragHere}</p>
    <p class="text-slate-300">{t().fileDrag.releaseToConfirm}</p>
  </div>
</div>

<style lang="postcss">
  .overlay {
    @apply invisible fixed z-[9999] flex h-screen w-screen items-center justify-center bg-black/50 opacity-0 backdrop-blur-sm transition;
  }

  .overlay.active {
    @apply visible opacity-100;
  }
</style>
