import { createApp } from "vue";
import "./style.scss";
import App from "./App.vue";
import router from './router/index.js';
import './assets/iconfont/iconfont.css'
import store from "./store/index";
const app = createApp(App);
app.use(router)
app.use(store)
app.mount("#app");
