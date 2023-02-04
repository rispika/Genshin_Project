<template>
    <div>
        <!-- https://webstatic.mihoyo.com/ys/app/interactive-map/index.html?lang=zh-cn#/map/2 -->
        
        <iframe id="show-iframe" frameborder="0" dcrolling="yes" style="background-color:transparent;height:100%;width: 100%; top:0;left:0;
            	bottom:0;" src="https://webstatic.mihoyo.com/ys/app/interactive-map/index.html?lang=zh-cn#/">
        </iframe>
    </div>
</template>

<script setup>
import { onMounted,onUnmounted } from 'vue';
import { useStore } from 'vuex';
const store = useStore()
onMounted(()=>{
    store.commit('setFlagMenu', false)
})
onUnmounted(()=>{
    store.commit('setFlagMenu', true)
})
</script>

<style lang="scss" scoped>

</style>