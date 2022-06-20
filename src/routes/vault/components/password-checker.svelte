<script lang="ts">
    import Modal from './modal.svelte'
    import PasswordInput from './password-input.svelte';
    export let show = false;

</script>



<Modal bind:show={show}>
    <div class="strength-checker">
        <div class="checker-header">
          <h2>Strength Checker</h2>
        </div>
    
        <PasswordInput showFeedback={true} />
    </div>
</Modal>



<style lang="scss">
  .checker-header {
    display: flex;
    justify-content: center;
    h2 {
      color: white;
      padding: 0;
      margin: 0;
    }
  }

  .strength-checker{
    display: flex;
    width: 350px;
    flex-direction: column;
    gap: 15px;
  }
</style>