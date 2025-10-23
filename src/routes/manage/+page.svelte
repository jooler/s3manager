<script lang="ts">
  import { t } from "$lib/i18n.svelte";
  import { globalState, setAlert } from "$lib/store.svelte";
  import type { S3Object, MultipartUpload } from "$lib/type";
  import { invoke } from "@tauri-apps/api/core";
  import { RefreshCw, Download, Trash2, Copy } from "lucide-svelte";
  import { onMount } from "svelte";

  let files: S3Object[] = $state([]);
  let multipartUploads: MultipartUpload[] = $state([]);
  let loading = $state(false);
  let error: string | null = $state(null);
  let currentPage = $state(1);
  let pageSize = $state(10);
  let totalCount = $state(0);
  let continuationToken: string | undefined = $state();
  let nextContinuationToken: string | undefined = $state();

  const pageSizeOptions = [10, 50, 100];

  onMount(() => {
    loadData();
  });

  $effect(() => {
    // 当激活的存储桶改变时，重新加载数据
    if (globalState.activeSelectedBucketId) {
      currentPage = 1;
      continuationToken = undefined;
      loadData();
    }
  });

  async function loadData() {
    if (!globalState.selectedBucket) {
      setAlert(t().common.noBucketWarning);
      return;
    }

    loading = true;
    error = null;

    try {
      const bucket = globalState.selectedBucket.value;
      
      // Load files
      const filesResponse = await invoke("r2_list_objects", {
        bucketName: bucket.bucketName,
        accountId: bucket.accountId,
        accessKey: bucket.accessKey,
        secretKey: bucket.secretKey,
        maxKeys: pageSize,
        continuationToken: currentPage === 1 ? undefined : continuationToken,
      });

      files = (filesResponse as any).objects.sort(
        (a: S3Object, b: S3Object) => b.lastModified - a.lastModified
      );
      totalCount = (filesResponse as any).totalCount;
      nextContinuationToken = (filesResponse as any).continuationToken;

      // Load multipart uploads
      const uploadsResponse = await invoke("r2_list_multipart_uploads", {
        bucketName: bucket.bucketName,
        accountId: bucket.accountId,
        accessKey: bucket.accessKey,
        secretKey: bucket.secretKey,
      });

      multipartUploads = (uploadsResponse as any).uploads;
    } catch (e) {
      error = e instanceof Error ? e.message : "Failed to load data";
      console.error("Error loading data:", e);
    } finally {
      loading = false;
    }
  }

  async function deleteFile(key: string) {
    if (!confirm(t().manage.files.deleteConfirm)) return;

    if (!globalState.selectedBucket) return;

    try {
      const bucket = globalState.selectedBucket.value;
      await invoke("r2_delete_object", {
        bucketName: bucket.bucketName,
        accountId: bucket.accountId,
        accessKey: bucket.accessKey,
        secretKey: bucket.secretKey,
        key,
      });

      setAlert(t().manage.files.deleteSuccess);
      await loadData();
    } catch (e) {
      setAlert(t().manage.files.deleteFailed);
      console.error("Error deleting file:", e);
    }
  }

  async function abortUpload(key: string, uploadId: string) {
    if (!confirm(t().manage.multipartUploads.abortConfirm)) return;

    if (!globalState.selectedBucket) return;

    try {
      const bucket = globalState.selectedBucket.value;
      await invoke("r2_abort_multipart_upload_cmd", {
        bucketName: bucket.bucketName,
        accountId: bucket.accountId,
        accessKey: bucket.accessKey,
        secretKey: bucket.secretKey,
        key,
        uploadId,
      });

      setAlert(t().manage.multipartUploads.abortSuccess);
      await loadData();
    } catch (e) {
      setAlert(t().manage.multipartUploads.abortFailed);
      console.error("Error aborting upload:", e);
    }
  }

  async function copyUrl(key: string) {
    try {
      const bucket = globalState.selectedBucket?.value;
      if (!bucket) return;

      const domain = bucket.customDomain || `https://${bucket.accountId}.r2.cloudflarestorage.com`;
      const url = `${domain}/${key}`;
      await navigator.clipboard.writeText(url);
      setAlert(t().manage.files.copySuccess);
    } catch (e) {
      setAlert(t().manage.files.copyFailed);
    }
  }

  async function downloadFile(key: string) {
    try {
      const bucket = globalState.selectedBucket?.value;
      if (!bucket) return;

      const domain = bucket.customDomain || `https://${bucket.accountId}.r2.cloudflarestorage.com`;
      const url = `${domain}/${key}`;

      // Open in browser
      window.open(url, "_blank");
    } catch (e) {
      console.error("Error downloading file:", e);
    }
  }

  function formatSize(bytes: number): string {
    if (bytes === 0) return "0 B";
    const k = 1024;
    const sizes = ["B", "KB", "MB", "GB"];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return Math.round((bytes / Math.pow(k, i)) * 100) / 100 + " " + sizes[i];
  }

  function formatDate(timestamp: number): string {
    return new Date(timestamp * 1000).toLocaleString();
  }

  function nextPage() {
    if (nextContinuationToken) {
      currentPage++;
      continuationToken = nextContinuationToken;
      loadData();
    }
  }

  function previousPage() {
    if (currentPage > 1) {
      currentPage--;
      continuationToken = undefined;
      loadData();
    }
  }

  function changePageSize(newSize: number) {
    pageSize = newSize;
    currentPage = 1;
    continuationToken = undefined;
    loadData();
  }
</script>

<div class="flex h-full flex-col p-4 gap-4">
  <!-- Header Section (Fixed Height) -->
  <div class="flex flex-col gap-4">
    {#if !globalState.selectedBucket}
      <div class="rounded-lg bg-yellow-50 p-4 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-200">
        {t().common.noBucketWarning}
      </div>
    {:else}
      <!-- Toolbar -->
      <div class="flex items-center gap-4 rounded-lg bg-slate-100 p-3 dark:bg-slate-800">
        <div class="flex items-center gap-2">
          <label for="page-size" class="text-sm text-slate-600 dark:text-slate-400">
            {t().manage.toolbar.pageSize}:
          </label>
          <select
            id="page-size"
            value={pageSize}
            onchange={(e) => changePageSize(Number(e.currentTarget.value))}
            class="rounded border border-slate-300 bg-white px-2 py-1 text-sm dark:border-slate-600 dark:bg-slate-700 dark:text-white"
          >
            {#each pageSizeOptions as size}
              <option value={size}>{size}</option>
            {/each}
          </select>
        </div>

        <button
          onclick={() => loadData()}
          disabled={loading}
          class="ml-auto flex items-center gap-2 rounded bg-blue-500 px-3 py-1 text-sm text-white hover:bg-blue-600 disabled:opacity-50"
        >
          <RefreshCw size={16} />
          {t().manage.toolbar.refresh}
        </button>
      </div>

      {#if error}
        <div class="rounded-lg bg-red-50 p-4 text-red-800 dark:bg-red-900 dark:text-red-200">
          {error}
        </div>
      {/if}

      <!-- Multipart Uploads Section -->
      {#if multipartUploads.length > 0}
        <div class="rounded-lg border border-slate-200 dark:border-slate-700">
          <div class="border-b border-slate-200 bg-slate-50 p-3 dark:border-slate-700 dark:bg-slate-800">
            <h2 class="font-semibold text-slate-800 dark:text-slate-200">
              {t().manage.multipartUploads.title}
            </h2>
          </div>
          <div class="overflow-x-auto">
            <table class="w-full text-sm">
              <thead class="border-b border-slate-200 bg-slate-50 dark:border-slate-700 dark:bg-slate-800">
                <tr>
                  <th class="px-4 py-2 text-left">{t().manage.multipartUploads.name}</th>
                  <th class="px-4 py-2 text-left">{t().manage.multipartUploads.initiated}</th>
                  <th class="px-4 py-2 text-right">{t().manage.multipartUploads.actions}</th>
                </tr>
              </thead>
              <tbody>
                {#each multipartUploads as upload}
                  <tr class="border-b border-slate-200 hover:bg-slate-50 dark:border-slate-700 dark:hover:bg-slate-800">
                    <td class="px-4 py-2">{upload.key}</td>
                    <td class="px-4 py-2">{formatDate(upload.initiated)}</td>
                    <td class="px-4 py-2 text-right">
                      <button
                        onclick={() => abortUpload(upload.key, upload.uploadId)}
                        class="text-red-500 hover:text-red-700"
                      >
                        {t().manage.multipartUploads.abort}
                      </button>
                    </td>
                  </tr>
                {/each}
              </tbody>
            </table>
          </div>
        </div>
      {/if}
    {/if}
  </div>

  {#if globalState.selectedBucket}
    <!-- Main Content Area (Flexible Height) -->
    <div class="flex flex-1 flex-col gap-4 overflow-hidden">
      <!-- Files Section -->
      <div class="relative flex flex-1 flex-col gap-4 rounded-lg border border-slate-200 dark:border-slate-700 overflow-hidden">
        {#if loading}
          <!-- Loading Overlay -->
          <div class="absolute inset-0 z-10 flex items-center justify-center bg-white/50 dark:bg-slate-900/50 backdrop-blur-sm">
            <div class="flex flex-col items-center gap-3">
              <div class="h-8 w-8 animate-spin rounded-full border-4 border-slate-300 border-t-blue-500 dark:border-slate-600 dark:border-t-blue-400"></div>
              <p class="text-sm text-slate-600 dark:text-slate-400">{t().common.loading}</p>
            </div>
          </div>
        {/if}

        {#if files.length === 0 && !loading}
          <div class="flex flex-1 items-center justify-center p-4 text-center text-slate-500 dark:text-slate-400">
            {t().manage.files.noFiles}
          </div>
        {:else if files.length > 0}
          <div class="flex flex-1 flex-col overflow-hidden">
            <!-- Table Header (Fixed) -->
            <table class="w-full text-sm flex-shrink-0">
              <thead class="border-b border-slate-200 bg-slate-50 dark:border-slate-700 dark:bg-slate-800">
                <tr>
                  <th class="px-4 py-2 text-left">{t().manage.files.name}</th>
                  <th class="px-4 py-2 text-right">{t().manage.files.size}</th>
                  <th class="px-4 py-2 text-left">{t().manage.files.modified}</th>
                  <th class="px-4 py-2 text-right">{t().manage.files.actions}</th>
                </tr>
              </thead>
            </table>

            <!-- Table Body (Scrollable) -->
            <div class="flex-1 overflow-y-auto">
              <table class="w-full text-sm">
                <tbody>
                  {#each files as file}
                    <tr class="border-b border-slate-200 hover:bg-slate-50 dark:border-slate-700 dark:hover:bg-slate-800">
                      <td class="px-4 py-2 font-mono text-xs">{file.key}</td>
                      <td class="px-4 py-2 text-right">{formatSize(file.size)}</td>
                      <td class="px-4 py-2">{formatDate(file.lastModified)}</td>
                      <td class="px-4 py-2 text-right">
                        <div class="flex justify-end gap-2">
                          <button
                            onclick={() => copyUrl(file.key)}
                            title={t().manage.files.copyUrl}
                            class="text-blue-500 hover:text-blue-700"
                          >
                            <Copy size={16} />
                          </button>
                          <button
                            onclick={() => downloadFile(file.key)}
                            title={t().manage.files.download}
                            class="text-green-500 hover:text-green-700"
                          >
                            <Download size={16} />
                          </button>
                          <button
                            onclick={() => deleteFile(file.key)}
                            title={t().manage.files.delete}
                            class="text-red-500 hover:text-red-700"
                          >
                            <Trash2 size={16} />
                          </button>
                        </div>
                      </td>
                    </tr>
                  {/each}
                </tbody>
              </table>
            </div>
          </div>
        {/if}
      </div>

      <!-- Pagination (Fixed Height) -->
      <div class="flex items-center justify-between rounded-lg bg-slate-100 p-3 dark:bg-slate-800 flex-shrink-0">
        <button
          onclick={previousPage}
          disabled={currentPage === 1 || loading}
          class="rounded bg-slate-300 px-3 py-1 text-sm hover:bg-slate-400 disabled:opacity-50 dark:bg-slate-600 dark:hover:bg-slate-500"
        >
          {t().manage.pagination.previous}
        </button>

        <span class="text-sm text-slate-600 dark:text-slate-400">
          {t().manage.pagination.page} {currentPage}
        </span>

        <button
          onclick={nextPage}
          disabled={!nextContinuationToken || loading}
          class="rounded bg-slate-300 px-3 py-1 text-sm hover:bg-slate-400 disabled:opacity-50 dark:bg-slate-600 dark:hover:bg-slate-500"
        >
          {t().manage.pagination.next}
        </button>
      </div>
    </div>
  {/if}
</div>

