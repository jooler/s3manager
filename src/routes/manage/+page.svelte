<script lang="ts">
  import { t } from "$lib/i18n.svelte";
  import { globalState, setAlert } from "$lib/store.svelte";
  import type { S3Object, MultipartUpload } from "$lib/type";
  import { invoke } from "@tauri-apps/api/core";
  import { RefreshCw, Download, Trash2, Copy, Eye, Play } from "lucide-svelte";
  import ImagePreview from "$lib/components/ImagePreview.svelte";
  import VideoPreview from "$lib/components/VideoPreview.svelte";
  import { generateOSSPresignedUrl, isOSSBucket } from "$lib/oss-client";

  let files: S3Object[] = $state([]);
  let multipartUploads: MultipartUpload[] = $state([]);
  let loading = $state(false);
  let error: string | null = $state(null);
  let currentPage = $state(1);
  let pageSize = $state(100);
  let totalCount = $state(0);
  let continuationToken: string | undefined = $state();
  let nextContinuationToken: string | undefined = $state();
  let continuationTokenHistory: (string | undefined)[] = $state([]); // 记录每一页的 token
  let previewImageUrl: string | null = $state(null);
  let previewFileName: string | null = $state(null);
  let previewVideoUrl: string | null = $state(null);
  let previewVideoFileName: string | null = $state(null);
  let lastLoadedBucketId: number | undefined = $state(undefined);

  const pageSizeOptions = [25, 50, 100, 200, 500];

  $effect(() => {
    // 当激活的存储桶改变时，重新加载数据
    // 使用 lastLoadedBucketId 避免重复加载
    console.log("$effect triggered:", {
      activeSelectedBucketId: globalState.activeSelectedBucketId,
      selectedBucket: globalState.selectedBucket?.value.bucketName,
      lastLoadedBucketId,
      willLoad: globalState.activeSelectedBucketId &&
                globalState.selectedBucket &&
                globalState.activeSelectedBucketId !== lastLoadedBucketId,
    });

    if (
      globalState.activeSelectedBucketId &&
      globalState.selectedBucket &&
      globalState.activeSelectedBucketId !== lastLoadedBucketId
    ) {
      console.log("✅ Bucket changed, loading data for:", {
        bucketId: globalState.activeSelectedBucketId,
        bucketName: globalState.selectedBucket.value.bucketName,
        previousBucketId: lastLoadedBucketId,
      });

      // 先清空数据
      files = [];
      multipartUploads = [];
      totalCount = 0;

      // 重置分页
      currentPage = 1;
      continuationToken = undefined;
      nextContinuationToken = undefined;
      continuationTokenHistory = [];

      // 记录当前加载的存储桶 ID
      lastLoadedBucketId = globalState.activeSelectedBucketId;

      // 加载数据
      loadData();
    }
  });

  async function loadData() {
    if (!globalState.selectedBucket) {
      setAlert(t().common.noBucketWarning);
      return;
    }

    // 防止重复加载
    if (loading) {
      console.log("Already loading, skipping duplicate request");
      return;
    }

    loading = true;
    error = null;

    try {
      const bucket = globalState.selectedBucket.value;

      console.log("Loading data for bucket:", {
        bucketId: bucket.id,
        bucketName: bucket.bucketName,
        endpoint: bucket.endpoint,
        isOSS: bucket.endpoint?.includes("aliyuncs.com"),
        currentPage,
        pageSize,
        stackTrace: new Error().stack?.split('\n').slice(2, 4).join('\n'), // 显示调用栈
      });

      // Load files
      const filesResponse = await invoke("r2_list_objects", {
        bucketName: bucket.bucketName,
        accountId: bucket.accountId,
        accessKey: bucket.accessKey,
        secretKey: bucket.secretKey,
        maxKeys: pageSize,
        continuationToken: currentPage === 1 ? undefined : continuationToken,
        endpoint: bucket.endpoint || undefined,
      });

      files = (filesResponse as any).objects.sort(
        (a: S3Object, b: S3Object) => b.lastModified - a.lastModified
      );
      totalCount = (filesResponse as any).totalCount;
      nextContinuationToken = (filesResponse as any).continuationToken;

      console.log("Loaded files:", {
        filesCount: files.length,
        totalCount,
        nextContinuationToken,
        hasNextPage: !!nextContinuationToken,
      });

      // Load multipart uploads
      const uploadsResponse = await invoke("r2_list_multipart_uploads", {
        bucketName: bucket.bucketName,
        accountId: bucket.accountId,
        accessKey: bucket.accessKey,
        secretKey: bucket.secretKey,
        endpoint: bucket.endpoint || undefined,
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
        endpoint: bucket.endpoint || undefined,
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
        endpoint: bucket.endpoint || undefined,
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
      // 保存当前页的 token 到历史记录
      continuationTokenHistory.push(continuationToken);

      currentPage++;
      continuationToken = nextContinuationToken;
      loadData();
    }
  }

  function previousPage() {
    if (currentPage > 1) {
      currentPage--;

      // 从历史记录中恢复上一页的 token
      continuationToken = continuationTokenHistory.pop();

      loadData();
    }
  }

  function changePageSize(newSize: number) {
    pageSize = newSize;
    currentPage = 1;
    continuationToken = undefined;
    continuationTokenHistory = [];
    loadData();
  }

  function isImageFile(key: string): boolean {
    const imageExtensions = [
      ".jpg",
      ".jpeg",
      ".png",
      ".gif",
      ".webp",
      ".svg",
      ".bmp",
      ".ico",
      ".tiff",
    ];
    const lowerKey = key.toLowerCase();
    return imageExtensions.some((ext) => lowerKey.endsWith(ext));
  }

  function isVideoFile(key: string): boolean {
    const videoExtensions = [
      ".mp4",
      ".webm",
      ".ogg",
      ".ogv",
      ".avi",
      ".mov",
      ".wmv",
      ".flv",
      ".mkv",
      ".m4v",
      ".3gp",
    ];
    const lowerKey = key.toLowerCase();
    return videoExtensions.some((ext) => lowerKey.endsWith(ext));
  }

  async function previewImage(key: string) {
    try {
      const bucket = globalState.selectedBucket?.value;
      if (!bucket) {
        console.error("No bucket selected");
        return;
      }

      console.log("Previewing image:", {
        key,
        bucketName: bucket.bucketName,
        endpoint: bucket.endpoint,
        isOSS: isOSSBucket(bucket),
      });

      let presignedUrl: string;

      // 判断是 OSS 还是 R2
      if (isOSSBucket(bucket)) {
        // 使用 OSS SDK 生成预签名 URL
        console.log("Using OSS SDK to generate presigned URL");
        presignedUrl = await generateOSSPresignedUrl(bucket, key, 3600);
      } else {
        // 使用后端 Tauri 命令生成 R2 预签名 URL
        console.log("Using Tauri backend to generate R2 presigned URL");
        presignedUrl = await invoke<string>("r2_get_presigned_url", {
          bucketName: bucket.bucketName,
          accountId: bucket.accountId,
          accessKey: bucket.accessKey,
          secretKey: bucket.secretKey,
          key,
          endpoint: bucket.endpoint || undefined,
          expiresIn: 3600, // 1 小时
        });
      }

      console.log("Generated presigned URL:", presignedUrl);

      previewImageUrl = presignedUrl;
      previewFileName = key;
    } catch (e) {
      console.error("Error previewing image:", e);
      const errorMsg = e instanceof Error ? e.message : "Failed to preview image";
      setAlert(errorMsg);
    }
  }

  async function previewVideo(key: string) {
    try {
      const bucket = globalState.selectedBucket?.value;
      if (!bucket) {
        console.error("No bucket selected");
        return;
      }

      console.log("Previewing video:", {
        key,
        bucketName: bucket.bucketName,
        endpoint: bucket.endpoint,
        isOSS: isOSSBucket(bucket),
      });

      let presignedUrl: string;

      // 判断是 OSS 还是 R2
      if (isOSSBucket(bucket)) {
        // 使用 OSS SDK 生成预签名 URL
        console.log("Using OSS SDK to generate presigned URL");
        presignedUrl = await generateOSSPresignedUrl(bucket, key, 3600);
      } else {
        // 使用后端 Tauri 命令生成 R2 预签名 URL
        console.log("Using Tauri backend to generate R2 presigned URL");
        presignedUrl = await invoke<string>("r2_get_presigned_url", {
          bucketName: bucket.bucketName,
          accountId: bucket.accountId,
          accessKey: bucket.accessKey,
          secretKey: bucket.secretKey,
          key,
          endpoint: bucket.endpoint || undefined,
          expiresIn: 3600, // 1 小时
        });
      }

      console.log("Generated presigned URL:", presignedUrl);

      previewVideoUrl = presignedUrl;
      previewVideoFileName = key;
    } catch (e) {
      console.error("Error previewing video:", e);
      const errorMsg = e instanceof Error ? e.message : "Failed to preview video";
      setAlert(errorMsg);
    }
  }
</script>

<div class="flex h-full flex-col p-4 gap-4">

  {#if globalState.selectedBucket}
    <!-- Main Content Area (Flexible Height) -->
    <div class="flex flex-1 flex-col gap-4 overflow-hidden">
      <!-- Combined Files and Multipart Uploads Section -->
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

        {#if files.length === 0 && multipartUploads.length === 0 && !loading}
          <div class="flex flex-1 items-center justify-center p-4 text-center text-slate-500 dark:text-slate-400">
            {t().manage.files.noFiles}
          </div>
        {:else if files.length > 0 || multipartUploads.length > 0}
          <div class="flex flex-1 flex-col overflow-hidden">
            <!-- Table Header (Fixed) -->
            <table class="w-full text-sm flex-shrink-0 table-fixed">
              <colgroup>
                <col style="width: auto;" />
                <col style="width: 120px;" />
                <col style="width: 200px;" />
                <col style="width: 160px;" />
              </colgroup>
              <thead class="border-b border-slate-200 bg-slate-50 dark:border-slate-700 dark:bg-slate-800">
                <tr>
                  <th class="px-4 py-2 text-left">{t().manage.files.name}</th>
                  <th class="px-4 py-2 text-right whitespace-nowrap">{t().manage.files.size}</th>
                  <th class="px-4 py-2 text-left whitespace-nowrap">{t().manage.files.modified}</th>
                  <th class="px-4 py-2 text-right whitespace-nowrap">{t().manage.files.actions}</th>
                </tr>
              </thead>
            </table>

            <!-- Table Body (Scrollable) -->
            <div class="flex-1 overflow-y-auto">
              <table class="w-full text-sm table-fixed">
                <colgroup>
                  <col style="width: auto;" />
                  <col style="width: 120px;" />
                  <col style="width: 200px;" />
                  <col style="width: 160px;" />
                </colgroup>
                <tbody>
                  <!-- Multipart Uploads (显示在最前面) -->
                  {#each multipartUploads as upload}
                    <tr class="border-b border-slate-200 bg-orange-50 hover:bg-orange-100 dark:border-slate-700 dark:bg-orange-900/20 dark:hover:bg-orange-900/30">
                      <td class="px-4 py-2 font-mono text-xs truncate" title={upload.key}>
                        <div class="flex items-center gap-2">
                          <span class="inline-flex items-center rounded-full bg-orange-100 px-2 py-0.5 text-xs font-medium text-orange-800 dark:bg-orange-900/50 dark:text-orange-300">
                            上传中
                          </span>
                          <span>{upload.key}</span>
                        </div>
                      </td>
                      <td class="px-4 py-2 text-right whitespace-nowrap text-slate-400 dark:text-slate-500">-</td>
                      <td class="px-4 py-2 whitespace-nowrap">{formatDate(upload.initiated)}</td>
                      <td class="px-4 py-2 text-right whitespace-nowrap">
                        <button
                          onclick={() => abortUpload(upload.key, upload.uploadId)}
                          class="text-red-500 hover:text-red-700"
                          title={t().manage.multipartUploads.abort}
                        >
                          <Trash2 size={16} />
                        </button>
                      </td>
                    </tr>
                  {/each}

                  <!-- Files (正常文件列表) -->
                  {#each files as file}
                    <tr class="border-b border-slate-200 hover:bg-slate-50 dark:border-slate-700 dark:hover:bg-slate-800">
                      <td class="px-4 py-2 font-mono text-xs truncate" title={file.key}>{file.key}</td>
                      <td class="px-4 py-2 text-right whitespace-nowrap">{formatSize(file.size)}</td>
                      <td class="px-4 py-2 whitespace-nowrap">{formatDate(file.lastModified)}</td>
                      <td class="px-4 py-2 text-right whitespace-nowrap">
                        <div class="flex justify-end gap-2 flex-nowrap">
                          {#if isImageFile(file.key)}
                            <button
                              onclick={() => previewImage(file.key)}
                              title={t().manage.files.preview}
                              class="text-purple-500 hover:text-purple-700"
                            >
                              <Eye size={16} />
                            </button>
                          {/if}
                          {#if isVideoFile(file.key)}
                            <button
                              onclick={() => previewVideo(file.key)}
                              title="预览视频"
                              class="text-green-600 hover:text-green-800"
                            >
                              <Play size={16} />
                            </button>
                          {/if}
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
    </div>
  {/if}
  <!-- Header Section (Fixed Height) -->
  <div class="flex flex-col gap-4">
    {#if !globalState.selectedBucket}
      <div class="rounded-lg bg-yellow-50 p-4 text-yellow-800 dark:bg-yellow-900 dark:text-yellow-200">
        {t().common.noBucketWarning}
      </div>
    {:else}
      <!-- Toolbar -->
      <div class="flex items-center justify-between gap-4 rounded-lg bg-slate-100 p-3 dark:bg-slate-800">
        <!-- 左侧：分页控件 -->
        <div class="flex items-center gap-3">
          <!-- 上一页按钮 -->
          <button
            onclick={previousPage}
            disabled={currentPage === 1 || loading}
            class="flex items-center justify-center w-8 h-8 rounded border border-slate-300 bg-white hover:bg-slate-50 disabled:opacity-30 disabled:cursor-not-allowed dark:border-slate-600 dark:bg-slate-700 dark:hover:bg-slate-600 transition-colors"
            aria-label="上一页"
          >
            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <polyline points="15 18 9 12 15 6"></polyline>
            </svg>
          </button>

          <!-- 当前页显示 -->
          <span class="text-sm text-slate-700 dark:text-slate-300 min-w-[60px] text-center">
            第 {currentPage} 页
          </span>

          <!-- 下一页按钮 -->
          <button
            onclick={nextPage}
            disabled={!nextContinuationToken || loading}
            class="flex items-center justify-center w-8 h-8 rounded border border-slate-300 bg-white hover:bg-slate-50 disabled:opacity-30 disabled:cursor-not-allowed dark:border-slate-600 dark:bg-slate-700 dark:hover:bg-slate-600 transition-colors"
            aria-label="下一页"
          >
            <svg xmlns="http://www.w3.org/2000/svg" width="16" height="16" viewBox="0 0 24 24" fill="none" stroke="currentColor" stroke-width="2" stroke-linecap="round" stroke-linejoin="round">
              <polyline points="9 18 15 12 9 6"></polyline>
            </svg>
          </button>

          <!-- 分隔线 -->
          <div class="w-px h-6 bg-slate-300 dark:bg-slate-600 mx-1"></div>

          <!-- 每页数量选择 -->
          <select
            value={pageSize}
            onchange={(e) => changePageSize(Number(e.currentTarget.value))}
            class="rounded border border-slate-300 bg-white px-3 py-1 text-sm dark:border-slate-600 dark:bg-slate-700 dark:text-white cursor-pointer hover:border-slate-400 dark:hover:border-slate-500 transition-colors"
          >
            {#each pageSizeOptions as size}
              <option value={size}>{size}条/页</option>
            {/each}
          </select>
        </div>

        <!-- 右侧：刷新按钮 -->
        <button
          onclick={() => loadData()}
          disabled={loading}
          class="flex items-center gap-2 rounded bg-blue-500 px-3 py-1 text-sm text-white hover:bg-blue-600 disabled:opacity-50"
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
    {/if}
  </div>
</div>

{#if previewImageUrl && previewFileName}
  <ImagePreview
    imageUrl={previewImageUrl}
    fileName={previewFileName}
    onClose={() => {
      previewImageUrl = null;
      previewFileName = null;
    }}
  />
{/if}

{#if previewVideoUrl && previewVideoFileName}
  <VideoPreview
    imageUrl={previewVideoUrl}
    fileName={previewVideoFileName}
    onClose={() => {
      previewVideoUrl = null;
      previewVideoFileName = null;
    }}
  />
{/if}
