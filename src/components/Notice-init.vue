<template>
    <div v-if="store.state.noticeInitFlag" class="notice shadow">
        <div>
            <h1>❥(^_-)自动初始化失败</h1>
        </div>
        <div class="line"></div>
        <div>
            <h2>
                解决方案:
            </h2>
            <h3 style="font-family: '黑体';margin-top: 10px;">
                1.打开原神->打开祈愿->打开抽卡记录后->点击左下角重试自动初始化。
            </h3>
            <h3 style="font-family: '黑体';margin-top: 20px;">
                2.点击右下角->手动定位原神位置。
                （定位要求:定位至原神安装目录内Genshin Impact Game文件夹）
            </h3>
        </div>
        <div class="line"></div>
        <div>
            <button @click="restart">
                重试
                <i class="fa fa-refresh" aria-hidden="true"></i>
            </button>
            <button @click="position">
                手动定位
                <i class="fa fa-file" aria-hidden="true"></i>
            </button>
        </div>
    </div>
</template>

<script setup>
import { useStore } from 'vuex';
import { defineEmits } from 'vue'
import { open } from '@tauri-apps/api/dialog';
import { writeTextFile, exists, BaseDirectory  } from '@tauri-apps/api/fs';

const store = useStore()
const emits = defineEmits(['restart']);
const restart = () => {
    store.commit('setNoticeInitFlag', false)
    emits(restart)
}
const position = async () => {
    console.log(123);
    const selected = await open({
        directory: true
    });
    console.log(`你选择的路径为${selected}`);
    if (await exists(`${selected}\\YuanShen.exe`)) {
        store.commit('setNoticeInitFlag', false)
        console.log(`寻找到原神文件夹`);
        await writeTextFile('genshin_project\\path_log.txt', selected, {dir: BaseDirectory.LocalData});
        emits('restart')
    } else {
        alert(`该文件夹非目标文件夹!`)
    }
}
</script>

<style scoped>
.notice {
    position: fixed;
    z-index: 999;
    background-color: rgb(250, 250, 250);
    left: 50%;
    top: 50%;
    height: 400px;
    width: 550px;
    margin-left: -275px;
    margin-top: -180px;
    border-radius: 20px;
}

.notice :nth-child(1) {
    height: 74px;
    display: flex;
    align-items: center;
    justify-content: center;
}

.notice :nth-child(3) {
    height: 250px;
}

.notice :nth-child(5) {
    display: flex;
    align-items: center;
    justify-content: space-around;
    height: 74px;
}

.notice :nth-child(5) button {
    display: flex;
    align-items: center;
    justify-content: center;
    height: 50px;
    width: 150px;
    gap: 10px;
    line-height: 50px;
    font-size: 20px;
    border-radius: 10px;
    font-weight: 1000;
    background-color: rgb(229, 229, 229);
}

.line {
    height: 1px;
    width: 100%;
    background-color: rgb(42, 42, 42);
}

@media (prefers-color-scheme: Dark) {
    .notice {
        background-color: rgb(64, 69, 80);
    }

    .line {
        background-color: rgba(255, 255, 255, 0.5);
    }

    .notice :nth-child(5) button {
        background-color: rgb(83, 82, 82);
    }
}
</style>