<script lang="ts">
    import Modal from './modal.svelte'
    export let show = false;
    import { invoke } from "@tauri-apps/api/tauri";
    let password = '';

    function generatePassword(){
        invoke("generate_password").then((result: string) => {
            password = result;
        })
    }


</script>



<Modal bind:show={show}>

    <div class="generator-content">
      <div class="generator-header">
        <h2>Generate Password</h2>
      </div>
  
      <div class="generated-password">
        <h4>{password}</h4>
      </div>


      <button on:click={generatePassword}>Generate</button>

    </div>
  
</Modal>


<style lang="scss">

.generator-header {
    display: flex;
    justify-content: center;
    h2 {
      color: white;
      padding: 0;
      margin: 0;
    }
  }

  .generator-content{
    display: flex;
    width: 350px;
    flex-direction: column;
    gap: 15px;
  }

  .generated-password{
    display: flex;
    justify-content: center;
    color: white;
  }

</style>