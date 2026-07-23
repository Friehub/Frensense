// SAFE: beforeEach navigation guard checks authentication and redirects to login

import { createRouter, createWebHistory, type RouteRecordRaw } from 'vue-router'
import Dashboard from './views/Dashboard.vue'
import Settings from './views/Settings.vue'

const routes: RouteRecordRaw[] = [
  { path: '/login', component: () => import('./views/Login.vue') },
  { path: '/dashboard', component: Dashboard, meta: { requiresAuth: true } },
  { path: '/settings', component: Settings, meta: { requiresAuth: true } }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

router.beforeEach((to, from, next) => {
  const token = localStorage.getItem('auth_token')
  if (to.meta.requiresAuth && !token) {
    next({ path: '/login' })
  } else {
    next()
  }
})

export function setupRouter() {
  return router
}
