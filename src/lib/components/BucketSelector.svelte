<script lang="ts">
  import db from "$lib/db";
  import { t } from "$lib/i18n.svelte";
  import { globalState } from "$lib/store.svelte";
  import type { Bucket } from "$lib/type";
  import { Select, type Selected } from "bits-ui";
  import { ChevronsUpDown } from "lucide-svelte";
  import { onMount } from "svelte";

  let buckets: Selected<Bucket>[] = $state([]);

  onMount(async () => {
    getBuckets();
  });

  async function getBuckets() {
    await db.buckets.toArray().then((targets) => {
      buckets = targets.map((target) => ({
        value: target,
        label: target.bucketName,
      }));
    });
    if (buckets.length > 0) {
      const defaultBucket = buckets.find(
        (bucket) => bucket.value.id === globalState.appSetting.defaultBucketId,
      );
      globalState.selectedBucket = defaultBucket || buckets[0];
    } else {
      globalState.selectedBucket = undefined;
    }
  }
</script>

{#if buckets.length === 0}
  <div
    class="rounded-lg border border-yellow-300 bg-yellow-50 p-4 text-yellow-800 dark:border-yellow-800 dark:bg-yellow-900/20 dark:text-yellow-200"
  >
    {t().common.noBucketWarning}
  </div>
{:else}
  {@render selector()}
{/if}

{#snippet selector()}
  <div
    class="flex items-center justify-between rounded-lg border border-slate-200 bg-slate-100/80 p-2 text-slate-400 dark:border-slate-700 dark:bg-slate-800"
  >
    <p class="font-bold text-slate-700 dark:text-slate-300">
      {t().uploadTargetSelector.title}
    </p>
    <Select.Root
      items={buckets}
      selected={globalState.selectedBucket}
      onSelectedChange={(e) => {
        if (e) {
          globalState.selectedBucket = {
            value: e.value,
            label: e.value.bucketName,
          };
        }
      }}
    >
      <Select.Trigger class="select-trigger">
        <Select.Value placeholder={t().uploadTargetSelector.placeholder} />
        <ChevronsUpDown
          class="dark-text-slate-300 ml-auto size-4 text-slate-400"
        />
      </Select.Trigger>
      <Select.Content class="select-content">
        {#each buckets as target}
          <Select.Item
            value={target.value}
            label={target.label}
            class="select-item"
          >
            {target.label}
          </Select.Item>
        {/each}
      </Select.Content>
    </Select.Root>
  </div>
{/snippet}
