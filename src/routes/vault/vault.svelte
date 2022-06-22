<script lang="ts">
  export let location;
  import { onMount } from "svelte";
  import { invoke } from "@tauri-apps/api/tauri";
  import Menu from "./components/menu.svelte";
  import NewPasswordModal from "./components/new-password-modal.svelte";
  import { listen, idle, onIdle } from "svelte-idle";
  import logo from "../../assets/no-icon.png";
  import ViewModal from "./components/view-modal.svelte";
  import PasswordCheckerModal from './components/password-checker.svelte'
  import GeneratePasswordModal from './components/generate-password.svelte'
import { navigate, Router } from "svelte-routing";

  // start idle listener
  listen({
    timer: 300000,
  });

  let idleTimeout;

  let vault;

  onMount(() => {
    vault = (location as any).state;
  });

  let showHeaderMenu = false;

  let showPasswordGenModal = false;
  let showPasswordStrengthModal = false;
  let showNewPasswordModal = false;
  let showViewModal = false;
  let currentVaultItem;

  function onNewPassword() {
    showNewPasswordModal = true;
  }

  function onMenuClick() {
    showHeaderMenu = !showHeaderMenu;
  }

  function onPasswordGeneratorClick() {
    showPasswordGenModal = true;
    showHeaderMenu = false;
  }

  function onPasswordStrengthClick() {
    showPasswordStrengthModal = true;
    showHeaderMenu = false;
  }

  function onImageError(e) {
    (e as any).target.src = logo;
  }

  function handleMessage(event) {
    console.log(event);
  }

  $: {
    if($idle){
        alert("Are you still there? You will be logged out in 30 seconds if no activity is detected!")
        idleTimeout = setTimeout(() => {
         
          invoke("logout").then(() => {
            alert("you've been logged out");
            vault = null;
            navigate("/");
            
          })

        }, 30000);
    }else{
        console.log("not idle")
        clearTimeout(idleTimeout)
    }
  }
</script>

<ViewModal bind:show={showViewModal} vaultItem={currentVaultItem} />

<NewPasswordModal on:message={handleMessage} bind:show={showNewPasswordModal} />

<GeneratePasswordModal bind:show={showPasswordGenModal} />

<PasswordCheckerModal bind:show={showPasswordStrengthModal}  />


<div class="vault-content">
  <header class="header">
    <span on:click={onNewPassword}>Add New</span>

    <div class="menu">
      <span on:click={onMenuClick}>Tools</span>
      <Menu bind:showMenu={showHeaderMenu}>
        <ul class="menu-list">
          <li on:click={onPasswordGeneratorClick}>Password Generator</li>
          <li on:click={onPasswordStrengthClick}>Password Strength Checker</li>
        </ul>
      </Menu>
    </div>
  </header>

  <section class="password-list">
    {#if vault}
      <ul>
        {#each vault.passwords as password}
          <li
            on:click={(e) => {
              currentVaultItem = password;
              showViewModal = true;
            }}
          >
            <div class="password-list-item">
              <div class="list-item-icon">
                <img
                  src={password.icon}
                  on:error={onImageError}
                  alt="image for {password.password_id}"
                />
              </div>

              <div class="list-item-info">
                <h3>{password.source}</h3>
                <p>{password.username}</p>
              </div>
            </div>
          </li>
        {/each}
      </ul>
    {/if}
  </section>
</div>

<style lang="scss">
  .vault-content {
    height: 100vh;
    width: 100vw;
    display: flex;
    flex-direction: column;
  }

  .header {
    position: fixed;
    box-shadow: rgba(50, 50, 93, 0.25) 0px 6px 12px -2px,
      rgba(0, 0, 0, 0.3) 0px 3px 7px -3px;
    display: flex;
    width: 100%;

    background-color: #007fff;

    color: white;

    .menu {
      display: flex;
      flex-direction: column;
      position: relative;
    }

    span {
      padding: 15px;
      font-size: 24px;
      cursor: pointer;

      &:hover {
        background-color: rgba(0, 0, 0, 0.3);
      }
    }
  }

  .menu-list {
    padding: 0;
    margin: 0;

    li {
      cursor: pointer;
      list-style: none;
      padding: 15px;
      &:hover {
        background-color: rgba(0, 0, 0, 0.3);
      }
    }
  }

  form {
    width: 250px;
  }

  .password-list {
    margin-top: 60px;
    display: flex;
    height: 100%;
    overflow: auto;
    justify-content: center;

    .list-item-info {
      color: white;
      display: flex;
      justify-content: center;
      align-items: center;
      flex-direction: column;
      gap: 10px;

      h3 {
        margin: 0;
        padding: 0;
      }

      p {
        margin: 0;
        color: rgba($color: white, $alpha: 0.6);
      }
    }

    .password-list-item {
      box-shadow: rgba(0, 0, 0, 0.19) 0px 10px 20px,
        rgba(0, 0, 0, 0.23) 0px 6px 6px;
      border-radius: 15px;
      width: 150px;
      display: flex;
      flex-direction: column;
      gap: 15px;
      background: #3b3c36;
      padding: 15px;
    }

    ul {
      width: 100%;
      margin: 0;
      padding: 0;
      display: flex;
      flex-wrap: wrap;
      justify-content: center;
    }

    li {
      cursor: pointer;
      list-style: none;
      padding: 15px;

      img {
        width: 100%;
        height: 100px;
        object-fit: contain;
      }
    }
  }



</style>
