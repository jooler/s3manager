<script lang="ts">
  import AddBucket from "$lib/components/AddBucket.svelte";
  import db from "$lib/db";
  import { t } from "$lib/i18n.svelte";
  import { globalState } from "$lib/store.svelte";
  import type { Bucket } from "$lib/type";
  import { Select } from "bits-ui";
  import { ChevronsUpDown } from "lucide-svelte";
  import { onMount } from "svelte";

  const languages = [
    { value: "en", label: "English" },
    { value: "zh", label: "中文" },
  ];

  // 上传目标管理相关状态
  let buckets: Array<Bucket> = $state([]);
  let addBucketModalShow = $state(false);
  let editBucketId: number | undefined = $state();

  onMount(async () => {
    buckets = await db.buckets.toArray();
  });

  async function setDefaultBucket(id: number) {
    globalState.appSetting.defaultBucketId = id;
  }

  async function deleteBucket(id: number) {
    await db.buckets.delete(id);
    buckets = await db.buckets.toArray();
    checkDefaultBucket();
  }

  async function onAddBucketClose() {
    buckets = await db.buckets.toArray();
    // 如果只有一个存储桶且没有默认存储桶，自动设置为默认
    if (buckets.length === 1 && !globalState.appSetting.defaultBucketId) {
      await setDefaultBucket(buckets[0].id!);
    }
  }

  function checkDefaultBucket() {
    // 如果 buckets 中已经没有 defaultBucketId 对应的存储桶，检查 buckets 的数量，如果大于 0，使用第一个存储桶作为默认存储桶
    if (globalState.appSetting.defaultBucketId) {
      const defaultBucket = buckets.find(
        (bucket) => bucket.id === globalState.appSetting.defaultBucketId,
      );
      if (!defaultBucket) {
        // 如果 defaultBucketId 对应的存储桶被删除，清空 defaultBucketId
        globalState.appSetting.defaultBucketId = undefined;
      }
    }
  }
</script>

<AddBucket
  onclose={onAddBucketClose}
  bind:show={addBucketModalShow}
  bind:editBucketId
/>

<div class="mx-auto flex h-full flex-col gap-2 p-2">
  <h1 class="text-2xl font-bold text-slate-800 dark:text-slate-200">
    {t().common.setting}
  </h1>

  <div class="settings-section space-y-2 p-2">
    <div class="flex items-center justify-between">
      <span class="text-slate-600 dark:text-slate-400"
        >{t().settings.language}</span
      >
      <Select.Root
        items={languages}
        onSelectedChange={(selected) => {
          if (selected) {
            globalState.appSetting.locale = selected.value;
          }
        }}
      >
        <Select.Trigger class="select-trigger">
          <Select.Value placeholder={t().common.selectLanguage} />
          <ChevronsUpDown
            class="dark-text-slate-300 ml-auto size-4 text-slate-400"
          />
        </Select.Trigger>
        <Select.Content class="select-content">
          {#each languages as lang}
            <Select.Item
              value={lang.value}
              label={lang.label}
              class="select-item"
            >
              {lang.label}
            </Select.Item>
          {/each}
        </Select.Content>
      </Select.Root>
    </div>
  </div>

  <div class="settings-section flex min-h-0 flex-col overflow-hidden">
    <div class="flex items-center justify-between px-2 pt-2">
      <h2 class="font-bold text-slate-700 dark:text-slate-300">
        {t().settings.buckets}
      </h2>
      <button
        class="button button-primary"
        onclick={() => (addBucketModalShow = true)}
        >{t().addBucket.addNew}</button
      >
    </div>
    <div class="min-h-0 overflow-y-auto px-2 pb-2">
      {#each buckets as bucket}
        <div
          class="flex items-center justify-between border-b py-1 last:border-b-0 dark:border-slate-700"
        >
          <div class="flex-1">
            <div class="target-details">
              <p>{t().settings.bucketDetails.bucket}: {bucket.bucketName}</p>
              <p>{t().settings.bucketDetails.accountId}: {bucket.accountId}</p>
            </div>
          </div>
          {#if globalState.appSetting.defaultBucketId === bucket.id}
            <span class="target-details">
              {t().settings.defaultBucket}
            </span>
          {:else}
            <button
              class="button button-primary button-opacity text-sm"
              onclick={() => setDefaultBucket(bucket.id!)}
            >
              {t().settings.setDefault}
            </button>
          {/if}
          <button
            class="button button-primary button-opacity text-sm"
            onclick={() => {
              editBucketId = bucket.id;
              addBucketModalShow = true;
            }}
          >
            {t().settings.edit}
          </button>
          <button
            class="button button-danger button-opacity text-sm"
            onclick={() => deleteBucket(bucket.id!)}
          >
            {t().common.delete}
          </button>
        </div>
      {/each}
    </div>
  </div>
</div>

<style lang="postcss">
  .settings-section {
    @apply rounded-lg border border-slate-200 bg-slate-100/80 text-slate-400 dark:border-slate-700 dark:bg-slate-800;
  }

  .target-details {
    @apply text-xs text-slate-500 dark:text-slate-400;
  }

  .button-opacity {
    @apply opacity-90 hover:opacity-100;
  }
</style>
