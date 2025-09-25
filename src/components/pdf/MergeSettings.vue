<template>
  <div class="space-y-6">
    <div>
      <h3 class="text-lg font-medium text-gray-900 dark:text-white mb-4">
        PDF Merge Settings
      </h3>
      <p class="text-sm text-gray-600 dark:text-gray-400 mb-4">
        Arrange the order of your selected PDF files for merging.
      </p>
    </div>

    <!-- File Reordering -->
    <div class="space-y-4">
      <div class="flex items-center justify-between">
        <h4 class="text-md font-medium text-gray-800 dark:text-gray-200">
          File Order ({{ mergeOptions.files.length }} files)
        </h4>
      </div>

      <!-- File List with Ordering -->
      <div v-if="mergeOptions.files.length > 0" class="space-y-3">
        <div 
          v-for="(file, index) in mergeOptions.files" 
          :key="file.id"
          class="flex items-center gap-3 p-4 bg-gray-50 dark:bg-gray-800 rounded-lg border border-gray-200 dark:border-gray-700"
        >
          <!-- Order controls -->
          <div class="flex flex-col gap-1">
            <UButton
              v-if="index > 0"
              color="neutral"
              variant="ghost"
              size="xs"
              @click="moveFileUp(index)"
            >
              <UIcon name="i-heroicons-chevron-up" />
            </UButton>
            <UButton
              v-if="index < mergeOptions.files.length - 1"
              color="neutral"
              variant="ghost"
              size="xs"
              @click="moveFileDown(index)"
            >
              <UIcon name="i-heroicons-chevron-down" />
            </UButton>
          </div>

          <!-- File order indicator -->
          <div class="flex items-center justify-center w-8 h-8 bg-primary-100 dark:bg-primary-900 text-primary-700 dark:text-primary-300 rounded-full text-sm font-medium">
            {{ index + 1 }}
          </div>

          <!-- File info -->
          <div class="flex-1 min-w-0">
            <div class="flex items-center gap-2">
              <UIcon name="i-heroicons-document-text" class="text-red-500 flex-shrink-0" />
              <span class="text-sm font-medium text-gray-700 dark:text-gray-300 truncate">
                {{ file.name }}
              </span>
            </div>
            <div class="flex items-center gap-2 text-xs text-gray-500 mt-1">
              <span>{{ formatFileSize(file.size) }}</span>
              <span v-if="file.pageCount > 0">• {{ file.pageCount }} pages</span>
            </div>
          </div>
        </div>
      </div>

      <!-- Empty state -->
      <div v-else class="p-8 text-center border-2 border-dashed border-gray-300 dark:border-gray-600 rounded-lg">
        <UIcon name="i-heroicons-document-plus" class="text-4xl text-gray-400 dark:text-gray-500 mb-2 mx-auto" />
        <p class="text-gray-600 dark:text-gray-400 mb-4">No PDF files selected</p>
        <p class="text-xs text-gray-500">Please select files using the main file browser above</p>
      </div>
    </div>

    <!-- Merge Preview -->
    <div v-if="mergeOptions.files.length > 1" class="p-4 bg-blue-50 dark:bg-blue-900/20 rounded-lg border border-blue-200 dark:border-blue-800">
      <div class="flex items-center gap-2 mb-2">
        <UIcon name="i-heroicons-information-circle" class="text-blue-500" />
        <span class="text-sm font-medium text-blue-700 dark:text-blue-300">Merge Preview</span>
      </div>
      <p class="text-sm text-blue-600 dark:text-blue-400">
        Files will be merged in the order shown: 
        <strong>{{ mergeOptions.files.map(f => f.name).join(' → ') }}</strong>
      </p>
      <p class="text-xs text-blue-500 dark:text-blue-500 mt-1">
        Total pages: {{ totalPages }}
      </p>
    </div>

    <!-- Validation Messages -->
    <div v-if="validationErrors.length > 0" class="space-y-2">
      <div v-for="error in validationErrors" :key="error" class="p-3 bg-red-50 dark:bg-red-900/20 rounded border border-red-200 dark:border-red-800">
        <p class="text-sm text-red-600 dark:text-red-400">{{ error }}</p>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'

export interface MergeFile {
  id: string
  name: string
  path: string
  size: number
  pageCount: number
}

export interface MergeOptions {
  files: MergeFile[]
}

interface Props {
  selectedFiles: { path: string; name: string; size?: number; pageCount?: number }[]
}

interface Emits {
  (e: 'validate', isValid: boolean, errors: string[]): void
  (e: 'reorder', files: { path: string; name: string; size?: number; pageCount?: number }[]): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

// Model
const mergeOptions = defineModel<MergeOptions>('mergeOptions', {
  required: true
})

// State
const validationErrors = ref<string[]>([])

// Sync mergeOptions with props.selectedFiles
watch(() => props.selectedFiles, (newFiles) => {
  mergeOptions.value.files = newFiles.map((file, index) => ({
    id: file.path + index,
    name: file.name,
    path: file.path,
    size: file.size || 0,
    pageCount: file.pageCount || 0
  }))
}, { immediate: true, deep: true })

// Computed
const totalPages = computed(() => {
  return mergeOptions.value.files.reduce((sum, file) => sum + file.pageCount, 0)
})

const isValid = computed(() => {
  const errors: string[] = []
  
  if (mergeOptions.value.files.length < 2) {
    errors.push('Please select at least 2 PDF files to merge')
  }
  
  if (mergeOptions.value.files.length > 2) {
    errors.push('Maximum 2 PDF files allowed for merging')
  }

  validationErrors.value = errors
  return errors.length === 0
})

// Watch for validation changes
watch(isValid, (valid) => {
  emit('validate', valid, validationErrors.value)
}, { immediate: true })

// Methods

const moveFileUp = (index: number) => {
  if (index > 0) {
    const files = [...mergeOptions.value.files]
    ;[files[index], files[index - 1]] = [files[index - 1], files[index]]
    mergeOptions.value.files = files
    // Emit the reordering back to parent
    emitReorderedFiles(files)
  }
}

const moveFileDown = (index: number) => {
  if (index < mergeOptions.value.files.length - 1) {
    const files = [...mergeOptions.value.files]
    ;[files[index], files[index + 1]] = [files[index + 1], files[index]]
    mergeOptions.value.files = files
    // Emit the reordering back to parent
    emitReorderedFiles(files)
  }
}

const emitReorderedFiles = (files: MergeFile[]) => {
  const reorderedFiles = files.map(f => ({
    path: f.path,
    name: f.name,
    size: f.size,
    pageCount: f.pageCount
  }))
  emit('reorder', reorderedFiles)
}

const formatFileSize = (bytes: number): string => {
  if (bytes === 0) return '0 Bytes'
  const k = 1024
  const sizes = ['Bytes', 'KB', 'MB', 'GB']
  const i = Math.floor(Math.log(bytes) / Math.log(k))
  return parseFloat((bytes / Math.pow(k, i)).toFixed(2)) + ' ' + sizes[i]
}
</script>