<script lang="ts">
  import { globalState } from "$lib/store.svelte";
  import { onMount } from "svelte";

  let modal: HTMLDialogElement;
  let isLoading = $state(true);

  onMount(() => {
    requestAnimationFrame(() => {
      isLoading = false;
    });
  });

  function onclose() {
    globalState.modal.isShow = false;
    if (globalState.modal.onClose) {
      globalState.modal.onClose();
      globalState.modal.onClose = undefined;
    }
    globalState.modal.children = undefined;
  }

  $effect(() => {
    if (globalState.modal.isShow) {
      modal.showModal();
    } else {
      modal.close();
    }
  });
</script>

<dialog
  id="modal"
  class="modal modal-bottom sm:modal-middle"
  bind:this={modal}
  {onclose}
>
  <div class="modal-box scrollbar-hide dark:bg-slate-800">
    {#if isLoading}
      <div class="flex h-full items-center justify-center p-2">loading...</div>
    {:else}
      {@render globalState.modal.children?.()}
    {/if}
  </div>
  <form method="dialog" class="modal-backdrop">
    <button>close</button>
  </form>
</dialog>
