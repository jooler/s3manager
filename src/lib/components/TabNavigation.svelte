<script lang="ts">
  import { page } from "$app/state";
  import { t } from "$lib/i18n.svelte";
  import { Database, CloudUpload, ArrowsUpFromLine } from "lucide-svelte";

  const tabs = $derived([
    { href: "/manage", icon: Database, label: t().manage.title },
    { href: "/", icon: CloudUpload, label: t().common.upload },
    { href: "/transfer", icon: ArrowsUpFromLine, label: t().common.transfer },
  ]);

  function isActive(href: string): boolean {
    if (href === "/") {
      return page.route.id === "/";
    }
    return page.route.id?.startsWith(href) ?? false;
  }
</script>

<div class="flex items-center gap-1 border-b border-slate-200 bg-white px-4 dark:border-slate-700 dark:bg-slate-800">
  {#each tabs as { href, icon: Icon, label }}
    <a
      {href}
      class="flex items-center gap-2 border-b-2 px-4 py-3 text-sm font-medium transition-colors {isActive(href)
        ? 'border-cyan-600 text-cyan-600 dark:border-cyan-400 dark:text-cyan-400'
        : 'border-transparent text-slate-600 hover:text-slate-900 dark:text-slate-400 dark:hover:text-slate-200'}"
    >
      <Icon size={18} />
      <span>{label}</span>
    </a>
  {/each}
</div>

<style lang="postcss">
  a {
    @apply transition-all duration-200;
  }

  a:hover {
    @apply bg-slate-50 dark:bg-slate-700/50;
  }
</style>

