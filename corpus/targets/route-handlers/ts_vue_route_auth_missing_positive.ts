// [frensense]
// observation: Vue Router navigation guard is not present or does not check authentication state
// impact: unauthenticated users can access protected routes and view sensitive data or perform actions
// improvement: add beforeEnter guard or router.beforeEach that redirects unauthenticated users to login
// cwe: CWE-287
// cvss: 9.8
// owasp: A07:2021
// severity: Critical

import { createRouter, createWebHistory, type RouteRecordRaw } from 'vue-router'
import Dashboard from './views/Dashboard.vue'
import Settings from './views/Settings.vue'

const routes: RouteRecordRaw[] = [
  { path: '/login', component: () => import('./views/Login.vue') },
  { path: '/dashboard', component: Dashboard },
  { path: '/settings', component: Settings }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export function setupRouter() {
  return router
}
