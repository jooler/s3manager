<script lang="ts">
  import { page } from "$app/state";
  import { t } from "$lib/i18n.svelte";
  import { globalState } from "$lib/store.svelte";
  import {
    ArrowsUpFromLine,
    CloudUpload,
    PanelRightClose,
    PanelRightOpen,
    Settings,
    Crown,
  } from "lucide-svelte";

  const links = $derived([
    { href: "/", icon: CloudUpload, label: t().common.upload },
    { href: "/transfer", icon: ArrowsUpFromLine, label: t().common.transfer },
    { href: "/setting", icon: Settings, label: t().common.setting },
  ]);
</script>

{@render Desktop()}
{@render Mobile()}

{#snippet Desktop()}
  <nav
    class="hidden min-h-dvh border-r border-slate-200 bg-white transition-all md:block dark:border-slate-700 dark:bg-slate-800 {globalState
      .appSetting.sidebarCollapsed
      ? 'w-16'
      : 'w-40'}"
  >
    <ul class="flex flex-col items-center space-y-2">
      <li
        class="flex w-full {globalState.appSetting.sidebarCollapsed
          ? 'justify-center'
          : 'justify-end'}"
      >
        <button
          onclick={() =>
            (globalState.appSetting.sidebarCollapsed =
              !globalState.appSetting.sidebarCollapsed)}
          class="nav-link gapped"
          aria-label={globalState.appSetting.sidebarCollapsed
            ? t().common.expand
            : t().common.collapse}
        >
          {#if globalState.appSetting.sidebarCollapsed}
            <PanelRightClose class="size-5" />
          {:else}
            <PanelRightOpen class="size-5" />
          {/if}
        </button>
      </li>
      {#each links as { href, icon: Icon, label }}
        <li class="w-full px-2">
          <a
            {href}
            class="nav-link gapped bg {globalState.appSetting.sidebarCollapsed
              ? '-mx-2 rounded-none'
              : 'rounded-lg'}"
            class:min-w-28={!globalState.appSetting.sidebarCollapsed}
            aria-current={page.route.id === href ? "page" : null}
          >
            <Icon class="size-5" />
            {#if !globalState.appSetting.sidebarCollapsed}
              <span class="text-nowrap">{label}</span>
            {/if}
          </a>
        </li>
      {/each}
    </ul>
  </nav>
{/snippet}

{#snippet Mobile()}
  <div class="fixed inset-x-0 bottom-0 z-50 md:hidden">
    <nav
      class="flex items-center justify-around border-t border-slate-200 bg-white px-4 py-3 dark:border-slate-700 dark:bg-slate-700"
    >
      {#each links as { href, icon: Icon, label }}
        <a
          {href}
          class="nav-link flex-col gap-1"
          aria-current={page.route.id === href ? "page" : null}
        >
          <Icon class="size-5" />
          <span class="text-nowrap">{label}</span>
        </a>
      {/each}
    </nav>
  </div>
{/snippet}

<style lang="postcss">
  .nav-link {
    @apply flex h-12 cursor-pointer items-center justify-center text-slate-700 transition-colors dark:text-slate-200;
  }

  .gapped {
    @apply gap-3 px-4;
  }

  .nav-link[aria-current] {
    @apply text-cyan-600 dark:text-cyan-400;
  }

  .nav-link[aria-current].bg {
    @apply bg-cyan-50 dark:bg-cyan-900/30;
  }
</style>
