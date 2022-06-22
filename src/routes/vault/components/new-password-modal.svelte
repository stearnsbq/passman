<script lang="ts">
    import Modal from './modal.svelte'
    import IconInput from './icon-input.svelte'
    import PasswordInput from './password-input.svelte'
    import { createEventDispatcher } from 'svelte';
    import { invoke } from "@tauri-apps/api/tauri";

    export let show = false;

    let image: string;
    let password: string;
    let source: string;
    let username: string;

    const dispatch = createEventDispatcher();

    function onCreate() {


  


        invoke("add_new_password", {source, username, password, image}).then((result) => {
            dispatch('new-password');
            show = false;
        })

    }

</script>



<Modal bind:show={show}>

    <div class="new-password-form"> 

        <div class="header">
            <h1>Add a new password</h1>
        </div>

        <div class="body">

            <div class="icon">
                <IconInput bind:image={image}></IconInput>
            </div>
         
            <div class="info-input">
                <input placeholder="Source" bind:value={source}>
                <input placeholder="Username" bind:value={username}>

                <PasswordInput bind:password={password} ></PasswordInput>
    
                <button on:click={onCreate}>Create</button>
            </div>
    
        </div>


    </div>
        

</Modal>



<style lang="scss">

    .new-password-form{
        display: flex;
        width: 350px;
        flex-direction: column;
        gap: 15px;

        .header{
            display: flex;
            justify-content: center;
            h1{
                color: white;
                padding: 0;
                margin: 0;
            }
        }
        
        .body{
            display: flex;
            flex-direction: row;
            gap: 15px;
        }

        .info-input{
            width: 100%;
            display: flex;
            flex-direction: column;
            gap: 15px;
        }

    }

</style>