<script lang="ts">
  import db from "$lib/db";
  import { t } from "$lib/i18n.svelte";
  import { globalState } from "$lib/store.svelte";
  import type { Bucket } from "$lib/type";
  import { onMount } from "svelte";

  let buckets: Bucket[] = $state([]);

  onMount(async () => {
    loadBuckets();
  });

  // 监听存储桶刷新信号
  $effect(() => {
    // 当 bucketsRefreshSignal 变化时，重新加载存储桶列表
    globalState.bucketsRefreshSignal;
    loadBuckets();
  });

  async function loadBuckets() {
    buckets = await db.buckets.toArray();
    // 如果没有激活的存储桶，设置为上次激活的或第一个
    if (buckets.length > 0 && !globalState.activeSelectedBucketId) {
      let bucketId: number | undefined;

      // 优先使用上次激活的存储桶
      if (globalState.appSetting.lastActiveBucketId) {
        const lastActiveBucket = buckets.find(
          (b) => b.id === globalState.appSetting.lastActiveBucketId,
        );
        bucketId = lastActiveBucket?.id;
      }

      // 如果上次激活的不存在，使用默认存储桶
      if (!bucketId && globalState.appSetting.defaultBucketId) {
        const defaultBucket = buckets.find(
          (b) => b.id === globalState.appSetting.defaultBucketId,
        );
        bucketId = defaultBucket?.id;
      }

      // 最后使用第一个存储桶
      if (!bucketId) {
        bucketId = buckets[0].id;
      }

      // 找到存储桶对象
      const bucket = buckets.find((b) => b.id === bucketId);
      if (bucket) {
        console.log("Initializing with bucket:", {
          id: bucket.id,
          name: bucket.bucketName,
          endpoint: bucket.endpoint,
        });

        // 先更新 selectedBucket（同步操作）
        globalState.selectedBucket = {
          value: bucket,
          label: bucket.bucketName,
        };

        // 然后更新 activeSelectedBucketId（触发 $effect）
        globalState.activeSelectedBucketId = bucketId;
      }
    }
  }

  async function selectBucket(bucketId: number | undefined) {
    if (!bucketId) return;

    console.log("Selecting bucket:", bucketId);

    // 先找到存储桶对象
    const bucket = buckets.find((b) => b.id === bucketId);
    if (!bucket) {
      console.error("Bucket not found:", bucketId);
      return;
    }

    console.log("Found bucket:", {
      id: bucket.id,
      name: bucket.bucketName,
      endpoint: bucket.endpoint,
      isOSS: bucket.endpoint?.includes("aliyuncs.com"),
    });

    // 先更新 selectedBucket（同步操作）
    globalState.selectedBucket = {
      value: bucket,
      label: bucket.bucketName,
    };

    // 然后更新 activeSelectedBucketId（触发 $effect）
    globalState.activeSelectedBucketId = bucketId;

    // 最后保存到数据库（异步操作，不影响 UI）
    globalState.appSetting.lastActiveBucketId = bucketId;
    const settings = await db.appSettings.get(1);
    if (settings) {
      await db.appSettings.update(1, { lastActiveBucketId: bucketId });
    }
  }
</script>

<div class="flex flex-col {globalState.appSetting.sidebarCollapsed ? 'gap-1' : 'gap-2'}">
  {#if buckets.length === 0}
    <div class="px-2 py-4 text-center text-sm text-slate-500 dark:text-slate-400">
      {t().common.noBucketWarning}
    </div>
  {:else}
    {#each buckets as bucket (bucket.id)}
      <button
        onclick={() => selectBucket(bucket.id)}
        class="flex items-center justify-center rounded-lg transition-colors {globalState.appSetting.sidebarCollapsed
          ? 'h-10 w-10 text-sm font-bold'
          : 'gap-2 px-3 py-2 text-left text-sm'} {globalState.activeSelectedBucketId === bucket.id
          ? 'bg-cyan-100 text-cyan-900 dark:bg-cyan-900/30 dark:text-cyan-200'
          : 'text-slate-700 hover:bg-slate-100 dark:text-slate-300 dark:hover:bg-slate-700/50'}"
        title={bucket.bucketName}
      >
        {#if globalState.appSetting.sidebarCollapsed}
          <span>{bucket.bucketName.charAt(0).toUpperCase()}</span>
        {:else}
          <div class="flex-1 truncate font-medium">{bucket.bucketName}</div>
          {#if globalState.activeSelectedBucketId === bucket.id}
            <div class="h-2 w-2 rounded-full bg-cyan-600 dark:bg-cyan-400"></div>
          {/if}
        {/if}
      </button>
    {/each}
  {/if}
</div>

