<template>
  <div id="home" @contextmenu.prevent>
    <Header></Header>
    <div class="main">
      <Nav></Nav>
      <Transition>
        <router-view class="view"></router-view>
      </Transition>
    </div>
    <span class="uid">UID:{{ store.state.UID }}</span>
    <div v-show="store.state.flagMenu" class="menu" :class="{ 'menu-show': flagMenu }">
      <button @click="flagMenu = !flagMenu;flagUIDManager = false"><span><i class="iconfont icon-liebiao"></i></span>菜单</button>
      <button @click="flagUIDManager = !flagUIDManager"><span><i class="iconfont icon-guanliyuansousuo"></i></span>UID</button>
      <button @click="changeTheme">
        <span>
          <Transition name="theme">
            <i v-if="store.state.theme === 'light'" class="iconfont icon-lieri"></i>
          </Transition>
          <Transition name="theme">
            <i v-if="store.state.theme === 'dark'" class="iconfont icon-yejing"></i>
          </Transition>
        </span>
        主题
      </button>
    </div>
    <Transition>
      <UIDManager v-if="flagUIDManager"></UIDManager>
    </Transition>
    <Loading></Loading>
    <Notice @restart="initProject"></Notice>
  </div>
</template>

<script setup>
import UIDManager from './components/UIDManager.vue'
import Notice from './components/Notice-init.vue'
import Loading from './components/Loading.vue'
import Header from './components/Header.vue';
import Nav from './components/Nav.vue';
import { onMounted, watch, computed, ref } from 'vue';
import { useStore } from 'vuex';
import { invoke } from '@tauri-apps/api';
import { appWindow } from '@tauri-apps/api/window';
const flagMenu = ref(false)
const flagUIDManager = ref(false)
const store = useStore()
onMounted(async () => {
  initTheme()
  store.commit('setFlagLoading', true)
  initProject()
})
const initTheme = async () => {
  store.commit('setTheme', await appWindow.theme())
  if (store.state.theme === 'dark') {
    const body = document.querySelector('body')
    body.setAttribute('data-theme', 'dark')
  }
}
const theme = computed(() => {
  return store.state.theme
})
watch(theme, (newVal) => {
  console.log(`切换主题:${newVal}`);
  const body = document.querySelector('body')
  body.setAttribute('data-theme', newVal)

}, { immediate: true, deep: true })
//修改主题
const changeTheme = () => {
  if (store.state.theme === 'dark') {
    store.commit('setTheme', 'light')
  } else {
    store.commit('setTheme', 'dark')
  }
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

<style lang="scss" scoped>
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

//菜单
.menu-show {
  transform: translate(0px) !important;
}

.menu {
  position: fixed;
  z-index: 11;
  right: 0%;
  top: 80px;
  height: 80px;
  width: 240px;
  border-radius: 80px 0 0 80px;
  transition: all .5s ease;
  transform: translate(160px);
  backdrop-filter: blur(5px);
  background-color: rgba(255, 255, 255, 0.3);
  display: flex;
  overflow: hidden;

  button {
    position: relative;
    height: 80px;
    width: 80px;
    transition: all .5s ease;
    display: flex;
    align-items: center;
    justify-content: center;
    overflow: hidden;
    flex-direction: column;
    
    span {
      height: 40px;
      display: flex;
      align-items: center;
      justify-content: center;
      i {
        position: absolute;
        font-size: 30px;
      }
    }

  }
}

// 菜单结束
.v-enter-active,
.v-leave-active,
.theme-enter-active,
.theme-leave-active {
  transition: all .5s ease;
}

.v-enter-from,
.v-leave-to {
  opacity: 0;
}

.theme-enter-from {
  opacity: 0;
  transform: translateY(50px);
}

.theme-leave-to {
  opacity: 0;
  transform: translateY(-50px);
}

@media screen and (min-width: 1600px) {
  .uid {
    font-size: 36px;
  }
}

body[data-theme=dark] {
  .menu {
    background-color: rgba(0, 0, 0, 0.3);
  }
}
</style>
