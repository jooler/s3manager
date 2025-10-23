<script lang="ts">
  import { globalState } from "$lib/store.svelte";
  import { quadOut } from "svelte/easing";
  import { fly } from "svelte/transition";

  $effect(() => {
    if (globalState.alertMessage) {
      const timer = setTimeout(() => {
        globalState.alertMessage = "";
      }, 2000);

      return () => clearTimeout(timer);
    }
  });
</script>

{#if globalState.alertMessage}
  <div
    class="top-safe fixed right-0 left-0 z-[9999] mx-auto mt-4 max-w-[90vw] px-4 sm:max-w-sm"
    transition:fly={{ y: -50, duration: 300, easing: quadOut }}
  >
    <div
      role="alert"
      class="alert bg-primary flex items-center justify-center rounded-2xl
                   border-none px-6 py-3 text-center text-white shadow-lg
                   ring-1 ring-white/10 backdrop-blur"
    >
      <p class="truncate text-sm font-medium">{globalState.alertMessage}</p>
    </div>
  </div>
{/if}
