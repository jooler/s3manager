<script lang="ts">
  import FileUploaderProgress from "$lib/components/FileUploaderProgress.svelte";
  import { t } from "$lib/i18n.svelte";
</script>

<div class="mx-auto flex h-full flex-col gap-2 p-2">
  <h1 class="text-2xl font-bold text-slate-800 dark:text-slate-200">
    {t().transfer.title}
  </h1>

  <FileUploaderProgress />
</div>
