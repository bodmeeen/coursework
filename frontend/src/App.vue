<template>
  <main class="min-h-screen bg-gray-100 p-8 font-sans text-gray-900">
    
    <div class="bg-white p-6 border border-gray-300 w-full max-w-md mx-auto">
      
      <h1 class="text-xl font-bold mb-2">Аналіз відгуків</h1>
      <p class="text-gray-600 mb-6 text-sm">Завантажте CSV файл</p>

      <!-- @change викликає handleFileChange при виборі нового файлу -->
      <div class="mb-4">
        <input 
          type="file" 
          accept=".csv" 
          @change="handleFileChange"
          :disabled="isLoading"
          class="w-full border border-gray-300 p-2 bg-gray-50 cursor-pointer text-sm"
        />
      </div>

      <!-- Кнопка вимкнена якщо файл не обрано або йде завантаження -->
      <button 
        @click="uploadFile" 
        :disabled="!selectedFile || isLoading"
        class="w-full bg-blue-600 text-white font-bold py-2 px-4 border border-blue-700 disabled:opacity-50 disabled:bg-gray-400 disabled:border-gray-500"
      >
        {{ isLoading ? 'Обробка...' : 'Аналізувати дані' }}
      </button>

      <div 
        v-if="statusMessage" 
        class="mt-4 p-3 border text-sm text-left break-words"
        :class="statusMessage.includes('X') ? 'bg-red-50 border-red-200 text-red-700' : 'bg-green-50 border-green-200 text-green-700'"
      >
        {{ statusMessage }}
      </div>
      
    </div>
  </main>
</template>

<script setup lang="ts">
import { ref } from 'vue'

// selectedFile зберігає об'єкт обраного CSV 
// Початкове null поки файл не вибрано
const selectedFile = ref<File | null>(null)

const statusMessage = ref<string>('')

// Для блокування кнопки під час оброблення
const isLoading = ref<boolean>(false)

// Викликається кожен раз коли подається новий файл
const handleFileChange = (event: Event) => {
  // Перетворювання Event на HTMLInputElement щоб мати доступ до його властивостей
  const target = event.target as HTMLInputElement
  
  if (target.files && target.files.length > 0) {
    selectedFile.value = target.files[0]
    statusMessage.value = '' 
  } else {
    selectedFile.value = null
  }
}

// Викликається при натисканні Аналізувати дані
const uploadFile = async () => {
  if (!selectedFile.value) return

  // Вмикається стан завантаження і виводиться повідомлення
  isLoading.value = true
  statusMessage.value = 'Відправка файлу на бекенд...'

  // Формування FormData, це стандартний спосіб браузера відправляти файли через HTTP (multipart/form-data)
  const formData = new FormData()
  formData.append('file', selectedFile.value)

  try {
    const response = await fetch('http://localhost:3000/upload', {
      method: 'POST',
      body: formData,
    })

    if (response.ok) {
      const data = await response.json()
      statusMessage.value = `Відповідь: ${JSON.stringify(data)}`
    } else {
      // 400 або 500
      statusMessage.value = `Помилка сервера: ${response.status}`
    }
  } catch (error) {
    // Якщо взагалі немає відповіді від серверу
    console.error(error)
    statusMessage.value = 'Не вдалося з\'єднатися'
  } finally {
    isLoading.value = false
  }
}
</script>