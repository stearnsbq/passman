<script lang="ts">
      export let location;
      import { onMount } from 'svelte';
      import Menu from './components/menu.svelte'
      import Modal from './components/modal.svelte'
      import PasswordInput from './components/password-input.svelte'
      import IconInput from './components/icon-input.svelte';
      import NewPasswordModal from './components/new-password-modal.svelte'


      onMount(() => {
          console.log((location as any).state)
      })

      let showHeaderMenu = false;

      let showPasswordGenModal = false;
      let showPasswordStrengthModal = false;
      let showNewPasswordModal = false;

      function onNewPassword(){
        showNewPasswordModal = true;
      }

      function onMenuClick(){
        showHeaderMenu = !showHeaderMenu;
      }

      function onPasswordGeneratorClick(){
        showPasswordGenModal = true;
        showHeaderMenu = false;
      }

      function onPasswordStrengthClick(){
        showPasswordStrengthModal = true;
        showHeaderMenu = false;
     }

     function onNewPasswordSubmit(form){

     }

</script>


<NewPasswordModal bind:show={showNewPasswordModal}></NewPasswordModal>

<Modal bind:show={showPasswordGenModal}>

</Modal>

<Modal bind:show={showPasswordStrengthModal}>

</Modal>

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
        
    </section>



</div>


<style lang="scss">
    .vault-content{
        height: 100vh;
        width: 100vw;
        display: flex;
        flex-direction: column;
    }

    .header{
        position: relative;
        box-shadow: rgba(50, 50, 93, 0.25) 0px 6px 12px -2px, rgba(0, 0, 0, 0.3) 0px 3px 7px -3px;
        display: flex;
        width: 100%;

        background-color: #007fff;

        color: white;

        .menu{
            display: flex;
            flex-direction: column;
            position: relative;
        }

        span{
            padding: 15px;
            font-size: 24px;  
            cursor: pointer;

            &:hover{
                background-color: rgba(0, 0, 0, 0.3);
              
            }
        }

    }

    .menu-list{
        padding: 0;
        margin: 0;

        li{

            cursor: pointer;
            list-style: none;
            padding: 15px;
            &:hover{
                background-color: rgba(0, 0, 0, 0.3);
            }
        }
    }

    form{
        width: 250px;
    }

</style>