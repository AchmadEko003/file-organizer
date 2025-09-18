<template>
  <div class="min-h-screen bg-gray-50 dark:bg-gray-900 p-6">
    <div class="max-w-full mx-auto">
      <!-- Header -->
      <div class="mb-8">
        <h1 class="text-3xl font-bold text-gray-900 dark:text-white mb-2">
          PDF Tools
        </h1>
        <p class="text-gray-600 dark:text-gray-400">
          Split, compress, and extract your PDF files with ease
        </p>
      </div>

      <!-- Main Content -->
      <div class="grid grid-cols-1 xl:grid-cols-3 gap-8">
        <!-- File Upload Section -->
        <UCard class="h-fit xl:col-span-2">
          <template #header>
            <div class="flex items-center gap-2">
              <UIcon name="i-heroicons-document-text" class="text-xl" />
              <h2 class="text-lg font-semibold">Select PDF File</h2>
            </div>
          </template>

          <div class="border-2 border-dashed rounded-lg p-8 text-center border-gray-300 dark:border-gray-600 hover:border-gray-400 dark:hover:border-gray-500 transition-colors">
            <UIcon 
              name="i-heroicons-cloud-arrow-up" 
              class="text-4xl text-gray-400 dark:text-gray-500 mb-4 mx-auto" 
            />
            <p class="text-lg font-medium text-gray-700 dark:text-gray-300 mb-2">
              Select PDF file
            </p>
            <p class="text-sm text-gray-500 dark:text-gray-400 mb-4">
              Click to browse for PDF file
            </p>
            <UButton
              color="primary"
              variant="solid"
              size="lg"
              @click="triggerFileInput"
              :loading="isProcessing"
            >
              <UIcon name="i-heroicons-folder-open" class="mr-2" />
              Browse File
            </UButton>
          </div>

          <div v-if="selectedFiles.length > 0" class="mt-6">
            <h3 class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-3">
              Selected File
            </h3>
            <div class="space-y-2">
              <div
                v-for="(file, index) in selectedFiles"
                :key="index"
                class="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-800 rounded-lg"
              >
                <div class="flex items-center gap-2 flex-1 min-w-0">
                  <UIcon name="i-heroicons-document-text" class="text-red-500 flex-shrink-0" />
                  <span class="text-sm font-medium text-gray-700 dark:text-gray-300 truncate">
                    {{ file.name }}
                  </span>
                  <span class="text-xs text-gray-500 flex-shrink-0">
                    ({{ formatFileSize(file.size) }})
                  </span>
                </div>
                <UButton
                  color="error"
                  variant="ghost"
                  size="sm"
                  @click="removeFile(index)"
                >
                  <UIcon name="i-heroicons-x-mark" />
                </UButton>
              </div>
            </div>
          </div>
        </UCard>

        <!-- Action & Settings Section -->
        <UCard>
          <template #header>
            <div class="flex items-center gap-2">
              <UIcon name="i-heroicons-cog-6-tooth" class="text-xl" />
              <h2 class="text-lg font-semibold">Action & Settings</h2>
            </div>
          </template>

          <div class="space-y-6">
            <!-- Action Selection -->
            <div>
              <label class="block text-sm font-medium text-gray-700 dark:text-gray-300 mb-3">
                Select Action
              </label>
              <div class="grid grid-cols-2 gap-3">
                <div
                  v-for="option in actionOptions"
                  :key="option.value"
                  @click="selectedAction = option.value"
                  :class="[
                    'p-4 border rounded-lg cursor-pointer transition-all',
                    selectedAction === option.value
                      ? 'border-primary-500 bg-primary-50 dark:bg-primary-950'
                      : 'border-gray-200 dark:border-gray-700 hover:border-gray-300 dark:hover:border-gray-600'
                  ]"
                >
                  <div class="flex items-center gap-3">
                    <UIcon 
                      :name="option.icon" 
                      class="text-xl" 
                      :class="selectedAction === option.value ? 'text-primary-600' : 'text-gray-500'" 
                    />
                    <div>
                      <div 
                        class="font-medium text-sm" 
                        :class="selectedAction === option.value ? 'text-primary-700 dark:text-primary-300' : 'text-gray-700 dark:text-gray-300'"
                      >
                        {{ option.label }}
                      </div>
                      <div class="text-xs text-gray-500 dark:text-gray-400">
                        {{ option.description }}
                      </div>
                    </div>
                  </div>
                </div>
              </div>
            </div>

            <!-- Additional Options for Split -->
            <div v-if="selectedAction === 'split'" class="space-y-4">
              <USeparator label="Split Options" />
              
              <UFormGroup label="Split Method">
                <USelect
                  v-model="splitOptions.method"
                  :items="splitMethods"
                  placeholder="Choose split method"
                />
              </UFormGroup>

              <UFormGroup 
                v-if="splitOptions.method === 'pages'"
                label="Page Range"
                help="e.g., 1-5, 7, 10-15"
              >
                <UInput
                  v-model="splitOptions.pageRange"
                  placeholder="1-5, 7, 10-15"
                />
              </UFormGroup>

              <UFormGroup 
                v-if="splitOptions.method === 'interval'"
                label="Pages per File"
              >
                <UInput
                  v-model="splitOptions.interval"
                  type="number"
                  placeholder="5"
                  min="1"
                />
              </UFormGroup>
            </div>

            <!-- Additional Options for Compress -->
            <div v-if="selectedAction === 'compress'" class="space-y-4">
              <USeparator label="Compression Options" />
              
              <UFormGroup label="Compression Level">
                <USelect
                  v-model="compressionLevel"
                  :options="compressionLevels"
                  placeholder="Choose compression level"
                />
              </UFormGroup>
            </div>

            <!-- Output Path -->
            <div>
              <UFormGroup label="Save Location" help="Choose where to save the processed files">
                <div class="flex gap-2">
                  <UInput
                    v-model="outputPath"
                    placeholder="Select output folder..."
                    readonly
                    class="flex-1"
                  />
                  <UButton
                    color="neutral"
                    variant="outline"
                    @click="selectOutputPath"
                  >
                    <UIcon name="i-heroicons-folder" class="mr-2" />
                    Browse
                  </UButton>
                </div>
              </UFormGroup>
            </div>

            <!-- Process Button -->
            <UButton
              color="primary"
              variant="solid"
              size="lg"
              block
              :disabled="!canProcess"
              :loading="isProcessing"
              @click="processPDF"
            >
              <UIcon :name="getActionIcon(selectedAction)" class="mr-2" />
              {{ getActionButtonText(selectedAction) }}
            </UButton>
          </div>
        </UCard>
      </div>

      <!-- Progress Section -->
      <UCard v-if="isProcessing || processResults.length > 0" class="mt-8">
        <template #header>
          <div class="flex items-center gap-2">
            <UIcon name="i-heroicons-clock" class="text-xl" />
            <h2 class="text-lg font-semibold">Processing Status</h2>
          </div>
        </template>

        <div v-if="isProcessing" class="space-y-4">
          <div class="flex items-center gap-3">
            <UIcon name="i-heroicons-arrow-path" class="text-xl animate-spin text-primary-500" />
            <span class="text-sm font-medium">Processing PDF files...</span>
          </div>
          <UProgress :value="progressValue" :max="100" />
          <p class="text-sm text-gray-600 dark:text-gray-400">
            {{ progressText }}
          </p>
        </div>

        <div v-if="processResults.length > 0 && !isProcessing" class="space-y-3">
          <h3 class="text-sm font-medium text-gray-700 dark:text-gray-300">
            Processing Complete
          </h3>
          <div class="space-y-2">
            <div
              v-for="(result, index) in processResults"
              :key="index"
              class="flex items-center justify-between p-3 bg-green-50 dark:bg-green-950 rounded-lg"
            >
              <div class="flex items-center gap-2">
                <UIcon name="i-heroicons-check-circle" class="text-green-500" />
                <span class="text-sm font-medium text-green-700 dark:text-green-300">
                  {{ result.filename }}
                </span>
              </div>
              <UButton
                color="success"
                variant="ghost"
                size="sm"
                @click="openFile(result.path)"
              >
                <UIcon name="i-heroicons-folder-open" class="mr-1" />
                Open
              </UButton>
            </div>
          </div>
        </div>
      </UCard>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'
import { invoke } from '@tauri-apps/api/core';

const isProcessing = ref(false)
const selectedFiles = ref<{ path: string; name: string; size?: number }[]>([])
const selectedAction = ref('split')
const outputPath = ref('')
const progressValue = ref(0)
const progressText = ref('')
const processResults = ref<Array<{ filename: string; path: string }>>([])

const splitOptions = ref({
  method: 'pages',
  pageRange: '',
  interval: 5
})

const compressionLevel = ref('medium')

const actionOptions = [
  {
    value: 'split',
    label: 'Split PDF',
    description: 'Split into multiple files',
    icon: 'i-heroicons-scissors'
  },
  {
    value: 'compress',
    label: 'Compress PDF',
    description: 'Reduce file size',
    icon: 'i-heroicons-archive-box-arrow-down'
  },
  {
    value: 'extract',
    label: 'Extract Pages',
    description: 'Extract specific pages',
    icon: 'i-heroicons-document-arrow-down'
  }
]

const splitMethods = [
  { value: 'pages', label: 'By Page Range' },
  { value: 'interval', label: 'By Page Interval' },
  { value: 'all', label: 'Split All Pages' }
]

const compressionLevels = ref([
  { value: 'low', label: 'Low (Better Quality)' },
  { value: 'medium', label: 'Medium (Balanced)' },
  { value: 'high', label: 'High (Smaller Size)' }
])

const canProcess = computed(() => {
  return selectedFiles.value.length > 0 && outputPath.value && !isProcessing.value
})

const triggerFileInput = async () => {
  try {
    const selected = await open({
      multiple: false,
      filters: [{
        name: 'PDF Files',
        extensions: ['pdf']
      }]
    })
    
    if (selected && typeof selected === 'string') {
      const fileObject = {
        path: selected,
        name: selected.split('/').pop() || selected.split('\\').pop() || 'Unknown',
        size: undefined
      }
      
      selectedFiles.value = [fileObject]

      console.log('Selected file:', await invoke('get_pdf_page_count', { filePath: fileObject.path }))
    }
  } catch (error) {
    console.error('Error selecting files:', error)
  }
}

const removeFile = (index: number) => {
  selectedFiles.value.splice(index, 1)
}

const formatFileSize = (bytes: number | undefined): string => {
  if (!bytes || bytes === 0) return 'Unknown size'
  const k = 1024
  const sizes = ['Bytes', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
}

const selectOutputPath = async () => {
  try {
    const selected = await open({
      directory: true,
      multiple: false,
      title: 'Select Output Folder'
    })
    
    if (selected && typeof selected === 'string') {
      outputPath.value = selected
    }
  } catch (error) {
    console.error('Error selecting output path:', error)
  }
}

const getActionIcon = (action: string): string => {
  const iconMap: Record<string, string> = {
    split: 'i-heroicons-scissors',
    compress: 'i-heroicons-archive-box-arrow-down',
    extract: 'i-heroicons-document-arrow-down'
  }
  return iconMap[action] || 'i-heroicons-cog-6-tooth'
}

const getActionButtonText = (action: string): string => {
  const textMap: Record<string, string> = {
    split: 'Split PDF File',
    compress: 'Compress PDF File',
    extract: 'Extract PDF Pages'
  }
  return textMap[action] || 'Process File'
}

const processPDF = async () => {
  if (!canProcess.value) return
  isProcessing.value = true
  progressValue.value = 0
  processResults.value = []
  
  try {
    progressText.value = 'Preparing to process files...'
    progressValue.value = 10
    
    for (let i = 0; i < selectedFiles.value.length; i++) {
      const file = selectedFiles.value[i]
      progressText.value = `Processing ${file.name}...`
      progressValue.value = 20 + (i / selectedFiles.value.length) * 70
      
      await new Promise(resolve => setTimeout(resolve, 1000))
      
      processResults.value.push({
        filename: `${selectedAction.value}_${file.name}`,
        path: `${outputPath.value}/${selectedAction.value}_${file.name}`
      })
    }
    
    progressText.value = 'Processing complete!'
    progressValue.value = 100
    
    selectedFiles.value = []
    
  } catch (error) {
    console.error('Error processing PDF:', error)
  } finally {
    isProcessing.value = false
  }
}

const openFile = async (filePath: string) => {
  try {
    const { openPath } = await import('@tauri-apps/plugin-opener')
    await openPath(filePath)
  } catch (error) {
    console.error('Error opening file:', error)
  }
}
</script>