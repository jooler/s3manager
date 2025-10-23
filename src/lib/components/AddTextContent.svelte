<script lang="ts">
  import { t } from "$lib/i18n.svelte";
  import { closeModal, globalState } from "$lib/store.svelte";
  import { generateTimestampTextFileName } from "$lib/tools";

  let textContent = $state("");
  let remoteFilename = $state(generateTimestampTextFileName());

  function confirm() {
    globalState.files.push({
      type: "text",
      id: Date.now().toString(),
      source: { fileContent: textContent },
      remoteFilename,
      remoteFilenamePrefix: "",
    });
    closeModal();
  }
</script>

<div class="space-y-2">
  <div>
    <p
      class="mb-1 block text-sm font-medium text-slate-700 dark:text-slate-300"
    >
      {t().textUploader.textSeparator}
    </p>
    <textarea
      bind:value={textContent}
      class="w-full resize-none rounded-lg bg-slate-50 p-3 focus:outline-none dark:bg-slate-700"
      placeholder={t().textUploader.textPlaceholder}
      rows="6"
    ></textarea>
  </div>
  <div>
    <p
      class="mb-1 block text-sm font-medium text-slate-700 dark:text-slate-300"
    >
      {t().textUploader.filenameSeparator}
    </p>
    <input
      bind:value={remoteFilename}
      class="w-full rounded-lg bg-slate-50 p-3 focus:outline-none dark:bg-slate-700"
      placeholder={t().textUploader.filenamePlaceholder}
    />
  </div>
  <div class="mt-4 flex justify-end space-x-2">
    <button onclick={closeModal} class="button button-primary"
      >{t().addBucket.cancel}</button
    >
    <button onclick={confirm} class="button button-primary"
      >{t().addBucket.save}</button
    >
  </div>
</div>
