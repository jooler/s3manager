<script lang="ts">
  import { showModal } from "$lib/store.svelte";
  import {
    checkClipboardContent,
    openDialogForDir,
    openDialogForFiles,
  } from "$lib/tools";
  import { UploadCloud } from "lucide-svelte";
  import AddTextContent from "./AddTextContent.svelte";
  import { t } from "$lib/i18n.svelte";
  import { onMount } from "svelte";
  import { platform } from "@tauri-apps/plugin-os";

  let isMobile = $state(false);

  onMount(() => {
    const currentPlatform = platform();
    if (currentPlatform === "ios" || currentPlatform === "android") {
      isMobile = true;
    }
  });
</script>

<div class="flex items-center justify-center gap-12">
  <UploadCloud
    class="hidden size-32 text-cyan-500 sm:block dark:text-slate-400"
  />
  <div class="flex flex-1 flex-col items-start gap-3">
    <p class="text-slate-500 dark:text-slate-300">
      {t().fileUploader.bucketReadyTip}
    </p>
    <div class="grid grid-cols-2 gap-2">
      <button onclick={openDialogForFiles} class="button button-primary"
        >{t().fileUploader.selectFile}</button
      >
      {#if !isMobile}
        <button onclick={openDialogForDir} class="button button-primary"
          >{t().fileUploader.selectFolder}</button
        >
        <button onclick={checkClipboardContent} class="button button-primary"
          >{t().fileUploader.selectClipboard}</button
        >
      {/if}
      <button onclick={() => showModal(text)} class="button button-primary"
        >{t().fileUploader.selectNewText}</button
      >
    </div>
  </div>
</div>

{#snippet text()}
  <AddTextContent />
{/snippet}
