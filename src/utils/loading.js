import { createApp, ref } from "vue";
import Loading from "../components/Loading.vue"
const divDom = document.createElement('div')
divDom.setAttribute('class','loading-container')
const flag_loading = ref(true)

const $loading = createApp(Loading,{flag_loading}).mount(divDom)

const loadPlgin = {
    show() {
        flag_loading.value = true
        document.body.appendChild($loading.$el)
    },
    hide() {
        flag_loading.value = false
    }
}

export default {
    install(app) {
        //3.0全局挂载
        app.config.globalProperties.$loading = loadPlgin
    }
}