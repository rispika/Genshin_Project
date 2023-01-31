<template>
    <div v-if="store.state.flagLoading" class="loading"  :style="{ width: 'calc(100% - ' + store.state.loadingMargin + 'px)' }">
        <div class="out_box">
            <span></span>
            <span></span>
            <span></span>
            <span></span>
            <span></span>
        </div>
        <h1 class="inner_text">{{ store.state.loadingMsg }}</h1>
        <h1 class="inner_text">{{ poetry[radom].v }}</h1>
        <h2 style="transform: translateX(120px);font-family: '华文琥珀'">--{{ poetry[radom].k }}</h2>
    </div>
</template>
<script setup>
import { ref } from 'vue';
import { useStore } from 'vuex';
const radom = ref(parseInt(Math.random() * 14))
const store = useStore()
const poetry = ref([
    {
        v: '山有木兮木有枝，心悦君兮君不知。',
        k: '佚名《越人歌》'
    },
    {
        v: '人生若只如初见，何事秋风悲画扇。',
        k: '纳兰性德《木兰词·拟古决绝词柬友》'
    },
    {
        v: '十年生死两茫茫，不思量，自难忘。',
        k: '苏轼《江城子·乙卯正月二十日夜记梦》'
    },
    {
        v: '曾经沧海难为水，除却巫山不是云。',
        k: '元稹《离思五首·其四》'
    },
    {
        v: '玲珑骰子安红豆，入骨相思知不知。',
        k: '温庭筠《南歌子词二首 / 新添声杨柳枝词》'
    },
    {
        v: '只愿君心似我心，定不负相思意。',
        k: '李之仪《卜算子·我住长江头》'
    },
    {
        v: '平生不会相思，才会相思，便害相思。',
        k: '徐再思《折桂令·春情》'
    },
    {
        v: '愿得一心人，白头不相离。',
        k: '卓文君《白头吟》'
    },
    {
        v: '山无陵，江水为竭。冬雷震震，夏雨雪。天地合，乃敢与君绝。',
        k: '佚名《上邪》'
    },
    {
        v: '入我相思门，知我相思苦。',
        k: '李白《三五七言 / 秋风词》'
    },
    {
        v: '海上生明月，天涯共此时。',
        k: '张九龄《望月怀远》'
    },
    {
        v: '月上柳梢头，人约黄昏后。',
        k: '欧阳修《生查子·元夕》'
    },
    {
        v: '明月几时有？把酒问青天。',
        k: '苏轼《水调歌头·丙辰中秋》'
    }
])
</script>
    
<style lang="scss" scoped>
*{
    color: #606060;
}
.loading {
    position: fixed;
    z-index: 99;
    right: 0%;
    top: 35px;
    height: calc(100% - 35px);
    background-color: #e3f9fd6b;
    backdrop-filter: blur(12px);
    display: flex;
    align-items: center;
    justify-content: center;
    flex-direction: column;
    gap: 50px;
    will-change: width;
    transition: all .5s ease;
}

.out_box {
    position: relative;
    height: 150px;
    width: 150px;
    box-shadow: 0;
}

.out_box span {
    position: absolute;
    height: 100%;
    width: 100%;
    animation: loading 3.5s linear infinite;
    box-shadow: 0;
}

span::before {
    position: absolute;
    content: '';
    height: 15px;
    width: 15px;
    bottom: 0;
    border-radius: 50%;
    left: calc(50% - 10px);
    background-color: #606060;
}

span:nth-child(1) {
    animation-delay: 0.1s;
}

span:nth-child(2) {
    animation-delay: 0.2s;
}

span:nth-child(3) {
    animation-delay: 0.3s;
}

span:nth-child(4) {
    animation-delay: 0.4s;
}

span:nth-child(5) {
    animation-delay: 0.5s;
}

.inner_text {
    position: relative;
    font-size: 30px;
    line-height: 30px;
    font-family: '华文琥珀';
    /* background-image: linear-gradient(to right bottom,#e0eafc,#cfdef3);
    -webkit-background-clip: text;
    -webkit-text-fill-color: transparent; */
}

@keyframes loading {
    75% {
        transform: rotate(600deg);
    }

    79% {
        transform: rotate(720deg);
        opacity: 1;
    }

    80% {
        transform: rotate(720deg);
        opacity: 0;
    }

    100% {
        transform: rotateZ(810deg);
        opacity: 0;
    }
}

body[data-theme=dark] {
    .loading {
        background-color: #8080804f;
    }

    span::before {
        background-color: #fff;
    }
    *{
        color: #fff;
    }
}
</style>