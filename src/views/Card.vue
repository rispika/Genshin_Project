<template>
    <div class="card">
        <div class="card_box shadow">
            <div id="chart301" class="echart_box"></div>
            <span class="all_count">共{{ count301 }}抽</span>
            <span class="time_count">{{ dataRange301 }}</span>
            <div class="msg_box shadow">
                <div v-for="(v, index) in dataCountList301" :key="index">
                    <div class="img_box">
                        <img :src="getAvatarFile(v.name)" :alt="v.name">
                    </div>
                    <span :style="{ width: v.count * 0.7 + '%' }"
                        :class="{ bg_yes: v.count < 63, bg_no: v.count >= 63 }">{{
                            v.count
                                <= 63 ? '欧' : '非'
                        }}</span>
                            {{ v.count }}抽
                </div>
            </div>
        </div>
        <div class="card_box shadow">
            <div id="chart302" class="echart_box"></div>
            <span class="all_count">共{{ count302 }}抽</span>
            <span class="time_count">{{ dataRange302 }}</span>

            <div class="msg_box shadow">
                <div v-for="(v, index) in dataCountList302" :key="index">
                    <div class="img_box">
                        <img :src="getAvatarFile(v.name)" :alt="v.name">

                    </div>
                    <span :style="{ width: v.count * 0.7 + '%' }"
                        :class="{ bg_yes: v.count < 63, bg_no: v.count >= 63 }">{{
                            v.count
                                < 63 ? '欧' : '非'
                        }}</span>
                            {{ v.count }}抽
                </div>
            </div>
        </div>
        <div class="card_box shadow">
            <div id="chart200" class="echart_box"></div>
            <span class="all_count">共{{ count200 }}抽</span>
            <span class="time_count">{{ dataRange200 }}</span>

            <div class="msg_box shadow">
                <div v-for="(v, index) in dataCountList200" :key="index">
                    <div class="img_box">
                        <img :src="getAvatarFile(v.name)" :alt="v.name">

                    </div>
                    <span :style="{ width: v.count * 0.7 + '%' }"
                        :class="{ bg_yes: v.count < 63, bg_no: v.count >= 63 }">{{
                            v.count
                                < 63 ? '欧' : '非'
                        }}</span>
                            {{ v.count }}抽
                </div>
            </div>
        </div>
        <button class="upper_btn" @click="refresh">
            <i ref="refreshIcon" class="iconfont icon-Loading" aria-hidden="true"></i></button>
    </div>
</template>

<script setup>
import { useStore } from 'vuex';
import { http } from '@tauri-apps/api'
import { message } from '@tauri-apps/api/dialog';
import { invoke } from '@tauri-apps/api';
import { onMounted, onBeforeUnmount, ref, computed, watch } from 'vue'
import { appWindow } from '@tauri-apps/api/window';
//  按需引入 echarts
import * as echarts from "echarts";
const store = useStore()
const count200 = ref(0), count301 = ref(0), count302 = ref(302)
const dataRange200 = ref(null), dataRange301 = ref(null), dataRange302 = ref(null)
var dataCountList200
var dataCountList301
var dataCountList302
var mychart1, mychart2, mychart3
var option1 = {
    title: {
        text: '原神UP角色池',
        left: 'center',
        textStyle: {
            color: '#000',
            fontWeight: 'bolder',
            fontFamily: '华文琥珀',
            fontSize: 26
        }
    },
    tooltip: {
        trigger: 'item'
    },
    legend: {
        orient: 'vertical',
        left: 'left',
        top: '50',
        textStyle: {
            color: '#000'
        }
    },
    color: ['#ffa631', '#8d4bbb', '#44cef6'],
    series: [
        {
            name: '原神UP角色池',
            type: 'pie',
            radius: ['30%', '50%'],
            data: [
                {
                    value: 1, name: '五星',
                    itemStyle: {
                        color: {
                            type: 'linear',
                            x: 0,
                            y: 0,
                            x2: 1,
                            y2: 1,
                            colorStops: [
                                { offset: 0, color: '#fdc830' },
                                { offset: 1, color: '#f37335' }
                            ]
                        }
                    }
                },
                {
                    value: 3, name: '四星',
                    itemStyle: {
                        color: {
                            type: 'linear',
                            x: 0,
                            y: 0,
                            x2: 1,
                            y2: 1,
                            colorStops: [
                                { offset: 0, color: '#89216b' },
                                { offset: 1, color: '#3c1053' }
                            ]
                        }
                    }
                },
                {
                    value: 11, name: '三星',
                    itemStyle: {
                        color: {
                            type: 'linear',
                            x: 0,
                            y: 0,
                            x2: 1,
                            y2: 1,
                            colorStops: [
                                { offset: 0, color: '#8f94fb' },
                                { offset: 1, color: '#4a00e0' }
                            ]
                        }
                    }
                }
            ],
            itemStyle: {
                borderRadius: 5,
                borderColor: 'rgb(33,37,43)',
                borderWidth: 2,
            },
            label: {
                formatter: ' {b}\n({d}%)',
                textStyle: {
                    fontWeight: 'normal',
                    fontSize: 12
                },
                color: '#000',  //字体颜色
                position: 'outside',
                width: 200
            },
            emphasis: {
                itemStyle: {
                    shadowBlur: 10,
                    shadowOffsetX: 0,
                    shadowColor: 'rgba(0, 0, 0, 0.5)'
                },
                focus: 'self'
            }
        }
    ]
}
var option2 = {
    title: {
        text: '原神UP武器池',
        left: 'center',
        textStyle: {
            color: '#fff',
            fontWeight: 'bolder',
            fontFamily: '华文琥珀',
            fontSize: 26
        }
    },
    tooltip: {
        trigger: 'item'
    },
    legend: {
        orient: 'vertical',
        left: 'left',
        top: '50',
        textStyle: {
            color: '#000'
        }
    },
    // color: ['#ffa631','#8d4bbb','#44cef6'],
    series: [
        {
            name: '原神UP武器池',
            type: 'pie',
            radius: ['30%', '50%'],
            data: [
                {
                    value: 0, name: '五星',
                    itemStyle: {
                        color: {
                            type: 'linear',
                            x: 0,
                            y: 0,
                            x2: 1,
                            y2: 1,
                            colorStops: [
                                { offset: 0, color: '#fdc830' },
                                { offset: 1, color: '#f37335' }
                            ]
                        }
                    }
                },
                {
                    value: 0, name: '四星',
                    itemStyle: {
                        color: {
                            type: 'linear',
                            x: 0,
                            y: 0,
                            x2: 1,
                            y2: 1,
                            colorStops: [
                                { offset: 0, color: '#89216b' },
                                { offset: 1, color: '#3c1053' }
                            ]
                        }
                    }
                },
                {
                    value: 0, name: '三星',
                    itemStyle: {
                        color: {
                            type: 'linear',
                            x: 0,
                            y: 0,
                            x2: 1,
                            y2: 1,
                            colorStops: [
                                { offset: 0, color: '#8f94fb' },
                                { offset: 1, color: '#4a00e0' }
                            ]
                        }
                    }
                }
            ],
            itemStyle: {
                borderRadius: 5,
                borderColor: 'rgb(33,37,43)',
                borderWidth: 2,
            },
            label: {
                formatter: ' {b}\n({d}%)',
                textStyle: {
                    fontWeight: 'normal',
                    fontSize: 12
                },
                color: '#fff',  //字体颜色
                position: 'outside',
                width: 200
            },
            emphasis: {
                itemStyle: {
                    shadowBlur: 10,
                    shadowOffsetX: 0,
                    shadowColor: 'rgba(0, 0, 0, 0.5)'
                },
                focus: 'self'
            },
        }
    ]
}
var option3 = {
    title: {
        text: '原神常驻池',
        left: 'center',
        textStyle: {
            color: '#fff',
            fontWeight: 'bolder',
            fontFamily: '华文琥珀',
            fontSize: 26
        }
    },
    tooltip: {
        trigger: 'item'
    },
    legend: {
        orient: 'vertical',
        left: 'left',
        top: '50',
        textStyle: {
            color: '#000'
        }
    },
    color: ['#ffa631', '#8d4bbb', '#44cef6'],
    series: [
        {
            name: '原神常驻池',
            type: 'pie',
            radius: ['30%', '50%'],
            data: [
                {
                    value: 0, name: '五星',
                    itemStyle: {
                        color: {
                            type: 'linear',
                            x: 0,
                            y: 0,
                            x2: 1,
                            y2: 1,
                            colorStops: [
                                { offset: 0, color: '#fdc830' },
                                { offset: 1, color: '#f37335' }
                            ]
                        }
                    }
                },
                {
                    value: 0, name: '四星',
                    itemStyle: {
                        color: {
                            type: 'linear',
                            x: 0,
                            y: 0,
                            x2: 1,
                            y2: 1,
                            colorStops: [
                                { offset: 0, color: '#89216b' },
                                { offset: 1, color: '#3c1053' }
                            ]
                        }
                    }
                },
                {
                    value: 0, name: '三星',
                    itemStyle: {
                        color: {
                            type: 'linear',
                            x: 0,
                            y: 0,
                            x2: 1,
                            y2: 1,
                            colorStops: [
                                { offset: 0, color: '#8f94fb' },
                                { offset: 1, color: '#4a00e0' }
                            ]
                        }
                    }
                }
            ],
            itemStyle: {
                borderRadius: 5,
                borderColor: 'rgb(245,245,245)',
                borderWidth: 2,
            },
            label: {
                formatter: ' {b}\n({d}%)',
                textStyle: {
                    fontWeight: 'normal',
                    fontSize: 12
                },
                color: '#fff',  //字体颜色
                position: 'outside',
                width: 200
            },
            emphasis: {
                itemStyle: {
                    shadowBlur: 10,
                    shadowOffsetX: 0,
                    shadowColor: 'rgba(0, 0, 0, 0.5)'
                },
                focus: 'self'
            }
        }
    ]
}
const theme = computed(() => {
    return store.state.theme
})
const UID = computed(()=>{
    return store.state.UID
})
onMounted(async () => {
    store.commit('setFlagLoading', true)
    checkTheme(await appWindow.theme())
    initData()
    await setOptions()
    watch(theme, (newVal) => {
        checkTheme(newVal)
        destoryCharts()
        initCharts()
        store.commit('setFlagLoading', false)
    }, { immediate: true, deep: true })
    watch(UID, async(newVal) => {
        await setOptions()
        destoryCharts()
        initCharts()
    },{ immediate: true, deep: true })
})
onBeforeUnmount(() => {
    destoryCharts()
})


//获取图表信息->打包
const getAvatarFile = (name) => {
    return new URL(`../assets/avatar/${name}.webp`, import.meta.url).href
}
//初始化系统主题信息
async function checkTheme(theme) {
    var themeColor = theme === 'dark' ? '#fff' : '#000'
    var themeBorder = theme === 'dark' ? 'rgb(33,37,43)' : 'rgb(245,245,245)'
    console.log(`themeColor=${themeColor}`);
    option1.series[0].itemStyle.borderColor = themeBorder
    option2.series[0].itemStyle.borderColor = themeBorder
    option3.series[0].itemStyle.borderColor = themeBorder
    option1.title.textStyle.color = themeColor
    option1.legend.textStyle.color = themeColor
    option1.series[0].label.color = themeColor
    option2.title.textStyle.color = themeColor
    option2.legend.textStyle.color = themeColor
    option2.series[0].label.color = themeColor
    option3.title.textStyle.color = themeColor
    option3.legend.textStyle.color = themeColor
    option3.series[0].label.color = themeColor
}
//初始化图表
function initCharts() {
    const chart301 = document.querySelector('#chart301')
    const chart302 = document.querySelector('#chart302')
    const chart200 = document.querySelector('#chart200')
    mychart1 = echarts.init(chart301)
    mychart1.setOption(option1)
    mychart2 = echarts.init(chart302)
    mychart2.setOption(option2)
    mychart3 = echarts.init(chart200)
    mychart3.setOption(option3)
}
//摧毁图表
function destoryCharts() {
    if (mychart1!==undefined&&mychart1!==null) mychart1.dispose()
    if (mychart2!==undefined&&mychart2!==null) mychart2.dispose()
    if (mychart3!==undefined&&mychart3!==null) mychart3.dispose()
}
//图标大小自适应
window.addEventListener('resize', () => {
    if (mychart1) mychart1.resize()
    if (mychart2) mychart2.resize()
    if (mychart3) mychart3.resize()
})
//初始化数据
async function initData() {
    console.log(`开始执行initData`);
    console.log(`开始查询索引`);
    try {
        const uids = await invoke('get_data_uid')
        console.log(`uids=${uids}`);
        if (uids.length === 0) {
            console.log(`开始获取卡池url`);
            const url = await invoke('get_gacha_url')
            console.log(`卡池url_pre=${url}`)
            if (!url) {
                await message('请登录原神,打开抽卡记录后,再次打开本软件重试!-noUrl')
                await appWindow.close()
            }
            if (await request(url)) {
                await message('请登录原神,打开抽卡记录后,再次打开本软件重试!-noResp')
                await appWindow.close()
            } else {
                if (store.state.UID === 'NONE') {
                    store.dispatch('updateUID')
                }
            }
        }
    } catch (error) {
        console.log(`initData Err = ${error}`);
        await message(`请登录原神,打开抽卡记录后,再次打开本软件重试!${error}`)
        await appWindow.close()
    }

    await setOptions()
}
//设置渲染的数据!
async function setOptions() {
    console.log(`开始渲染数据!`);
    try {
        //1.数据图表
        option1.series[0].data[0].value = await invoke('get_count', {
            gachaType: '301',
            rankType: '5',
            uid: store.state.UID
        })
        option1.series[0].data[1].value = await invoke('get_count', {
            gachaType: '301',
            rankType: '4',
            uid: store.state.UID
        })
        option1.series[0].data[2].value = await invoke('get_count', {
            gachaType: '301',
            rankType: '3',
            uid: store.state.UID
        })
        count301.value = option1.series[0].data[0].value + option1.series[0].data[1].value + option1.series[0].data[2].value
        //
        option2.series[0].data[0].value = await invoke('get_count', {
            gachaType: '302',
            rankType: '5',
            uid: store.state.UID
        })
        option2.series[0].data[1].value = await invoke('get_count', {
            gachaType: '302',
            rankType: '4',
            uid: store.state.UID
        })
        option2.series[0].data[2].value = await invoke('get_count', {
            gachaType: '302',
            rankType: '3',
            uid: store.state.UID
        })
        count302.value = option2.series[0].data[0].value + option2.series[0].data[1].value + option2.series[0].data[2].value
        //
        option3.series[0].data[0].value = await invoke('get_count', {
            gachaType: '200',
            rankType: '5',
            uid: store.state.UID
        })
        option3.series[0].data[1].value = await invoke('get_count', {
            gachaType: '200',
            rankType: '4',
            uid: store.state.UID
        })
        option3.series[0].data[2].value = await invoke('get_count', {
            gachaType: '200',
            rankType: '3',
            uid: store.state.UID
        })
        count200.value = option3.series[0].data[0].value + option3.series[0].data[1].value + option3.series[0].data[2].value
        //2.下部5星数据
        store.commit('setLoadingMsg', '马上了!')
        dataCountList200 = await invoke('get_count_rank_list', { gachaType: '200', uid: store.state.UID })
        dataCountList301 = await invoke('get_count_rank_list', { gachaType: '301', uid: store.state.UID })
        dataCountList302 = await invoke('get_count_rank_list', { gachaType: '302', uid: store.state.UID })
        dataCountList200 = dataCountList200.reverse()
        dataCountList301 = dataCountList301.reverse()
        dataCountList302 = dataCountList302.reverse()
        //3.日期数据
        dataRange200.value = await invoke('get_gacha_time', { gachaType: '200', uid: store.state.UID })
        dataRange301.value = await invoke('get_gacha_time', { gachaType: '301', uid: store.state.UID })
        dataRange302.value = await invoke('get_gacha_time', { gachaType: '302', uid: store.state.UID })
    } catch (error) {
        store.commit('setFlagLoading', true)
        store.commit('setLoadingMsg', `发生故障,重新读取中`)
        await refresh()
    }

}
//整合所有fetch请求
async function request(url) {
    //   list1 = await fetch(url, 100);
    const flag200 = await fetch(url, 200)
    const flag301 = await fetch(url, 301)
    const flag302 = await fetch(url, 302)
    return flag200 || flag301 || flag302
}
//发出请求抓取卡池信息
async function fetch(url, type) {
    var list = []
    var page = 1
    var end_id = 0
    var msg
    switch (type) {
        case 301:
            msg = '活动up卡池'
            break
        case 302:
            msg = '武器up卡池'
            break
        case 200:
            msg = '常驻卡池'
            break
        case 100:
            msg = '新手卡池'
            break
        default:
            msg = '未知卡池'
            break
    }
    const httpConfig = {
        method: 'GET',
        responseType: 1
    }
    var flag_history = false
    while (true) {
        const dataUrl = url + '&gacha_type=' + type + '&page=' + page + '&size=20&end_id=' + end_id
        console.log(`dataUrl=${dataUrl}`);
        const res = await http.fetch(dataUrl, httpConfig)
        if (res.ok !== true) console.log(`error!code=${res.status}`);
        if (res.data.data === null) {
            console.log("权限过期");
            if (page === 1) flag_history = true
            break
        }
        if (res.data.data.list.length === 0) break
        list.push(...res.data.data.list)
        end_id = res.data.data.list[res.data.data.list.length - 1].id
        console.log(`${msg}:第${page}页打印结束`);
        store.commit('setLoadingMsg', `${msg}:第${page}页打印结束`)
        page++
    }
    console.log(list);
    invoke('add_data_list', { dataList: list })
    return flag_history
}
//刷新操作!
const refresh = async () => {
    store.commit('setFlagLoading', true)
    destoryCharts()
    console.log(`开始获取卡池url`);
    try {
        const url = await invoke('get_gacha_url')
        console.log(`卡池url_pre=${url}`)
        if (!url) {
            await message('请登录原神,打开抽卡记录后,再次刷新:无url')
        }
        if (await request(url)) {
            await message('请登录原神,打开抽卡记录后,再次刷新:请求为空')
        }
        if (store.state.UID === 'NONE') {
            store.dispatch('updateUID')
        }
    } catch (error) {
        console.log(`initData Err = ${error}`);
        await message('请登录原神,打开抽卡记录后,再次打开本软件重试!')
        await appWindow.close()
    }
    await setOptions()
    initCharts()
    store.commit('setFlagLoading', false)

}
</script>

<style lang="scss" scoped>
.card {
    position: relative;
    box-sizing: border-box;
    padding: 20px;
    min-height: 100%;
    display: flex;
    gap: 20px;
    justify-content: center;
    align-items: center;
    flex-wrap: wrap;
}

.card_box {
    position: relative;
    background-color: rgb(245, 245, 245);
    /* height: 700px;
    width: 320px; */
    min-height: 700px;
    min-width: 300px;
    height: 100%;
    width: 20%;
    border-radius: 20px;
    border: 0;
    display: flex;
    flex-direction: column;
}

.echart_box {
    position: relative;
    min-width: 300px;
    min-height: 300px;
    width: 100%;
    /* border-radius: 20px; */
    /* background-color: red; */
}

.all_count {
    position: absolute;
    top: 140px;
    left: 50%;
    transform: translate(-50%);
    z-index: 10;
}

.time_count {
    position: absolute;
    top: 270px;
    left: 50%;
    transform: translate(-50%);
    z-index: 10;
    font-family: '黑体';
    display: block;
    width: 100%;
    text-align: center;
}

.msg_box {
    flex: 1;
    box-sizing: border-box;
    padding: 8px 8px 8px 8px;
    text-align: center;
    background-color: rgb(233, 233, 233);
    border-radius: 15px;
    display: flex;
    flex-direction: column;
    gap: 8px;
}

.msg_box div {
    position: relative;
    height: 50px;
    display: flex;
    justify-content: left;
    align-items: center;
    /* background-color: red; */
    gap: 10px;
    overflow: hidden;
}

.msg_box img {
    height: 100%;
    width: 100%;
    object-fit: cover;
}

.msg_box div span {
    border-radius: 30px;
    height: 30px;
    width: 60%;
    min-width: 10%;
    max-width: 60%;
    line-height: 30px;
}

.img_box {
    height: 50px;
    width: 50px;
    background: url(../assets/5star.webp) no-repeat;
    background-size: cover;
    border-radius: 5px;
}

.bg_yes {
    background: linear-gradient(to right, #a8ff78, #00f260, #38ef7d);
}

.bg_no {
    background: linear-gradient(to right, #ff416c, #e94057, #f27121);
}

.upper_btn {
    position: fixed;
    right: 5%;
    bottom: 5%;
    height: 80px;
    width: 80px;
    border-radius: 50%;
    backdrop-filter: blur(5px);
    transition: all .5s ease;
}

.upper_btn:hover {
    transform: rotate(-90deg);
}

.icon-Loading {
    font-size: 50px;
}

body[data-theme=dark] {
    .card_box {
        background-color: rgb(33, 37, 43);
        box-shadow: 1px 0.5px 1px 1px rgba(0, 0, 0, 0.205);

    }

    .msg_box {
        background-color: rgb(56, 59, 65);
    }

}
</style>