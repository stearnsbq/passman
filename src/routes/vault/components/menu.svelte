<script lang="ts">
    export let showMenu = false;
</script>


{#if showMenu}
<div class="menu-content">
    <slot />
</div>
{/if}



<style lang="scss">

    .menu-content{
        position: absolute;
        top: 50px;
        width: 250px;
        left: 0;
        background-color: #3B3C36;
     }

</style>