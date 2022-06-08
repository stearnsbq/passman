<script lang="ts">
  import { Link } from "svelte-routing";
  import { invoke } from "@tauri-apps/api/tauri";
  import { onMount } from 'svelte';
  import { navigate } from "svelte-routing";
  // colors main blue: #007FFF
  // secondary orange: #FF9944
  // text color #3B3C36
  // off text color: white

  function onLogin(event: any) {
    const formData = new FormData(event.target);

    const data: any = {};
    for (let field of formData) {
      const [key, value] = field;
      data[key] = value;
    }
    
    const {key} = data;


    invoke("login", {masterKey: key}).then((result) => {


      console.log(result)

    })


  }


  onMount(async () => {

    await invoke("is_vault_setup").then((result) => {

      if(!result){
        navigate("/setup", {replace: true})
      }


    })

  })

</script>

<div class="login-content">
  <div class="login-form">
    <section class="form-header">
      <h1>Please Enter Your Master Key</h1>
    </section>
    <form on:submit|preventDefault={onLogin}>
      <input name="key" type="password" required placeholder="Master Key" />

      <div class="form-controls">
        <button>Unlock</button>
      </div>
    </form>
  </div>
</div>

<style lang="scss">
  .login-content {
    height: 100vh;
    width: 100%;

    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
  }

  .login-form {
    background-color: #36454f;
    color: white;
    box-shadow: rgba(60, 64, 67, 0.3) 0px 1px 2px 0px,
      rgba(60, 64, 67, 0.15) 0px 2px 6px 2px;
    padding: 15px;
    border-radius: 5px;
    display: flex;
    flex-direction: column;
    justify-content: center;

    form {
      display: flex;
      flex-direction: column;
      justify-content: center;
      gap: 15px;

      .form-controls {
        display: flex;
        justify-content: center;
      }
    }
  }
</style>
