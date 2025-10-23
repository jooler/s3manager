<script lang="ts">
  import { t } from "$lib/i18n.svelte";
  import { Select } from "bits-ui";
  import { ChevronsUpDown } from "lucide-svelte";

  let {
    onTypeSelect,
  }: {
    onTypeSelect: (type: "r2" | "oss") => void;
  } = $props();

  const bucketTypes = [
    {
      value: "r2",
      label: "Cloudflare R2",
      description: "Global cloud storage with free downloads",
    },
    {
      value: "oss",
      label: "Aliyun OSS",
      description: "Alibaba Cloud Object Storage Service",
    },
  ];

  function handleSelect(type: string | undefined) {
    if (type === "r2" || type === "oss") {
      onTypeSelect(type);
    }
  }
</script>

<div class="space-y-4">
  <div class="flex items-center justify-between">
    <p class="text-lg font-semibold text-slate-800 dark:text-slate-200">
      {t().addBucket.selectBucketType}
    </p>
  </div>

  <div class="space-y-3">
    {#each bucketTypes as type}
      <button
        onclick={() => handleSelect(type.value)}
        class="w-full rounded-lg border-2 border-slate-200 p-4 text-left transition-all hover:border-cyan-500 hover:bg-cyan-50 dark:border-slate-700 dark:hover:bg-slate-800/50"
      >
        <div class="font-semibold text-slate-800 dark:text-slate-200">
          {type.label}
        </div>
        <div class="text-sm text-slate-500 dark:text-slate-400">
          {type.description}
        </div>
      </button>
    {/each}
  </div>
</div>

<style lang="postcss">
  button:hover {
    @apply border-cyan-500 bg-cyan-50 dark:bg-slate-800/50;
  }
</style>

