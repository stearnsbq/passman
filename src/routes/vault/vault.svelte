<script lang="ts">
      export let location;
      import { onMount } from 'svelte';

      onMount(() => {
          console.log((location as any).state)
      })

</script>



<div class="vault-content">
    
</div>


<style lang="scss">
    .vault-content{
        height: 100vh;
        width: 100%;

        display: flex;
        flex-direction: column;
        justify-content: center;
        align-items: center;
    }
</style>