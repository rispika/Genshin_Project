<template>
  <div id="home" @contextmenu.prevent>
    <Header></Header>
    <div class="main">
      <Nav></Nav>
      <Transition>
        <router-view class="view" v-slot="{ Component }">
          <KeepAlive include="Card">
            <component class="view" :is="Component" />
          </KeepAlive>
        </router-view>
      </Transition>
    </div>
    <span class="uid">UID:{{ store.state.UID }}</span>
    <button class="theme_btn" @click="changeTheme">
      <Transition name="sun">
        <i v-if="theme_bool" class="iconfont icon-lieri theme_item"></i>
      </Transition>
      <Transition name="moon">
        <i v-if="!theme_bool" class="iconfont icon-yejing theme_item"></i>
      </Transition>
    </button>
    <Loading></Loading>
    <Notice @restart="initProject"></Notice>
  </div>
</template>

<script setup>
import Notice from './components/Notice-init.vue'
import Loading from './components/Loading.vue'
import Header from './components/Header.vue';
import Nav from './components/Nav.vue';
import { currentMonitor } from '@tauri-apps/api/window';
import { onMounted, ref } from 'vue';
import { useStore } from 'vuex';
import { invoke } from '@tauri-apps/api';
import { appWindow } from '@tauri-apps/api/window';
const store = useStore()
const theme_bool = ref(null);
const monitor = await currentMonitor();
console.log(`monitor=${monitor}`);
onMounted(async () => {
  store.commit('setFlagLoading', true)
  initProject()
  theme_bool.value = await appWindow.theme() === 'light'
})
const changeTheme = () => {
  theme_bool.value = !theme_bool.value
  
}
async function initProject() {
  store.dispatch('updateUID')
  try {
    await invoke('check_genshin_path')
    store.commit('setFlagLoading', false)

  } catch (err) {
    console.log(`check_genshin_path 发生错误:${err}`);
    store.commit('setNoticeInitFlag', true)
  }

}
</script>

<style scoped>
#home {
  min-height: 100%;
  box-sizing: border-box;
  height: 100%;
  color: black;
  padding-top: 35px;
}

.main {
  height: calc(100vh - 35px);
  display: flex;
}

.uid {
  position: fixed;
  right: 1%;
  bottom: 1%;
  font-family: '华文琥珀';
  font-size: 20px;
}

.view {
  flex: 1;
}

.theme_btn {
  position: fixed;
  right: 5%;
  top: 10%;
  height: 80px;
  width: 80px;
  border-radius: 50%;
  backdrop-filter: blur(5px);
  transition: all .5s ease;
  display: flex;
  align-items: center;
  justify-content: center;
  overflow: hidden;
}

.theme_item {
  position: absolute;
  font-size: 35px;
}

.v-enter-active,
.v-leave-active {
  transition: all .5s ease;
}

.v-enter-from,
.v-leave-to {
  opacity: 0;
}
</style>
