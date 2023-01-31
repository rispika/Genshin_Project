<template>
    <div class="UIDManager">
        <button @click="store.commit('setUID', v.uid)" 
        :class="{focus:store.state.UID === v.uid}"
        v-for="(v,index) in uids" :key="index">
        UID:{{ v.uid }}
        </button>
    </div>
</template>

<script setup>
import { invoke } from '@tauri-apps/api';
import { ref, onMounted } from 'vue';
import { useStore } from 'vuex';
const store = useStore()
const uids = ref([])
onMounted(async()=>{
    const UIDS = await invoke('get_data_uid')
    for(var key in UIDS) {
        var ob = {
            uid: UIDS[key] 
        }
        uids.value.push(ob)
    }
})
</script>

<style lang="scss" scoped>
.focus {
    background-color: rgba(131, 131, 131, 0.527);
}

.UIDManager {
    position: fixed;
    z-index: 10;
    top: 170px;
    right: 0px;
    max-height: calc(100vh - 350px);
    width: 240px;
    overflow: auto;

    button {
        margin-bottom: 10px;
        width: 100%;
        height: 60px;
        border-radius: 30px;
        backdrop-filter: blur(8px);
        font-size: 20px;
    }

    &::-webkit-scrollbar {
        height: 0;
        width: 0;
    }
}
</style>