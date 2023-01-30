import { createRouter, createWebHistory } from 'vue-router'
// 1. 定义路由组件.
// 也可以从其他文件导入
import Home from '../views/Home.vue'
import Card from '../views/Card.vue'
import t1 from '../views/t1.vue'
import t2 from '../views/t2.vue'
const routerHistory = createWebHistory()

const router = createRouter({
    linkActiveClass: 'router-active',
    history: routerHistory,
    routes: [
        {
            path: '/',
            component: Home
        },
        {
            path: '/card',
            component: Card
        },
        {
            path: '/t1',
            component: t1
        },
        {
            path: '/t2',
            component: t2
        }
    ]
})

export default router
