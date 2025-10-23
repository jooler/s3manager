<script lang="ts">
  import { checkClipboardContent } from "$lib/tools";
  import { onDestroy, onMount } from "svelte";
  import FileUploaderReady from "./FileUploaderReady.svelte";
  import FileUploader from "./FileUploader.svelte";
  import { globalState } from "$lib/store.svelte";
  import { t } from "$lib/i18n.svelte";

  onMount(async () => {
    window.addEventListener("keydown", handleKeyDown);
  });

  onDestroy(() => {
    window.removeEventListener("keydown", handleKeyDown);
  });

  // 监听 ctrl+v
  function handleKeyDown(e: KeyboardEvent) {
    // 如果当前焦点是输入框，则不处理
    if (
      e.target instanceof HTMLInputElement ||
      e.target instanceof HTMLTextAreaElement
    ) {
      return;
    }

    // 处理 ctrl+v
    if ((e.ctrlKey || e.metaKey) && e.key === "v") {
      e.preventDefault();
      checkClipboardContent();
    }
  }
</script>

<div
  class="flex min-h-0 flex-1 flex-col items-center justify-center rounded-lg border border-slate-200 bg-slate-100/80 text-slate-400 dark:border-slate-700 dark:bg-slate-800"
>
  {#if !globalState.selectedBucket}
    <p class="dark:text-slate-300">{t().common.noBucketWarning}</p>
  {:else if !globalState.files.length}
    <FileUploaderReady />
  {:else}
    <FileUploader />
  {/if}
</div>
