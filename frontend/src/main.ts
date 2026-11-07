import { createApp } from 'vue'
import './assets/main.css'
import App from './App.vue'

// Створення додатку та тимчасовий запис його в змінну
const app = createApp(App)

// Запуск додатку та відображення його на сторінці
app.mount('#app')