import { createRouter, createMemoryHistory } from 'vue-router'
import { routes } from './route';

const router = createRouter({
  history: createMemoryHistory(),
  routes
})

export default router
