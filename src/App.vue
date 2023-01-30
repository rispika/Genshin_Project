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
    <Loading></Loading>
  </div>
</template>

<script setup>
import Loading from './components/Loading.vue'
import Header from './components/Header.vue';
import Nav from './components/Nav.vue';
import { onMounted } from 'vue';
import { useStore } from 'vuex';
const store = useStore()
onMounted(async () => {
  store.dispatch('updateUID')
  store.commit('setFlagLoading',true)
  console.log(`当前loading-flag = ${store.state.flagLoading}`);
})
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

.v-enter-active,
.v-leave-active {
  transition: all .5s ease;
}
.v-enter-from,
.v-leave-to {
  opacity: 0;
}
</style>
