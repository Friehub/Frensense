// SAFE: route-level beforeEnter guard protects individual routes

import { createRouter, createWebHistory, type RouteRecordRaw } from 'vue-router'
import Dashboard from './views/Dashboard.vue'

function authGuard(to: any, from: any, next: any) {
  const token = localStorage.getItem('auth_token')
  if (!token) {
    next({ path: '/login' })
  } else {
    next()
  }
}

const routes: RouteRecordRaw[] = [
  { path: '/login', component: () => import('./views/Login.vue') },
  { path: '/dashboard', component: Dashboard, beforeEnter: authGuard },
  { path: '/settings', component: () => import('./views/Settings.vue'), beforeEnter: authGuard }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export function setupRouter() {
  return router
}
