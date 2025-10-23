<script lang="ts">
  import db from "$lib/db";
  import { t } from "$lib/i18n.svelte";
  import { globalState, setAlert } from "$lib/store.svelte";
  import type { UploadHistory } from "$lib/type";
  import { invoke } from "@tauri-apps/api/core";
  import { Copy } from "lucide-svelte";
  import { onMount, untrack } from "svelte";

  const pageSize = 20;
  const tabs: { id: "all" | "in-progress" | "completed"; label: string }[] = [
    { id: "all", label: t().transfer.tabs.all },
    { id: "in-progress", label: t().transfer.tabs.inProgress },
    { id: "completed", label: t().transfer.tabs.completed },
  ];

  let activeTab: "all" | "in-progress" | "completed" = $state("all");
  let currentPage = $state(1);
  let totalItems = $state(0);
  let totalPages = $state(1);
  let files: UploadHistory[] = $state([]);
  let error = $state<string | null>(null);
  let inProgressFiles = $derived(Object.values(globalState.progress));
  let completedFiles: UploadHistory[] = $state([]);

  onMount(async () => {
    await getCompletedFiles();
    loadFiles();
  });

  $effect(() => {
    activeTab;
    currentPage;
    inProgressFiles;
    untrack(loadFiles);
  });

  async function loadFiles() {
    try {
      error = null;

      if (activeTab === "in-progress") {
        files = inProgressFiles;
        totalItems = files.length;
        totalPages = 1;
      } else if (activeTab === "completed") {
        await getCompletedFiles();
        files = completedFiles;
      } else {
        // all
        await getCompletedFiles();
        files = [...inProgressFiles, ...completedFiles];
      }
    } catch (e) {
      error = e instanceof Error ? e.message : "Failed to load files";
      console.error("Error loading files:", e);
    }
  }

  async function getCompletedFiles() {
    const offset = (currentPage - 1) * pageSize;
    const count = await db.history.count();
    totalItems = count;
    totalPages = Math.ceil(count / pageSize);
    completedFiles = await db.history
      .orderBy("timestamp")
      .reverse()
      .offset(offset)
      .limit(pageSize)
      .toArray();
  }

  async function copyLink(url: string) {
    try {
      await navigator.clipboard.writeText(url);
      setAlert(t().fileUploader.uploadStatus.copySuccess);
    } catch (e) {
      setAlert(t().fileUploader.uploadStatus.copyFailed);
    }
  }

  async function cancelUpload(fileId: string) {
    try {
      await invoke("r2_cancel_upload", { fileId });
    } catch (e) {
      console.error(e);
    }
  }
</script>

<div class="flex gap-2">
  {#each tabs as tab}
    <button
      class="nav-link gapped rounded-lg"
      class:bg={activeTab === tab.id}
      aria-current={activeTab === tab.id || undefined}
      onclick={() => {
        activeTab = tab.id;
        currentPage = 1;
      }}
    >
      {tab.label}
    </button>
  {/each}
</div>

<div
  class="flex min-h-0 flex-1 overflow-hidden rounded-lg border border-slate-200 bg-slate-100/80 text-slate-400 dark:border-slate-700 dark:bg-slate-800"
>
  {#if error}
    <div class="flex w-full items-center justify-center text-red-500">
      {error}
    </div>
  {:else if !files.length}
    <div class="flex w-full items-center justify-center">
      {t().fileUploader.uploadStatus.nothing}
    </div>
  {:else}
    <div class="w-full space-y-2 overflow-y-auto p-2">
      {#each files as file}
        <div
          class="flex w-full items-center gap-4 rounded-md bg-slate-50/80 p-2 shadow-sm backdrop-blur-sm transition-all hover:shadow-md dark:bg-slate-700/80"
        >
          <div class="flex flex-1 items-center justify-between">
            <div class="flex-1">
              <div class="text-sm text-slate-500 dark:text-slate-400">
                {file.filename}
              </div>
              {#if typeof file.status === "object" && "uploading" in file.status}
                <div
                  class="h-2 w-full overflow-hidden rounded-full bg-slate-200"
                >
                  <div class="progress-bar">
                    <!-- 已走过的进度 -->
                    <div
                      class="progress-completed"
                      style="width: {file.status.uploading.progress * 100}%"
                    ></div>
                    <!-- 未走过的进度 -->
                    <div
                      class="progress-remaining"
                      style="width: {100 -
                        file.status.uploading.progress * 100}%"
                    ></div>
                  </div>
                </div>
                <div class="mt-1 text-xs text-slate-500">
                  {Math.floor(file.status.uploading.progress * 100)}% -
                  {(file.status.uploading.bytesUploaded / 1024 / 1024).toFixed(
                    2,
                  )}MB /
                  {(file.status.uploading.totalBytes / 1024 / 1024).toFixed(
                    2,
                  )}MB
                  {#if file.status.uploading.speed > 0}
                    - {(file.status.uploading.speed / 1024 / 1024).toFixed(
                      2,
                    )}{t().fileUploader.uploadStatus.speed}
                  {/if}
                </div>
              {:else if file.status === "success"}
                <div class="text-sm">
                  <span class="text-green-500"
                    >{t().fileUploader.uploadStatus.uploadComplete}</span
                  >
                  <span class="text-xs"
                    >{new Date(file.timestamp * 1000).toLocaleString()}</span
                  >
                </div>
              {:else if typeof file.status === "object" && "error" in file.status}
                <div class="text-sm text-red-500">
                  {t().fileUploader.uploadStatus.uploadFailed}{file.status.error
                    .message} ·
                  <span class="text-xs"
                    >{new Date(file.timestamp * 1000).toLocaleString()}</span
                  >
                </div>
              {:else if file.status === "cancelled"}
                <div class="text-sm text-yellow-500">
                  {t().fileUploader.uploadStatus.cancelled} ·
                  <span class="text-xs"
                    >{new Date(file.timestamp * 1000).toLocaleString()}</span
                  >
                </div>
              {:else}
                <div class="text-sm text-slate-500">
                  {t().fileUploader.uploadStatus.waiting} ·
                  <span class="text-xs"
                    >{new Date(file.timestamp * 1000).toLocaleString()}</span
                  >
                </div>
              {/if}
            </div>
            <div class="flex items-center gap-2 px-2">
              {#if typeof file.status === "object" && "uploading" in file.status}
                <button
                  class="cursor-pointer rounded-md bg-red-50 px-3 py-1 text-sm text-red-600 transition-colors hover:bg-red-100 dark:bg-red-900/20 dark:text-red-400 dark:hover:bg-red-900/40"
                  onclick={() => cancelUpload(file.fileId)}
                >
                  {t().fileUploader.uploadStatus.cancel}
                </button>
              {/if}

              {#if (typeof file.status === "object" && "uploading" in file.status) || file.status === "success"}
                <button
                  class="action-button"
                  onclick={() => copyLink(file.url)}
                >
                  <Copy class="size-4" />
                </button>
              {/if}
            </div>
          </div>
        </div>
      {/each}

      <!-- Pagination controls -->
      <div class="flex items-center justify-between">
        <div class="text-sm text-slate-600 dark:text-slate-400">
          {t().fileUploader.uploadStatus.total}: {totalItems} ·
          {t().fileUploader.uploadStatus.page}: {currentPage}/{totalPages}
        </div>
        <div class="flex gap-2">
          <button
            class="rounded px-3 py-1 text-sm {currentPage === 1
              ? 'cursor-not-allowed bg-slate-100 text-slate-400 dark:bg-slate-800'
              : 'cursor-pointer bg-blue-500 text-white hover:bg-blue-600'}"
            disabled={currentPage === 1}
            onclick={() => {
              currentPage--;
            }}
          >
            {t().fileUploader.uploadStatus.previous}
          </button>
          <button
            class="rounded px-3 py-1 text-sm {currentPage === totalPages
              ? 'cursor-not-allowed bg-slate-100 text-slate-400 dark:bg-slate-800'
              : 'cursor-pointer bg-blue-500 text-white hover:bg-blue-600'}"
            disabled={currentPage === totalPages}
            onclick={() => {
              currentPage++;
            }}
          >
            {t().fileUploader.uploadStatus.next}
          </button>
        </div>
      </div>
    </div>
  {/if}
</div>

<style lang="postcss">
  .action-button {
    @apply cursor-pointer rounded-md p-1 text-slate-500 backdrop-blur-sm transition-all hover:bg-slate-200/50 hover:shadow-sm dark:text-slate-400 dark:hover:bg-slate-600/50;
  }
  .nav-link {
    @apply flex cursor-pointer items-center text-slate-700 transition-colors dark:text-slate-200;
  }

  .gapped {
    @apply px-4 py-2;
  }

  .nav-link[aria-current] {
    @apply text-cyan-600 dark:text-cyan-400;
  }

  .nav-link[aria-current].bg {
    @apply bg-cyan-50 dark:bg-cyan-900/30;
  }

  .progress-bar {
    @apply h-2 w-full overflow-hidden rounded-full bg-slate-200 dark:bg-slate-700;
    position: relative;
  }

  .progress-completed {
    @apply h-full bg-blue-500 dark:bg-blue-600;
    position: absolute;
    top: 0;
    left: 0;
    transition: width 0.3s ease;
  }

  .progress-remaining {
    @apply h-full bg-slate-200 dark:bg-slate-700;
    position: absolute;
    top: 0;
    right: 0;
    background: repeating-linear-gradient(
      45deg,
      rgba(255, 255, 255, 0.15),
      rgba(255, 255, 255, 0.15) 10px,
      transparent 10px,
      transparent 20px
    );
    background-size: 28px 28px;
    animation: progress-animation 1s linear infinite;
  }

  @keyframes progress-animation {
    0% {
      background-position: 0 0;
    }
    100% {
      background-position: 28px 0;
    }
  }
</style>
