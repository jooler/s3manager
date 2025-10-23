<script lang="ts">
  import Alert from "$lib/components/Alert.svelte";
  import FileDrag from "$lib/components/FileDrag.svelte";
  import Modal from "$lib/components/Modal.svelte";
  import Sidebar from "$lib/components/Sidebar.svelte";

  const { children } = $props<{ children: any }>();
  import db from "$lib/db";
  import {
    globalState,
    initAppSettings,
    setDragPaths,
    setIsDragging,
  } from "$lib/store.svelte";
  import { parsePaths } from "$lib/tools";
  import type { UploadHistory } from "$lib/type";
  import { listen, type UnlistenFn } from "@tauri-apps/api/event";
  import { onDestroy, onMount } from "svelte";
  import "../app.css";

  let unlistenDrag: UnlistenFn;
  let unlistenProgress: UnlistenFn;

  onMount(async () => {
    // initialize settings on load
    initAppSettings();

    // 监听拖拽事件
    unlistenDrag = await listen("tauri://drag-enter", async (event) => {
      setIsDragging(true);
    });

    // 监听上传进度事件
    unlistenProgress = await listen<UploadHistory>(
      "upload-progress",
      (event) => {
        globalState.progress[event.payload.fileId] = event.payload;
        // 如果上传完成了，无论成功还是失败，只要不是 uploading，就从 progress 移除，放入 db
        if (
          typeof event.payload.status !== "object" ||
          !("uploading" in event.payload.status)
        ) {
          db.history.put(event.payload);
          delete globalState.progress[event.payload.fileId];
        }
      },
    );
  });

  onDestroy(() => {
    if (unlistenDrag) {
      unlistenDrag();
    }
    if (unlistenProgress) {
      unlistenProgress();
    }
  });

  $effect(() => {
    // 如果 appSettings 有变化，更新到数据库
    db.appSettings.put({
      id: 1,
      ...$state.snapshot(globalState.appSetting),
    });
    // 监听到拖拽事件后，解析路径，然后清空 dragState.paths
    if (globalState.drag.paths.length > 0) {
      parsePaths(globalState.drag.paths);
      setDragPaths([]);
    }
  });
</script>

<FileDrag />
<Alert />
<Modal />

<div class="flex h-full bg-slate-50 dark:bg-slate-900">
  <Sidebar />
  <main class="flex-1 overflow-hidden pb-18 md:pb-0">
    {@render children()}
  </main>
</div>
