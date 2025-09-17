import { createRouter, createWebHistory } from 'vue-router'
import Home from '../views/Home.vue'

const routes = [
  {
    path: '/',
    name: 'Home',
    component: Home
  },
  {
    path: '/pdf',
    name: 'pdf-index',
    component: () => import('../views/pdf/index.vue')
  }
]

const router = createRouter({
  history: createWebHistory(),
  routes
})

export default router
