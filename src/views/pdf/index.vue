<template>
  <div class="bg-gray-50 dark:bg-gray-900 p-6">
    <div class="max-w-full mx-auto">
      <div class="mb-8">
        <h1 class="text-3xl font-bold text-gray-900 dark:text-white mb-2">
          PDF Tools
        </h1>
        <p class="text-gray-600">
          Split, compress, and delete pages from your PDF files with ease
        </p>
      </div>

      <div class="space-y-8">
        <UCard>
          <template #header>
            <div class="flex items-center gap-2">
              <UIcon name="i-heroicons-document-text" class="text-xl" />
              <h2 class="text-lg font-semibold">
                {{ selectedAction === 'merge' ? 'Select PDF Files (Max 2)' : 'Select PDF File' }}
              </h2>
            </div>
          </template>

          <div
            class="border-2 border-dashed rounded-lg p-8 text-center border-gray-300 dark:border-gray-600 hover:border-gray-400 dark:hover:border-gray-500 transition-colors">
            <UIcon name="i-heroicons-cloud-arrow-up" class="text-4xl text-gray-400 dark:text-gray-500 mb-4 mx-auto" />
            <p class="text-lg font-medium text-gray-700 dark:text-gray-300 mb-2">
              {{ selectedAction === 'merge' ? 'Select PDF files' : 'Select PDF file' }}
            </p>
            <p class="text-sm text-gray-500 dark:text-gray-400 mb-4">
              {{ selectedAction === 'merge' ? 'Click to add PDF files (max 2)' : 'Click to browse for PDF file' }}
            </p>
            <UButton color="primary" variant="solid" size="lg" @click="triggerFileInput" :loading="isProcessing">
              <UIcon name="i-heroicons-folder-open" class="mr-2" />
              {{ selectedAction === 'merge' ? 'Add PDF File' : 'Browse File' }}
            </UButton>
          </div>

          <div v-if="selectedFiles.length > 0" class="mt-6">
            <div class="flex items-center justify-between mb-3">
              <h3 class="text-sm font-medium text-gray-700 dark:text-gray-300">
                {{ selectedAction === 'merge' ? `Selected Files (${selectedFiles.length}/2)` : 'Selected File' }}
              </h3>
              <UButton 
                v-if="selectedAction === 'merge' && selectedFiles.length < 2"
                color="primary" 
                variant="outline" 
                size="sm" 
                @click="triggerFileInput"
              >
                <UIcon name="i-heroicons-plus" class="mr-1" />
                Add File
              </UButton>
            </div>
            <div class="space-y-2">
              <div v-for="(file, index) in selectedFiles" :key="index"
                class="flex items-center gap-3 p-3 bg-gray-50 dark:bg-gray-800 rounded-lg">
                <!-- Order indicator for merge -->
                <div v-if="selectedAction === 'merge'" class="flex items-center justify-center w-8 h-8 bg-primary-100 dark:bg-primary-900 text-primary-700 dark:text-primary-300 rounded-full text-sm font-medium flex-shrink-0">
                  {{ index + 1 }}
                </div>
                
                <div class="flex items-center gap-2 flex-1 min-w-0">
                  <UIcon name="i-heroicons-document-text" class="text-red-500 flex-shrink-0" />
                  <span class="text-sm font-medium text-gray-700 dark:text-gray-300 truncate">
                    {{ file.name }}
                  </span>
                  <span class="text-xs text-gray-500 flex-shrink-0">
                    ({{ formatFileSize(file.size) }})
                  </span>
                  <span v-if="file.pageCount && file.pageCount > 0" class="text-xs text-blue-600 dark:text-blue-400 flex-shrink-0">
                    {{ file.pageCount }} pages
                  </span>
                  <span v-else-if="selectedAction !== 'merge' && pdfPageCount > 0" class="text-xs text-blue-600 dark:text-blue-400 flex-shrink-0">
                    {{ pdfPageCount }} pages
                  </span>
                </div>
                <UButton color="error" variant="ghost" size="sm" @click="removeFile(index)">
                  <UIcon name="i-heroicons-x-mark" />
                </UButton>
              </div>
            </div>
            
            <!-- Merge preview -->
            <div v-if="selectedAction === 'merge' && selectedFiles.length > 1" class="mt-4 p-4 bg-blue-50 dark:bg-blue-900/20 rounded-lg border border-blue-200 dark:border-blue-800">
              <div class="flex items-center gap-2 mb-2">
                <UIcon name="i-heroicons-information-circle" class="text-blue-500" />
                <span class="text-sm font-medium text-blue-700 dark:text-blue-300">Merge Preview</span>
              </div>
              <p class="text-sm text-blue-600 dark:text-blue-400">
                Files will be merged in order: 
                <strong>{{ selectedFiles.map(f => f.name).join(' → ') }}</strong>
              </p>
              <p class="text-xs text-blue-500 dark:text-blue-500 mt-1">
                Total pages: {{ selectedFiles.reduce((sum, file) => sum + (file.pageCount || 0), 0) }}
              </p>
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
              <UButton color="primary" variant="solid" size="lg" block :loading="isProcessing"
                @click="openSettingsDrawer">
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
    }" :title="`${getActionLabel(selectedAction)} Settings`">
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
        <PDFSettings 
          :selected-action="selectedAction" 
          v-model:settings="pdfSettings" 
          :pdf-page-count="pdfPageCount"
          :selected-files="selectedFiles"
          @validate="handleSettingsValidation" 
          @file-reorder="handleFileReorder"
        />
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
import PDFSettings from '../../components/pdf/PDFSettings.vue'
import type { PDFSettings as PDFSettingsType } from '../../components/pdf/types'

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
const selectedFiles = ref<{ path: string; name: string; size?: number; pageCount?: number }[]>([])
const selectedAction = ref<'split' | 'compress' | 'delete' | 'merge'>('split')
const progressValue = ref(0)
const progressText = ref('')
const processResults = ref<Array<{ filename: string; path: string }>>([])
const isSettingsOpen = ref(false)
const pdfPageCount = ref(0)
const settingsValidation = ref({ isValid: false, errors: [] as string[] })

const pdfSettings = ref<PDFSettingsType>({
  splitOptions: {
    method: 'pages',
    pageRange: '',
    interval: 5
  },
  compressionLevel: 'medium',
  deletePages: '',
  mergeOptions: {
    files: []
  },
  outputPath: ''
})

const actionOptions = [
  {
    value: 'split',
    label: 'Split PDF',
    description: 'Split into multiple files',
    icon: 'i-heroicons-scissors'
  },
  {
    value: 'delete',
    label: 'Delete PDF',
    description: 'Remove unwanted pages',
    icon: 'i-heroicons-trash'
  },
  {
    value: 'merge',
    label: 'Merge PDF',
    description: 'Combine multiple PDFs',
    icon: 'i-heroicons-document-duplicate'
  }
  // {
  //   value: 'compress',
  //   label: 'Compress PDF',
  //   description: 'Reduce file size',
  //   icon: 'i-heroicons-archive-box-arrow-down'
  // }
]

const canProcess = computed(() => {
  // For merge action, require at least 2 files in main selection
  if (selectedAction.value === 'merge') {
    return selectedFiles.value.length >= 2 &&
      selectedAction.value &&
      !isProcessing.value
  }
  
  // For other actions, require selected files and output path
  return selectedFiles.value.length > 0 &&
    pdfSettings.value.outputPath &&
    selectedAction.value &&
    !isProcessing.value &&
    settingsValidation.value.isValid
})

// Initialize default output path on component mount
onMounted(async () => {
  try {
    const defaultPath = await getDefaultOutputPath();
    pdfSettings.value.outputPath = defaultPath;
  } catch (error) {
    console.error('Error setting default output path:', error);
  }
})

const handleActionSelect = (action: string) => {
  selectedAction.value = action as 'split' | 'compress' | 'delete' | 'merge'
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

const handleSettingsValidation = (isValid: boolean, errors: string[]) => {
  settingsValidation.value = { isValid, errors }
}

const handleFileReorder = (files: { path: string; name: string; size?: number; pageCount?: number }[]) => {
  selectedFiles.value = files
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
      // Check for duplicates
      if (selectedFiles.value.some(file => file.path === selected)) {
        toast.add({
          title: 'Duplicate File',
          description: 'This file is already selected',
          duration: 3000,
          color: 'warning',
          ui: {
            root: 'bg-white'
          }
        })
        return
      }

      // For merge, check file limit
      if (selectedAction.value === 'merge' && selectedFiles.value.length >= 2) {
        toast.add({
          title: 'File Limit',
          description: 'Maximum 2 files allowed for merging',
          duration: 3000,
          color: 'warning',
          ui: {
            root: 'bg-white'
          }
        })
        return
      }

      // Get PDF page count
      let pageCount = 0
      try {
        pageCount = await invoke('get_pdf_page_count', { filePath: selected }) as number
      } catch (error) {
        console.error('Error getting page count:', error)
      }

      const fileObject = {
        path: selected,
        name: selected.split('/').pop() || selected.split('\\').pop() || 'Unknown',
        size: undefined,
        pageCount: pageCount
      }

      if (selectedAction.value === 'merge') {
        // For merge, add to existing files
        selectedFiles.value.push(fileObject)
      } else {
        // For other actions, replace existing selection
        selectedFiles.value = [fileObject]
        pdfPageCount.value = pageCount
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

// Validation function for page ranges (moved from processing)
const validatePageRange = (pageRange: string, totalPages: number): { isValid: boolean; error?: string } => {
  if (!pageRange.trim()) {
    return { isValid: false, error: 'Page range cannot be empty' }
  }

  try {
    const ranges = pageRange.split(',').map((r: string) => r.trim()).filter((r: string) => r.length > 0)

    for (const range of ranges) {
      if (range.includes('-')) {
        const [startStr, endStr] = range.split('-').map((s: string) => s.trim())
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

const getActionIcon = (action: string): string => {
  const iconMap: Record<string, string> = {
    split: 'i-heroicons-scissors',
    compress: 'i-heroicons-archive-box-arrow-down',
    delete: 'i-heroicons-trash',
    merge: 'i-heroicons-document-duplicate'
  }
  return iconMap[action] || 'i-heroicons-cog-6-tooth'
}

const getActionButtonText = (action: string): string => {
  const textMap: Record<string, string> = {
    split: 'Split PDF File',
    compress: 'Compress PDF File',
    delete: 'Delete PDF Pages',
    merge: 'Merge PDF Files'
  }
  return textMap[action] || 'Process File'
}

const processPDF = async () => {
  if (!canProcess.value) return

  // Skip page count validation for merge action since it handles its own files
  if (selectedAction.value !== 'merge' && pdfPageCount.value === 0) {
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

  isSettingsOpen.value = false
  isProcessing.value = true

  try {
    if (selectedAction.value === 'split') {
      let pages: string[] = []

      if (pdfSettings.value.splitOptions.method === 'pages') {
        const validation = validatePageRange(pdfSettings.value.splitOptions.pageRange, pdfPageCount.value)
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

        pages = pdfSettings.value.splitOptions.pageRange.split(',').map((r: string) => r.trim())
      } else {
        const end = pdfSettings.value.splitOptions.method === 'interval' ? pdfSettings.value.splitOptions.interval : pdfPageCount.value
        for (let i = 1; i <= end; i++) {
          pages.push(i.toString())
        }
      }

      const result = await invoke('do_split', {
        filePath: selectedFiles.value[0].path,
        outputPath: pdfSettings.value.outputPath,
        splitOptions: pages
      }) as string

      toast.add({
        title: 'Success',
        description: result || 'PDF split completed successfully!',
        duration: 3000,
        color: 'success',
        ui: {
          root: 'bg-white'
        }
      })

      selectedFiles.value = []

    } else if (selectedAction.value === 'compress') {
      toast.add({
        title: 'Info',
        description: 'Compress functionality coming soon!',
        duration: 3000,
        color: 'info',
        ui: {
          root: 'bg-white'
        }
      })
    } else if (selectedAction.value === 'merge') {
      // Merge processing
      if (selectedFiles.value.length < 2) {
        toast.add({
          title: 'Validation Error',
          description: 'Please select at least 2 PDF files to merge',
          duration: 4000,
          color: 'error',
          ui: {
            root: 'bg-white'
          }
        })
        return
      }

      // Create file paths array in order
      const filePaths = selectedFiles.value.map(file => file.path)
      
      // Get default output path for merged file
      const defaultPath = await getDefaultOutputPath()
      const outputFileName = `merged_${Date.now()}.pdf`
      const outputPath = defaultPath.includes('\\') ? `${defaultPath}\\${outputFileName}` : `${defaultPath}/${outputFileName}`

      try {
        const result = await invoke('do_merge_pdfs', {
          filePaths: filePaths,
          outputPath: outputPath
        }) as string

        toast.add({
          title: 'Success',
          description: result || 'PDFs merged successfully!',
          duration: 3000,
          color: 'success',
          ui: {
            root: 'bg-white'
          }
        })

        // Clear selected files
        selectedFiles.value = []
      } catch (mergeError) {
        console.error('Error merging PDFs:', mergeError)
        toast.add({
          title: 'Error',
          description: 'Failed to merge PDFs. Please try again.',
          duration: 4000,
          color: 'error',
          ui: {
            root: 'bg-white'
          }
        })
        return
      }

    } else if (selectedAction.value === 'delete') {

      const validation = validatePageRange(pdfSettings.value.deletePages, pdfPageCount.value)
      if (!validation.isValid) {
        toast.add({
          title: 'Validation Error',
          description: validation.error || 'Invalid pages to delete',
          duration: 4000,
          color: 'error',
          ui: {
            root: 'bg-white'
          }
        })
        return
      }

      const result = await invoke('do_delete_pages', {
        filePath: selectedFiles.value[0].path,
        outputPath: pdfSettings.value.outputPath,
        selectedPages: pdfSettings.value.deletePages.split(',').map(p => parseInt(p.trim()))
      }) as string

      toast.add({
        title: 'Success',
        description: result || 'Pages deleted successfully!',
        duration: 3000,
        color: 'success',
        ui: {
          root: 'bg-white'
        }
      })

      selectedFiles.value = []
    }
  } catch (error) {
    console.error('Error processing PDF:', error)

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