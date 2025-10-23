<script lang="ts">
  import { LinkPreview } from "bits-ui";
  import { Eye } from "lucide-svelte";
  import type { File } from "$lib/type";
  import { invoke } from "@tauri-apps/api/core";
  import { setAlert } from "$lib/store.svelte";
  import { t } from "$lib/i18n.svelte";

  let { file }: { file: File } = $props();

  // 预览相关状态
  let previewContent = $state<string | null>(null);
  let previewLoading = $state(false);
  let previewError = $state<string | null>(null);

  // 预览文件
  async function previewFile(file: File) {
    previewLoading = true;
    previewError = null;
    try {
      if ("filePath" in file.source) {
        const path = file.source.filePath;
        previewContent = await invoke<string>("preview_file", { path });
      } else {
        previewContent = file.source.fileContent || "";
      }
    } catch (error) {
      previewError =
        error instanceof Error
          ? error.message
          : t().fileUploader.preview.previewFailed;
    } finally {
      previewLoading = false;
    }
  }
</script>

<LinkPreview.Root
  openDelay={100}
  closeDelay={100}
  onOpenChange={(isOpen) => {
    if (isOpen) {
      previewFile(file);
    } else {
      previewContent = null;
    }
  }}
>
  <LinkPreview.Trigger>
    <button class="action-button">
      <Eye class="size-4" />
    </button>
  </LinkPreview.Trigger>
  <LinkPreview.Content side="top">
    <div
      class="w-80 space-y-2 rounded-lg bg-slate-50/90 p-2 shadow-md backdrop-blur-sm dark:bg-slate-900/80"
    >
      <div class="flex items-center justify-center">
        {#if previewLoading}
          <div class="flex items-center justify-center p-2">
            <div
              class="size-8 animate-spin rounded-full border-b-2 border-slate-900"
            ></div>
          </div>
        {:else if previewContent}
          {#if file.type === "image"}
            <img
              src={previewContent}
              alt={t().fileUploader.preview.filePreview}
              class="max-h-48 max-w-48 rounded-md object-contain"
            />
          {:else if file.type === "text"}
            <div
              class="max-h-48 overflow-auto rounded-md bg-slate-100/50 p-2 text-sm dark:bg-slate-700/50"
            >
              <p>{previewContent}</p>
            </div>
          {/if}
        {/if}
      </div>

      <div class="space-y-2 text-xs">
        <div class="flex items-center justify-between">
          <span class="text-slate-500 dark:text-slate-400"
            >{t().fileUploader.preview.filename}</span
          >
          <span class="font-medium">{file.remoteFilename}</span>
        </div>
        <div class="flex items-center justify-between">
          <span class="text-slate-500 dark:text-slate-400"
            >{t().fileUploader.preview.remotePath}</span
          >
          <span
            class="rounded bg-slate-100/50 px-2 py-1 font-mono text-slate-700 dark:bg-slate-700/50 dark:text-slate-300"
          >
            {file.remoteFilenamePrefix}/{file.remoteFilename}
          </span>
        </div>
      </div>
    </div>
  </LinkPreview.Content>
</LinkPreview.Root>
