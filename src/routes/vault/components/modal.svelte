<script>
  export let show = false;

  function handleKeydown(event) {
    if (event.key === "Escape") {
      show = false;
    }
  }
</script>

<svelte:window on:keydown={handleKeydown} />

{#if show}
  <div class="modal">
    <div class="modal-content">
      <div class="modal-close">
        <span on:click={e => show = false}>Close</span>
      </div>
      <slot />
    </div>
  </div>
{/if}

<style lang="scss">
  .modal {
    z-index: 100;
    position: absolute;
    top: 0;
    left: 0;
    height: 100vh;
    width: 100vw;
    display: flex;
    justify-content: center;
    align-items: center;
    flex-direction: column;

    background-color: rgba($color: #000000, $alpha: 0.4);
    backdrop-filter: blur(5px);
  }

  .modal-content {
    background-color: #3b3c36;
    padding: 15px;

    .modal-close {
      display: flex;
      justify-content: flex-end;
      span {
        cursor: pointer;
        color: white;
      }
    }
  }
</style>
