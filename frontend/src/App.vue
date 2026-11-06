<template>
  <main class="min-h-screen bg-gray-100 p-8 font-sans text-gray-900">
    
    <div class="max-w-7xl mx-auto grid grid-cols-1 md:grid-cols-3 gap-6">
      
      <div class="bg-white p-6 border border-gray-300 shadow-sm h-fit">
        <h1 class="text-xl font-bold mb-2">Аналіз відгуків</h1>
        <p class="text-gray-600 mb-6 text-sm">Завантажте CSV файл для аналізу</p>

        <div class="mb-4">
          <input 
            type="file" 
            accept=".csv" 
            @change="handleFileChange"
            :disabled="isLoading"
            class="w-full border border-gray-300 p-2 bg-gray-50 cursor-pointer text-sm"
          />
        </div>

        <button 
          @click="uploadFile" 
          :disabled="!selectedFile || isLoading"
          class="w-full bg-blue-600 text-white font-bold py-2 px-4 border border-blue-700 disabled:opacity-50 hover:bg-blue-700 transition"
        >
          {{ isLoading ? 'Обробка...' : 'Аналізувати дані' }}
        </button>

        <div 
          v-if="statusMessage" 
          class="mt-4 p-3 border text-sm text-left break-words"
          :class="statusMessage.includes('Помилка') ? 'bg-red-50 border-red-200 text-red-700' : 'bg-blue-50 border-blue-200 text-blue-700'"
        >
          {{ statusMessage }}
        </div>
      </div>

      <!-- Графік тональності -->
      <div v-if="analysisResults.length > 0" class="bg-white p-6 border border-gray-300 shadow-sm">
        <h2 class="text-lg font-bold mb-4">Статистика тональності</h2>
        <div class="h-64">
          <Bar :data="sentimentChartData" :options="chartOptions" />
        </div>
      </div>

      <!-- Графік категорій (Zero-Shot) -->
      <div v-if="analysisResults.length > 0" class="bg-white p-6 border border-gray-300 shadow-sm">
        <h2 class="text-lg font-bold mb-4">Популярні теми (Категорії)</h2>
        <div class="h-64">
          <Bar :data="categoryChartData" :options="chartOptions" />
        </div>
      </div>

    </div>

    <!-- Таблиця з результатами -->
    <div v-if="analysisResults.length > 0" class="max-w-7xl mx-auto mt-6 bg-white border border-gray-300 shadow-sm overflow-hidden">
      <div class="p-4 border-b bg-gray-50">
        <h2 class="text-lg font-bold">Результати класифікації</h2>
      </div>
      <div class="overflow-x-auto">
        <table class="w-full text-sm text-left text-gray-600">
          <thead class="text-xs text-gray-700 uppercase bg-gray-100">
            <tr>
              <!-- Розділені заголовки -->
              <th class="px-4 py-3 w-16">ID</th>
              <th class="px-4 py-3 w-20">Рейтинг</th>
              <th class="px-4 py-3 w-1/2">Текст відгуку</th>
              <th class="px-4 py-3">Категорія (Zero-Shot)</th>
              <th class="px-4 py-3">Тональність (VADER)</th>
            </tr>
          </thead>
          <tbody>
            <tr v-for="(row, index) in analysisResults" :key="index" class="border-b hover:bg-gray-50">
              <!-- Розділені комірки з даними -->
              <td class="px-4 py-3 font-medium text-gray-900">
                #{{ row.user_id || index + 1 }}
              </td>
              <td class="px-4 py-3 font-semibold text-gray-700">
                {{ row.rating || 'N/A' }}
              </td>
              <td class="px-4 py-3 italic">"{{ row.text }}"</td>
              <td class="px-4 py-3 font-semibold">{{ row.customer_segment }}</td>
              <td class="px-4 py-3">
                <span 
                  class="px-2 py-1 text-xs font-bold rounded"
                  :class="{
                    'bg-green-100 text-green-700': row.sentiment === 'Позитивний',
                    'bg-red-100 text-red-700': row.sentiment === 'Негативний',
                    'bg-yellow-100 text-yellow-700': row.sentiment === 'Нейтральний'
                  }"
                >
                  {{ row.sentiment }}
                </span>
              </td>
            </tr>
          </tbody>
        </table>
      </div>
    </div>
    
  </main>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { Bar } from 'vue-chartjs'
import { Chart as ChartJS, Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale } from 'chart.js'

ChartJS.register(Title, Tooltip, Legend, BarElement, CategoryScale, LinearScale)

const selectedFile = ref<File | null>(null)
const statusMessage = ref<string>('')
const isLoading = ref<boolean>(false)
const analysisResults = ref<any[]>([])

const handleFileChange = (event: Event) => {
  const target = event.target as HTMLInputElement
  if (target.files && target.files.length > 0) {
    selectedFile.value = target.files[0]
    statusMessage.value = '' 
  } else {
    selectedFile.value = null
  }
}

const uploadFile = async () => {
  if (!selectedFile.value) return

  isLoading.value = true
  statusMessage.value = 'Аналізування'
  analysisResults.value = []

  const formData = new FormData()
  formData.append('file', selectedFile.value)

  try {
    const response = await fetch('http://localhost:3000/upload', {
      method: 'POST',
      body: formData,
    })

    if (response.ok) {
      const result = await response.json()
      analysisResults.value = result.data
      statusMessage.value = `Успішно оброблено рядків: ${result.processed_rows}`
    } else {
      statusMessage.value = `Помилка сервера: ${response.status}`
    }
  } catch (error) {
    console.error(error)
    statusMessage.value = 'Не вдалося з\'єднатися з сервером'
  } finally {
    isLoading.value = false
  }
}

// Графік тональності
const sentimentChartData = computed(() => {
  let pos = 0, neg = 0, neu = 0;
  
  analysisResults.value.forEach(row => {
    if (row.sentiment === 'Позитивний') pos++;
    else if (row.sentiment === 'Негативний') neg++;
    else neu++;
  });

  return {
    labels: ['Позитивний', 'Нейтральний', 'Негативний'],
    datasets: [{
      label: 'Кількість',
      data: [pos, neu, neg],
      backgroundColor: ['#10B981', '#F59E0B', '#EF4444'],
    }]
  }
})

// Графік категорій
const categoryChartData = computed(() => {
  const counts: Record<string, number> = {};
  
  analysisResults.value.forEach(row => {
    const category = row.customer_segment || 'Невідомо';
    counts[category] = (counts[category] || 0) + 1;
  });

  return {
    labels: Object.keys(counts),
    datasets: [{
      label: 'Кількість відгуків',
      data: Object.values(counts),
      backgroundColor: ['#3B82F6', '#8B5CF6', '#EC4899', '#F97316', '#14B8A6'], 
    }]
  }
})

const chartOptions = {
  responsive: true,
  maintainAspectRatio: false,
  plugins: {
    legend: { display: false }
  }
}
</script>