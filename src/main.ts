import { createApp } from 'vue'
import { createPinia } from 'pinia'
import { createRouter, createWebHistory } from 'vue-router'

import App from './App.vue'
import './index.css'
import i18n from './i18n'

import Dashboard  from './pages/Dashboard.vue'
import Activity   from './pages/Activity.vue'
import Statistics from './pages/Statistics.vue'
import History    from './pages/History.vue'
import Settings   from './pages/Settings.vue'
import Stretches  from './pages/Stretches.vue'

// ─── Router
const router = createRouter({
  history: createWebHistory(),
  routes: [
    { path: '/',           component: Dashboard,  name: 'dashboard'  },
    { path: '/activity',   component: Activity,   name: 'activity'   },
    { path: '/statistics', component: Statistics, name: 'statistics' },
    { path: '/history',    component: History,    name: 'history'    },
    { path: '/stretches',  component: Stretches,  name: 'stretches'  },
    { path: '/settings',   component: Settings,   name: 'settings'   },
  ],
})

// ─── App
const app = createApp(App)
app.use(createPinia())
app.use(router)
app.use(i18n)
app.mount('#app')
