<script lang="ts">
  import db from "$lib/db";
  import { t } from "$lib/i18n.svelte";
  import {
    closeModal,
    globalState,
    refreshBuckets,
    setAlert,
    showModal,
  } from "$lib/store.svelte";
  import type { Bucket } from "$lib/type";
  import { invoke } from "@tauri-apps/api/core";
  import { ArrowLeft, HelpCircle } from "lucide-svelte";
  import { onDestroy } from "svelte";
  import { marked } from "marked";
  import DOMPurify from "dompurify";
  import BucketTypeSelector from "./BucketTypeSelector.svelte";

  let showHelp = $state(false);
  let showTypeSelector = $state(false);

  let {
    onclose,
    show = $bindable(false),
    editBucketId = $bindable<number | undefined>(undefined),
  }: {
    onclose?: () => void;
    show: boolean;
    editBucketId?: number;
  } = $props();

  let checkResult = $state(false);
  let isChecking = $state(false);
  let errorMessage = $state("");

  let bucket: Bucket = $state({
    type: "r2",
    bucketName: "",
    accountId: "",
    accessKey: "",
    secretKey: "",
    customDomain: "",
    s3Api: "",
    endpoint: "",
  });

  $effect(() => {
    if (show) {
      // 如果是编辑模式，直接显示表单；否则显示类型选择器
      if (editBucketId) {
        db.buckets.get(editBucketId).then((b) => {
          if (b) {
            bucket = b;
            showTypeSelector = false;
            showModal(content);
            globalState.modal.onClose = onClose;
          }
        });
      } else {
        showTypeSelector = true;
        showModal(content);
        globalState.modal.onClose = onClose;
      }
    }
  });

  function resetState() {
    checkResult = false;
    isChecking = false;
    errorMessage = "";
  }

  function handleBucketTypeSelect(type: "r2" | "oss") {
    bucket.type = type;
    showTypeSelector = false;
    resetState();
    updateInputConfigs();
  }

  function updateInputConfigs() {
    if (bucket.type === "r2") {
      inputConfigs.length = 0;
      inputConfigs.push(
        {
          id: "s3Api",
          label: t().addBucket.labels.s3Api,
          focused: false,
          required: false,
          error: false,
        },
        {
          id: "bucketName",
          label: t().addBucket.labels.bucketName,
          focused: false,
          required: true,
        },
        {
          id: "accountId",
          label: t().addBucket.labels.accountId,
          focused: false,
          required: true,
        },
        {
          id: "accessKey",
          label: t().addBucket.labels.accessKey,
          focused: false,
          required: true,
        },
        {
          id: "secretKey",
          label: t().addBucket.labels.secretKey,
          focused: false,
          required: true,
        },
        {
          id: "customDomain",
          label: t().addBucket.labels.customDomain,
          focused: false,
          required: false,
        }
      );
    } else if (bucket.type === "oss") {
      inputConfigs.length = 0;
      inputConfigs.push(
        {
          id: "bucketName",
          label: t().addBucket.labels.bucketName,
          focused: false,
          required: true,
        },
        {
          id: "accessKey",
          label: t().addBucket.labels.accessKey,
          focused: false,
          required: true,
        },
        {
          id: "secretKey",
          label: t().addBucket.labels.secretKey,
          focused: false,
          required: true,
        },
        {
          id: "endpoint",
          label: t().addBucket.labels.endpoint,
          focused: false,
          required: true,
          placeholder: "https://oss-cn-hangzhou.aliyuncs.com",
        },
        {
          id: "customDomain",
          label: t().addBucket.labels.customDomain,
          focused: false,
          required: false,
        }
      );
    }
  }

  async function parseS3ApiUrl(url: string) {
    if (!url) return;
    try {
      const urlObj = new URL(url);
      const regex =
        /^https:\/\/([a-zA-Z0-9]+)\.r2\.cloudflarestorage\.com\/([a-zA-Z0-9-]+)\/?$/;
      if (!regex.test(url)) {
        const s3ApiInput = inputConfigs.find((c) => c.id === "s3Api");
        if (s3ApiInput) {
          s3ApiInput.label = t().addBucket.invalidS3Api;
          s3ApiInput.error = true;
        }
        return;
      }
      const accountId = urlObj.hostname.split(".")[0];
      const bucketName = urlObj.pathname.split("/")[1];
      bucket.accountId = accountId;
      bucket.bucketName = bucketName;
    } catch (e) {
      const s3ApiInput = inputConfigs.find((c) => c.id === "s3Api");
      if (s3ApiInput) {
        s3ApiInput.label = t().addBucket.invalidS3Api;
        s3ApiInput.error = true;
      }
      console.error("Invalid S3 API URL");
    }
  }

  let inputConfigs: Array<{
    id: string;
    label: string;
    focused: boolean;
    required: boolean;
    error?: boolean;
    placeholder?: string;
  }> = $state([]);

  async function saveBucket() {
    await db.buckets.put({
      ...bucket,
    });

    refreshBuckets();
    closeModal();
  }

  async function checkButket() {
    isChecking = true;
    errorMessage = "";
    try {
      await invoke("r2_ping", bucket);
      checkResult = true;
      setAlert("success");
    } catch (e) {
      checkResult = false;
      errorMessage = e as string;
      console.error(e);
    } finally {
      isChecking = false;
    }
  }

  function onClose() {
    if (onclose) {
      onclose();
    }
    bucket = {
      type: "r2",
      bucketName: "",
      accountId: "",
      accessKey: "",
      secretKey: "",
      customDomain: "",
      s3Api: "",
      endpoint: "",
    };
    show = false;
    editBucketId = undefined;
    showTypeSelector = false;
    showHelp = false;
    inputConfigs.length = 0;
  }

  function renderMarkdown(markdown: string) {
    marked.setOptions({
      async: false,
      gfm: false,
    });
    const html = marked(markdown);
    return DOMPurify.sanitize(html as string);
  }
</script>

{#snippet content()}
  {#if showTypeSelector}
    <BucketTypeSelector onTypeSelect={handleBucketTypeSelect} />
  {:else if showHelp}
    <div class="space-y-4">
      <div class="flex items-center gap-2">
        <button class="button" onclick={() => (showHelp = false)}>
          <ArrowLeft size={20} />
        </button>
        <p>{t().addBucket.howToUse}</p>
      </div>
      <div class="prose prose-sm prose-slate dark:prose-invert max-w-none">
        {@html renderMarkdown(t().addBucket.use)}
      </div>
    </div>
  {:else}
    <div class="space-y-6">
      <div class="flex items-center justify-between">
        <p>
          {bucket.type === "r2"
            ? t().addBucket.titleR2
            : t().addBucket.titleOSS}
        </p>
        <button class="button" onclick={() => (showHelp = true)}>
          <HelpCircle size={20} />
        </button>
      </div>

      {#each inputConfigs as config}
        <div class="relative">
          <input
            bind:value={bucket[config.id]}
            type="text"
            id={config.id}
            placeholder={config.placeholder || ""}
            class="input-field"
            onfocus={() => (config.focused = true)}
            onblur={() => (config.focused = false)}
            oninput={(e: Event) => {
              resetState();
              if (config.id === "s3Api") {
                const target = e.target as HTMLInputElement;
                if (config.error) {
                  config.label = t().addBucket.labels.s3Api;
                  config.error = false;
                }
                parseS3ApiUrl(target.value);
              }
            }}
          />
          <label
            for={config.id}
            class="input-label"
            class:input-label-active={config.focused || bucket[config.id]}
            class:error={config.error}
          >
            {config.label}{config.required ? "*" : ""}
          </label>
        </div>
      {/each}
    </div>
    <div class="mt-2">
      {#if errorMessage}
        <p class="text-sm text-rose-500">{errorMessage}</p>
      {/if}
      <div class="flex justify-end space-x-2">
        <button onclick={closeModal} class="button button-primary"
          >{t().addBucket.cancel}</button
        >
        {#if !checkResult}
          <button
            onclick={checkButket}
            class="button button-primary"
            disabled={isChecking}
          >
            {#if isChecking}
              {t().addBucket.checking}
            {:else}
              {t().addBucket.check}
            {/if}
          </button>
        {:else}
          <button onclick={saveBucket} class="button text-green-500"
            >{t().addBucket.save}</button
          >
        {/if}
      </div>
    </div>
  {/if}
{/snippet}

<style lang="postcss">
  .input-field {
    @apply w-full border-0 border-b border-slate-300 py-1 transition-colors outline-none dark:border-slate-500;
  }

  .input-field:focus {
    @apply border-cyan-500;
  }

  .input-label {
    @apply pointer-events-none absolute top-2 left-0 opacity-30 transition-all;
  }

  .input-label-active {
    @apply -translate-y-5 text-sm opacity-100;
  }

  .input-label-active:not(.error) {
    @apply text-cyan-500;
  }

  .input-label-active.error {
    @apply text-rose-500;
  }
</style>
