<template>
    <div class="nav shadow" :class="[{ long_nav: flag_arrow }]">
        <div class="nav_fix" :class="[{ long_nav: flag_arrow }]">
            <button @click="clickArrow" >
                <i :class="[{ i_click: flag_arrow }]" class="iconfont icon-xiangxia arrow"></i>
            </button>
            <router-link tag="button" class="link" to="/">
                <i class="iconfont icon-zhuye" aria-hidden="true"></i>
                <transition>
                    <span v-show="flag_arrow">主页</span>
                </transition>
            </router-link>
            <router-link tag="button" class="link" to="/card">
                <i class="iconfont icon-bingtu" aria-hidden="true"></i>
                <Transition>
                    <span v-show="flag_arrow">抽卡分析</span>
                </Transition>
            </router-link>
            <router-link tag="button" class="link" to="/t1">
                <i class="iconfont icon-shujuwajue" aria-hidden="true"></i>
                <Transition>
                    <span v-show="flag_arrow">还没做捏</span>
                </Transition>
            </router-link>
            <router-link tag="button" class="link" to="/t2">
                <i class="iconfont icon-qiapianxingshi" aria-hidden="true"></i>
                <Transition>
                    <span v-show="flag_arrow">嗨害害!</span>
                </Transition>
            </router-link>
            <router-link tag="button" class="link" to="/map">
                <i class="iconfont icon-ditu" aria-hidden="true"></i>
                <Transition>
                    <span v-show="flag_arrow">提瓦特地图</span>
                </Transition>
            </router-link>
            <div class="split"></div>
        </div>

    </div>
</template>

<script setup>
import { ref } from 'vue';
import { useStore } from "vuex";
const emits = defineEmits(['changeNav'])
const flag_arrow = ref(false)
const store = useStore()
const clickArrow = () => {
    flag_arrow.value = !flag_arrow.value
    if (flag_arrow.value == true) {
        store.commit('setLoadingMargin', 150)
    } else {
        store.commit('setLoadingMargin', 40)
    }
}
</script>

<style lang="scss" scoped>
.nav {
    position: relative;;
    min-height: 100%;
    width: 40px;
    height: 100%;
    background-color: rgb(245, 245, 245);
    transition: width .5s ease;
    will-change: width;
    z-index: 10;
}
.nav_fix {
    position: fixed;
    left: 0%;
    width: 40px;
    background-color: rgb(245, 245, 245);
    height: 100%;
    display: flex;
    flex-direction: column;
    align-items: center;
    transition: width .5s ease;
    will-change: width;
    z-index: 10;
}
.iconfont {
    font-size: 20px;
}

.nav span {
    font-weight: 600;
}

.link {
    color: #393e46;
    text-decoration: none;
    height: 40px;
    width: 100%;
    background-color: transparent;
    display: flex;
    justify-content: space-between;
    align-items: center;
    box-sizing: border-box;
    padding: 0 10px 0 12px;
}

.nav button {
    height: 40px;
    width: 100%;
    cursor: pointer;
    border: 0;
    background-color: transparent;
}

.link:hover {
    background-color: rgba(0, 0, 0, 0.15);
}

div .long_nav {
    width: 150px;
}
.arrow {
    display: inline-block;
    transition: all .5s ease;
}

.i_click {
    transform: rotateZ(-90deg) !important;
}

.split {
    background-color: rgb(0, 0, 0, 0.3);
    height: 0.1px;
    width: 100%;
}

.router-active {
    background-color: rgba(0, 0, 0, 0.15);
}


.v-enter-active,
.v-leave-active {
    transition: all .5s ease;
    will-change: width;
}

.v-enter-from,
.v-leave-to {
    opacity: 0;
    width: 0;
    transform: scale(0);
}

body[data-theme=dark] {
    .nav {
        background-color: rgb(33, 37, 43);
    }
    .nav_fix {
        background-color: rgb(33, 37, 43);
    }
    .split {
        background-color: rgba(255, 255, 255, 0.3);
    }

    .link:hover {
        background-color: rgb(50, 56, 66);
    }

    .router-active {
        background-color: rgb(50, 56, 66);
    }
}
</style>