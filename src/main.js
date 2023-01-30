import { createApp } from "vue";
import "./style.css";
import App from "./App.vue";
import router from './router/index.js';
import './assets/font-awesome-4.7.0/css/font-awesome.min.css'
import store from "./store/index";
const app = createApp(App);
app.use(router)
app.use(store)
app.mount("#app");
