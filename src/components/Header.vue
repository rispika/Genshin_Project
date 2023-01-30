<template>
    <div class="shadow" id="header" data-tauri-drag-region>
        <img src="../assets/img/paimeng64.ico" alt="">
        <span class="label" data-tauri-drag-region>Genshin Project</span>
        <button @click="appWindow.minimize()" class="btn" style="background-color: #0c8918"></button>
        <button @click="appWindow.toggleMaximize()" class="btn" style="background: #e9bb1d"></button>
        <button @click="appWindow.close()" class="btn" style="background: #c83c23"></button>
    </div>
</template>

<script setup>
import { appWindow } from '@tauri-apps/api/window'

</script>

<style scoped>
#header {
    background-color: rgb(232,232,232);
    height: 35px;
    left: 0%;
    top: 0%;
    width: 100%;
    position: fixed;
    display: flex;
    align-items: center;
    justify-content: right;
    gap: 1.875rem;
    padding: 0 10px 0 10px;
    box-sizing: border-box;
    z-index: 9999;
}
@media(prefers-color-scheme: Dark) {
    #header {
        background-color: rgb(33,37,43);
    }
    
}
#header img {
    position: absolute;
    border-radius: 5px;
    height: 20px;
    width: 20px;
    left: 8px;

}

.btn {
    right: 50px;
    height: 20px;
    width: 20px;
    border-radius: 50%;
    border: 1px solid rgba(0, 0, 0,0.2);
    cursor: pointer;

}
.label {
    position: absolute;
    left: 40px;
    font-weight: 600;
    font-size: 12px;
}
</style>