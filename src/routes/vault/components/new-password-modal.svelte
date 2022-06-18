<script lang="ts">
    import Modal from './modal.svelte'
    import IconInput from './icon-input.svelte'
    import PasswordInput from './password-input.svelte'
    import { createEventDispatcher } from 'svelte';


    export let show = false;

    export let image: string;
    export let password: string;
    export let source: string;

    const dispatch = createEventDispatcher();

    function onCreate() {
        


    }

</script>



<Modal bind:show={show}>

    <div class="new-password-form"> 

        <div class="header">
            <h1>Add a new password</h1>
        </div>

        <div class="body">

            <div class="icon">
                <IconInput {image}></IconInput>
            </div>
         
            <div class="info-input">
                <input placeholder="Source" bind:value={source}>
                <PasswordInput {password} ></PasswordInput>
    
                <button>Create</button>
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