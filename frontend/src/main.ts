import { createApp } from 'vue'
import './assets/main.css'
import App from './App.vue'

// Імпортування налаштувань роутера
// import router from './router'


// Створення додатку та тимчасовий запис його в змінну
const app = createApp(App)

// Об'єднання додатка з роутером
// app.use(router)

// Запуск додатку та відображення його на сторінці
app.mount('#app')