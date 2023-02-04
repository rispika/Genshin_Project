<template>
    <div class="home">
        <img class="shadow" src="../assets/img/klee.jpg" alt="囊哒哟!">
        <button class="start-game" @click="start_game">
            <i class="iconfont icon-Game"></i>
            开始游戏
        </button>
        <h1>欢迎使用Genshin Project Tools!</h1>
    </div>
</template>

<script setup>
import { invoke } from '@tauri-apps/api';
import { isPermissionGranted, requestPermission, sendNotification } from '@tauri-apps/api/notification';
import { appWindow } from '@tauri-apps/api/window';
const start_game = async () => {
    console.log(`游戏启动!`)
    try {
        await invoke('start_genshin')
        let permissionGranted = await isPermissionGranted();
        if (!permissionGranted) {
            const permission = await requestPermission();
            permissionGranted = permission === 'granted';
        }
        if (permissionGranted) {
            sendNotification({ title: 'Genshin Project', body: '游戏启动~原神工具箱已自动隐藏到托盘!' });
        }
        await appWindow.hide()
    } catch (error) {
        alert(`打开游戏失败,${error}`)
    }
}
</script>

<style lang="scss" scoped>
.home {
    min-height: calc(100vh - 35px);
    width: 100%;
    display: flex;
    box-sizing: border-box;
    padding: 20px 20px 20px 20px;
    justify-content: center;
    align-items: center;
    flex-direction: column;
    gap: 3.125rem;
    line-height: 60px;

    h1 {
        font-size: 60px;
    }

    img {
        height: 400px;
        width: 400px;
        border-radius: 20px;
    }
}

.start-game {
    border-radius: 30px;
    height: 100px;
    width: 400px;
    font-family: '华文琥珀';
    font-size: 45px;

    i {
        font-size: 45px;
    }
}
</style>