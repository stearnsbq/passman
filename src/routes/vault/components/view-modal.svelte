<script lang="ts">
    import Modal from './modal.svelte';
    import { invoke } from "@tauri-apps/api/tauri";
    import logo from '../../../assets/no-icon.png';

    export let show;
    export let vaultItem;
    let showPassword = false;
    let password = '';
    let img;

    function revealPassword(){
        invoke("get_password", {id: vaultItem.password_id}).then((result: string) => {
            showPassword = true;
            password = result;
        })
    }

    function onImgError(){
        img.src = logo;
    }


    $: if(!show){
        showPassword = false;
        password = null;
    }

</script>



<Modal bind:show={show}>

    <div class="new-password-form"> 

        <div class="header">
            <h1>View</h1>
        </div>

        <div class="body">

            <div class="icon">
                <img bind:this={img}  on:error={onImgError} src={vaultItem.icon} alt="Put Icon Here">
            </div>
         
            <div class="info-input">
                <input readonly placeholder="Source" bind:value={vaultItem.source}>
                <input readonly placeholder="Username" bind:value={vaultItem.username}>


                {#if !showPassword}
                <div class="password-hidden">
                    <button on:click={revealPassword}>Reveal Password</button>
                </div>
                {:else}

                    <input readonly placeholder="Password" value={password}>

                {/if}

                <!-- <PasswordInput bind:password={password} ></PasswordInput> -->
    
            </div>
    
        </div>


    </div>




</Modal>



<style lang="scss">

    .password-hidden{
        display: flex;
        justify-content: center;
    }

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

    img{
            height: 100px;
            width: 100px;
            border-radius: 50%;
        }

</style>