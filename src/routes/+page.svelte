<script lang="ts">
  import BucketSelector from "$lib/components/BucketSelector.svelte";
  import FileUploaderRoot from "$lib/components/FileUploaderRoot.svelte";
  import { t } from "$lib/i18n.svelte";
</script>

<div class="mx-auto flex h-full flex-col gap-2 p-2">
  <h1 class="text-2xl font-bold text-slate-800 dark:text-slate-200">
    {t().common.upload}
  </h1>

  <BucketSelector />

  <FileUploaderRoot />
</div>
