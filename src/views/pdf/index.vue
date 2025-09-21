<template>
  <div class="min-h-screen bg-gray-50 dark:bg-gray-900 p-6">
    <div class="max-w-full mx-auto">
      <div class="mb-8">
        <h1 class="text-3xl font-bold text-gray-900 dark:text-white mb-2">
          PDF Tools
        </h1>
        <p class="text-gray-600 dark:text-gray-400">
          Split, compress, and extract your PDF files with ease
        </p>
      </div>

      <div class="space-y-8">
        <UCard>
          <template #header>
            <div class="flex items-center gap-2">
              <UIcon name="i-heroicons-document-text" class="text-xl" />
              <h2 class="text-lg font-semibold">Select PDF File</h2>
            </div>
          </template>

          <div
            class="border-2 border-dashed rounded-lg p-8 text-center border-gray-300 dark:border-gray-600 hover:border-gray-400 dark:hover:border-gray-500 transition-colors">
            <UIcon name="i-heroicons-cloud-arrow-up" class="text-4xl text-gray-400 dark:text-gray-500 mb-4 mx-auto" />
            <p class="text-lg font-medium text-gray-700 dark:text-gray-300 mb-2">
              Select PDF file
            </p>
            <p class="text-sm text-gray-500 dark:text-gray-400 mb-4">
              Click to browse for PDF file
            </p>
            <UButton color="primary" variant="solid" size="lg" @click="triggerFileInput" :loading="isProcessing">
              <UIcon name="i-heroicons-folder-open" class="mr-2" />
              Browse File
            </UButton>
          </div>

          <div v-if="selectedFiles.length > 0" class="mt-6">
            <h3 class="text-sm font-medium text-gray-700 dark:text-gray-300 mb-3">
              Selected File
            </h3>
            <div class="space-y-2">
              <div v-for="(file, index) in selectedFiles" :key="index"
                class="flex items-center justify-between p-3 bg-gray-50 dark:bg-gray-800 rounded-lg">
                <div class="flex items-center gap-2 flex-1 min-w-0">
                  <UIcon name="i-heroicons-document-text" class="text-red-500 flex-shrink-0" />
                  <span class="text-sm font-medium text-gray-700 dark:text-gray-300 truncate">
                    {{ file.name }}
                  </span>
                  <span class="text-xs text-gray-500 flex-shrink-0">
                    ({{ formatFileSize(file.size) }})
                  </span>
                  <span v-if="pdfPageCount > 0" class="text-xs text-blue-600 dark:text-blue-400 flex-shrink-0">
                    {{ pdfPageCount }} pages
                  </span>
                </div>
                <UButton color="error" variant="ghost" size="sm" @click="removeFile(index)">
                  <UIcon name="i-heroicons-x-mark" />
                </UButton>
              </div>
            </div>
          </div>
        </UCard>

        <UCard>
          <template #header>
            <div class="flex items-center gap-2">
              <UIcon name="i-heroicons-bolt" class="text-xl" />
              <h2 class="text-lg font-semibold">Actions</h2>
            </div>
          </template>

          <div class="space-y-4">
            <p class="text-sm text-gray-600 dark:text-gray-400">
              Choose an action to perform on your PDF file
            </p>

            <div class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-4">
              <UButton v-for="option in actionOptions" :key="option.value"
                :variant="selectedAction === option.value ? 'solid' : 'outline'"
                :color="selectedAction === option.value ? 'primary' : 'neutral'" size="lg"
                class="h-auto p-4 flex-col gap-2 min-h-[100px]" @click="handleActionSelect(option.value)">
                <UIcon :name="option.icon" class="text-2xl" />
                <div class="text-center">
                  <div class="font-medium text-sm">
                    {{ option.label }}
                  </div>
                  <div class="text-xs opacity-75">
                    {{ option.description }}
                  </div>
                </div>
              </UButton>
            </div>

            <div class="pt-4 border-t border-gray-200 dark:border-gray-700">
              <UButton v-if="selectedAction" color="primary" variant="solid" size="lg" block :disabled="!canProcess"
                :loading="isProcessing" @click="openSettingsDrawer">
                <UIcon :name="getActionIcon(selectedAction)" class="mr-2" />
                Process {{ getActionLabel(selectedAction) }}
              </UButton>
            </div>
          </div>
        </UCard>
      </div>

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
            <div v-for="(result, index) in processResults" :key="index"
              class="flex items-center justify-between p-3 bg-green-50 dark:bg-green-950 rounded-lg">
              <div class="flex items-center gap-2">
                <UIcon name="i-heroicons-check-circle" class="text-green-500" />
                <span class="text-sm font-medium text-green-700 dark:text-green-300">
                  {{ result.filename }}
                </span>
              </div>
              <UButton color="success" variant="ghost" size="sm" @click="openFile(result.path)">
                <UIcon name="i-heroicons-folder-open" class="mr-1" />
                Open
              </UButton>
            </div>
          </div>
        </div>
      </UCard>
    </div>

    <UDrawer v-model:open="isSettingsOpen" direction="right" :ui="{
      overlay: 'bg-black/20',
    }"
      :title="`${getActionLabel(selectedAction)} Settings`">
      <div style="display: none;"></div>

      <template #header>
        <div class="flex items-center justify-between">
          <h3 class="text-lg font-semibold text-gray-900 dark:text-white">
            {{ getActionLabel(selectedAction) }} Settings
          </h3>
          <UButton color="neutral" variant="ghost" icon="i-heroicons-x-mark" @click="isSettingsOpen = false" />
        </div>
      </template>

      <template #body>
        <div class="space-y-6">
          <div v-if="selectedAction === 'split'" class="flex gap-2">
            <USelect v-model="splitOptions.method" :items="splitMethods" placeholder="Choose split method" />

            <UInput v-if="splitOptions.method === 'pages'" v-model="splitOptions.pageRange"
              :placeholder="pdfPageCount > 0 ? `1-${pdfPageCount}, 7, 10-15` : '1-5, 7, 10-15'" />

            <UInput v-if="splitOptions.method === 'interval'" v-model="splitOptions.interval" type="number"
              placeholder="5" min="1" />
          </div>

          <div v-if="selectedAction === 'compress'" class="space-y-4">
            <USelect v-model="compressionLevel" :options="compressionLevels" placeholder="Choose compression level" />
          </div>

          <div v-if="selectedAction === 'extract'" class="space-y-4">
            <UFormGroup label="Pages to Extract" help="e.g., 1-3, 5, 8-10">
              <UInput v-model="extractOptions.pageRange" 
                :placeholder="pdfPageCount > 0 ? `1-${pdfPageCount}, 5, 8-10` : '1-3, 5, 8-10'" />
            </UFormGroup>
          </div>

          <div class="space-y-4">
            <USeparator label="Output Settings" />

            <UFormGroup label="Save Location" help="Choose where to save the processed files">
              <div class="flex gap-2">
                <UInput v-model="outputPath" placeholder="Select output folder..." readonly class="flex-1" />
                <UButton color="neutral" variant="outline" @click="selectOutputPath">
                  <UIcon name="i-heroicons-folder" class="mr-2" />
                  Browse
                </UButton>
              </div>
            </UFormGroup>
          </div>
        </div>
      </template>

      <template #footer>
        <div class="flex gap-3">
          <UButton color="neutral" variant="outline" block @click="isSettingsOpen = false">
            Cancel
          </UButton>
          <UButton color="primary" variant="solid" block :disabled="!canProcess" :loading="isProcessing"
            @click="processPDF">
            <UIcon :name="getActionIcon(selectedAction)" class="mr-2" />
            {{ getActionButtonText(selectedAction) }}
          </UButton>
        </div>
      </template>
    </UDrawer>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'
import { invoke } from '@tauri-apps/api/core';
import { documentDir } from '@tauri-apps/api/path';

const toast = useToast();

// Function to get default output path based on OS
const getDefaultOutputPath = async (): Promise<string> => {
  try {
    const documentPath = await documentDir();
    
    const isWindows = documentPath.includes('\\') || documentPath.includes('C:');
    
    let defaultPath: string;
    
    if (isWindows) {
      defaultPath = `${documentPath}\\docutools`;
    } else {
      defaultPath = `${documentPath}/docutools`;
    }
    
    return defaultPath;
  } catch (error) {
    console.error('Error getting default output path:', error);
    try {
      const documentPath = await documentDir();
      return `${documentPath}/docutools`;
    } catch {
      return 'Documents/docutools';
    }
  }
};

const isProcessing = ref(false)
const selectedFiles = ref<{ path: string; name: string; size?: number }[]>([])
const selectedAction = ref('split')
const outputPath = ref('')
const progressValue = ref(0)
const progressText = ref('')
const processResults = ref<Array<{ filename: string; path: string }>>([])
const isSettingsOpen = ref(false)
const pdfPageCount = ref(0)

const splitOptions = ref({
  method: 'pages',
  pageRange: '',
  interval: 5
})

const compressionLevel = ref('medium')

const extractOptions = ref({
  pageRange: ''
})

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
  { value: 'pages', label: 'By Range' },
  { value: 'interval', label: 'By Interval' },
  { value: 'all', label: 'Split All' }
]

const compressionLevels = ref([
  { value: 'low', label: 'Low (Better Quality)' },
  { value: 'medium', label: 'Medium (Balanced)' },
  { value: 'high', label: 'High (Smaller Size)' }
])

const canProcess = computed(() => {
  return selectedFiles.value.length > 0 && outputPath.value && selectedAction.value && !isProcessing.value
})

// Initialize default output path on component mount
onMounted(async () => {
  try {
    const defaultPath = await getDefaultOutputPath();
    outputPath.value = defaultPath;
  } catch (error) {
    console.error('Error setting default output path:', error);
  }
})

const handleActionSelect = (action: string) => {
  selectedAction.value = action
}

const openSettingsDrawer = () => {
  if (selectedAction.value) {
    isSettingsOpen.value = true
  }
}

const getActionLabel = (action: string): string => {
  const option = actionOptions.find(opt => opt.value === action)
  return option ? option.label : 'Action'
}

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

      // Get and store PDF page count for validation
      try {
        const pageCount = await invoke('get_pdf_page_count', { filePath: fileObject.path }) as number
        pdfPageCount.value = pageCount
        console.log('Selected file page count:', pageCount)
      } catch (error) {
        console.error('Error getting page count:', error)
        pdfPageCount.value = 0
      }
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

// Validation function for page ranges
const validatePageRange = (pageRange: string, totalPages: number): { isValid: boolean; error?: string } => {
  if (!pageRange.trim()) {
    return { isValid: false, error: 'Page range cannot be empty' }
  }

  try {
    const ranges = pageRange.split(',').map(r => r.trim()).filter(r => r.length > 0)
    
    for (const range of ranges) {
      if (range.includes('-')) {
        // Handle range like "1-5"
        const [startStr, endStr] = range.split('-').map(s => s.trim())
        const start = parseInt(startStr)
        const end = parseInt(endStr)
        
        if (isNaN(start) || isNaN(end)) {
          return { isValid: false, error: `Invalid page range format: "${range}"` }
        }
        
        if (start < 1) {
          return { isValid: false, error: `Page numbers must start from 1. Found: ${start}` }
        }
        
        if (end > totalPages) {
          return { isValid: false, error: `Page ${end} exceeds total pages (${totalPages})` }
        }
        
        if (start > end) {
          return { isValid: false, error: `Invalid range "${range}": start page must be less than or equal to end page` }
        }
      } else {
        // Handle single page like "7"
        const page = parseInt(range)
        
        if (isNaN(page)) {
          return { isValid: false, error: `Invalid page number: "${range}"` }
        }
        
        if (page < 1) {
          return { isValid: false, error: `Page numbers must start from 1. Found: ${page}` }
        }
        
        if (page > totalPages) {
          return { isValid: false, error: `Page ${page} exceeds total pages (${totalPages})` }
        }
      }
    }
    
    return { isValid: true }
  } catch (error) {
    return { isValid: false, error: 'Invalid page range format' }
  }
}

const processPDF = async () => {
  if (!canProcess.value) return

  // Validation for split action
  if (selectedAction.value === 'split' && splitOptions.value.method === 'pages') {
    if (pdfPageCount.value === 0) {
      toast.add({
        title: 'Error',
        description: 'Unable to determine PDF page count. Please try selecting the file again.',
        duration: 3000,
        color: 'error',
        ui: {
          root: 'bg-white'
        }
      })
      return
    }

    const validation = validatePageRange(splitOptions.value.pageRange, pdfPageCount.value)
    if (!validation.isValid) {
      toast.add({
        title: 'Validation Error',
        description: validation.error || 'Invalid page range',
        duration: 4000,
        color: 'error',
        ui: {
          root: 'bg-white'
        }
      })
      return
    }
  }

  // Validation for extract action
  if (selectedAction.value === 'extract') {
    if (pdfPageCount.value === 0) {
      toast.add({
        title: 'Error',
        description: 'Unable to determine PDF page count. Please try selecting the file again.',
        duration: 3000,
        color: 'error',
        ui: {
          root: 'bg-white'
        }
      })
      return
    }

    const validation = validatePageRange(extractOptions.value.pageRange, pdfPageCount.value)
    if (!validation.isValid) {
      toast.add({
        title: 'Validation Error',
        description: validation.error || 'Invalid page range',
        duration: 4000,
        color: 'error',
        ui: {
          root: 'bg-white'
        }
      })
      return
    }
  }

  isSettingsOpen.value = false
  isProcessing.value = true

  try {
    if (selectedAction.value === 'split') {
      const result = await invoke('do_split', {
        filePath: selectedFiles.value[0].path,
        outputPath: outputPath.value,
        splitOptions: splitOptions.value.pageRange.split(',').map(r => r.trim())
      }) as string

      console.log('Split result:', result)

      // Show success toast with result message
      toast.add({
        title: 'Success',
        description: result || 'PDF split completed successfully!',
        duration: 3000,
        color: 'success',
        ui: {
          root: 'bg-white'
        }
      })

      // Reset form after successful processing
      selectedFiles.value = []
      selectedAction.value = ''
      
    } else if (selectedAction.value === 'compress') {
      // TODO: Implement compress functionality
      toast.add({
        title: 'Info',
        description: 'Compress functionality coming soon!',
        duration: 3000,
        color: 'info',
        ui: {
          root: 'bg-white'
        }
      })
      
    } else if (selectedAction.value === 'extract') {
      // TODO: Implement extract functionality
      toast.add({
        title: 'Info',
        description: 'Extract functionality coming soon!',
        duration: 3000,
        color: 'info',
        ui: {
          root: 'bg-white'
        }
      })
    }

  } catch (error) {
    console.error('Error processing PDF:', error)
    
    // Show error toast
    toast.add({
      title: 'Error',
      description: 'Failed to process PDF. Please try again.',
      duration: 3000,
      color: 'error',
      ui: {
        root: 'bg-white'
      }
    })
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