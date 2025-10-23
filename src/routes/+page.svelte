<script lang="ts">
  import BucketSelector from "$lib/components/BucketSelector.svelte";
  import FileUploaderRoot from "$lib/components/FileUploaderRoot.svelte";
</script>

<div class="mx-auto flex h-full flex-col gap-2 p-2">
  <BucketSelector />

  <FileUploaderRoot />
</div>
