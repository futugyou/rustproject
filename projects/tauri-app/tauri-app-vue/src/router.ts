import { createRouter, createWebHistory, RouteRecordRaw } from 'vue-router'

const routes: RouteRecordRaw[] = [
    {
        path: '/',
        name: 'Home',
        component: () => import('../src/components/Welcome.vue')
    },
    {
        path: '/Cli',
        name: 'Cli',
        component: () => import('../src/components/Cli.vue')
    },
    {
        path: '/Clipboard',
        name: 'Clipboard',
        component: () => import('../src/components/Clipboard.vue')
    },
    {
        path: '/Dialog',
        name: 'Dialog',
        component: () => import('../src/components/Dialog.vue')
    },
    {
        path: '/Greet',
        name: 'Greet',
        component: () => import('../src/components/Greet.vue')
    },
    {
        path: '/Http',
        name: 'Http',
        component: () => import('../src/components/Http.vue')
    },
    {
        path: '/Show',
        name: 'Show',
        component: () => import('../src/components/Show.vue')
    },
    {
        path: '/TauriEvent',
        name: 'TauriEvent',
        component: () => import('../src/components/TauriEvent.vue')
    },
    {
        path: '/Notification',
        name: 'Notification',
        component: () => import('../src/components/Notification.vue')
    },
    {
        path: '/Welcome',
        name: 'Welcome',
        component: () => import('../src/components/Welcome.vue')
    },
]
const router = createRouter({
    history: createWebHistory("/"),
    routes: routes
})

export default router
