<script lang="ts">
  import validator from "password-validator";
  import { invoke } from "@tauri-apps/api/tauri";
  import { navigate, Router } from "svelte-routing";
  import { getFormData } from "../../lib/helpers";

  let key = "";
  let invalid = true;

  let createdKey = false;

  const passwordValidator = new validator()
    .is().min(8)
    .has().uppercase()
    .has().lowercase(1)
    .has().digits(1)
    .has().symbols(1)
    .has().not().spaces();

  function onSubmit(event) {
    if(!invalid){

      const data = getFormData(event.target);

      key = data.key;

      setupVault()

    }
  }

  function onInput(event) {
    invalid = !passwordValidator.validate(key);
  }

  function setupVault(){
    invoke("setup_vault", {masterKey: key}).then((result: string) => {
      navigate("/");
    });
  }

</script>

<div class="setup-content">
  <div class="setup-form">
    {#if !createdKey}
      <section class="form-header">
        <h1>Setup</h1>
      </section>

      <section class="setup-instructions">
        <p>Master key must follow these guidelines</p>
        <ul>
          <li>Must be atleast 8 characters long</li>
          <li>Must contain at least one uppercase letter</li>
          <li>Must contain at least one number</li>
          <li>Must contain at least one special character</li>
        </ul>
      </section>

      <form on:submit|preventDefault={onSubmit}>
        <input
          name="key"
          type="password"
          placeholder="Enter Master Key"
          on:input={onInput}
          bind:value={key}
        />
        <button disabled={invalid}>Create</button>
      </form>
    {/if}

  </div>
</div>

<style lang="scss">

.body-controls{
  display: flex;
  flex-direction: column;
  gap: 15px;
}

  .passphrase-body {
    display: flex;
    flex-direction: column;
    justify-content: center;

    p {
      text-align: center;
    }
  }

  h1 {
    margin: 0;
  }

  .setup-content {
    height: 100vh;
    width: 100%;

    display: flex;
    flex-direction: column;
    justify-content: center;
    align-items: center;
  }

  .setup-form {
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
