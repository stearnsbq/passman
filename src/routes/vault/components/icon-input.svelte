<script lang="ts">
    import logo from '../../../assets/svelte.png';

    let input;
    let img;

    let files;

    export let image = "";


    $: if (files) {

        toBase64(files[0]).then((result: string) => {
            image = result;
            img.src = image;
        });
            
	}

    function onImgError(){
 
        img.src = logo;
    }

    const toBase64 = file => new Promise((resolve, reject) => {
        const reader = new FileReader();
        reader.readAsDataURL(file);
        reader.onload = () => resolve(reader.result);
        reader.onerror = error => reject(error);
    });


</script>



<div class="icon-input" on:click={input.click()}>
    <img bind:this={img} on:error={onImgError} src={image} alt="Put Icon Here">

    <div class="overlay">
        <div class="text">Add New</div>
        <input bind:files bind:this={input}  accept=".png,.jpg,.jpeg,.gif" hidden type="file">
    </div>

</div>



<style lang="scss">

    .icon-input{

        width: 115px;
        height: 115px;
        position: relative;
        display: flex;
        justify-content: center;
        align-items: center;

        &:hover{
            span{
                display: block;
            }

            .overlay{
                display: flex;
            }

        }

        .overlay{
            cursor: pointer;
            border-radius: 50%;
            background-color: rgba($color: #000000, $alpha: 0.6);
            position: absolute;
            display: flex;
            flex-direction: column;
            justify-content: center;
            align-items: center;
            top: 0;
            bottom: 0;
            left: 0;
            right: 0;
            height: 100%;
            width: 100%;

            .text{
                color: white;
            }

           display: none;
        }

        img{
            height: 100px;
            width: 100px;
            border-radius: 50%;
        }
    }


</style>