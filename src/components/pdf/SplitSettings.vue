<template>
  <div class="space-y-4">
    <div class="flex gap-2">
      <USelect 
        v-model="splitOptions.method" 
        :items="splitMethods" 
        placeholder="Choose split method"
        @update:model-value="handleMethodChange"
      />

      <UInput 
        v-if="splitOptions.method === 'pages'" 
        v-model="splitOptions.pageRange"
        :placeholder="pdfPageCount > 0 ? `1-${pdfPageCount}, 7, 10-15` : '1-5, 7, 10-15'"
      />

      <UInputNumber 
        v-if="splitOptions.method === 'interval'" 
        v-model="splitOptions.interval"
        placeholder="5" 
        :min="1" 
        :max="pdfPageCount > 0 ? pdfPageCount : undefined"
      />
    </div>

    <div v-if="splitOptions.method === 'pages' && pdfPageCount > 0" class="text-sm text-gray-600 dark:text-gray-400">
      <p>Available pages: 1-{{ pdfPageCount }}</p>
      <p>Examples: "1-5" (pages 1 to 5), "1,3,5" (specific pages), "1-3,7-9" (multiple ranges)</p>
    </div>

    <div v-if="splitOptions.method === 'interval' && pdfPageCount > 0" class="text-sm text-gray-600 dark:text-gray-400">
      <p>Split every {{ splitOptions.interval }} pages from {{ pdfPageCount }} total pages</p>
      <p>This will create approximately {{ Math.ceil(pdfPageCount / splitOptions.interval) }} files</p>
    </div>

    <div v-if="splitOptions.method === 'all'" class="text-sm text-gray-600 dark:text-gray-400">
      <p>Split each page into separate files ({{ pdfPageCount }} files will be created)</p>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, watch } from 'vue'

export interface SplitOptions {
  method: 'pages' | 'interval' | 'all'
  pageRange: string
  interval: number
}

interface Props {
  pdfPageCount: number
}

interface Emits {
  (e: 'validate', isValid: boolean, error?: string): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const splitOptions = defineModel<SplitOptions>('splitOptions', {
  required: true,
  default: () => ({
    method: 'pages',
    pageRange: '',
    interval: 5
  })
})

const splitMethods = [
  { value: 'pages', label: 'By Range' },
  { value: 'interval', label: 'By Interval' },
  { value: 'all', label: 'Split All' }
]

// Validation computed
const isValid = computed(() => {
  switch (splitOptions.value.method) {
    case 'pages':
      return validatePageRange(splitOptions.value.pageRange, props.pdfPageCount)
    case 'interval':
      return splitOptions.value.interval >= 1 && 
             (props.pdfPageCount === 0 || splitOptions.value.interval <= props.pdfPageCount)
    case 'all':
      return props.pdfPageCount > 0
    default:
      return false
  }
})

// Validation function for page ranges
const validatePageRange = (pageRange: string, totalPages: number): boolean => {
  if (!pageRange.trim() || totalPages === 0) return false

  try {
    const ranges = pageRange.split(',').map(r => r.trim()).filter(r => r.length > 0)
    
    for (const range of ranges) {
      if (range.includes('-')) {
        const [startStr, endStr] = range.split('-').map(s => s.trim())
        const start = parseInt(startStr)
        const end = parseInt(endStr)
        
        if (isNaN(start) || isNaN(end) || start < 1 || end > totalPages || start > end) {
          return false
        }
      } else {
        const page = parseInt(range)
        if (isNaN(page) || page < 1 || page > totalPages) {
          return false
        }
      }
    }
    
    return true
  } catch {
    return false
  }
}

// Get validation error message
const getValidationError = (): string | undefined => {
  switch (splitOptions.value.method) {
    case 'pages':
      if (!splitOptions.value.pageRange.trim()) return 'Page range cannot be empty'
      if (props.pdfPageCount === 0) return 'PDF page count not available'
      if (!validatePageRange(splitOptions.value.pageRange, props.pdfPageCount)) {
        return 'Invalid page range format or pages exceed document length'
      }
      break
    case 'interval':
      if (splitOptions.value.interval < 1) return 'Interval must be at least 1'
      if (props.pdfPageCount > 0 && splitOptions.value.interval > props.pdfPageCount) {
        return 'Interval cannot exceed total pages'
      }
      break
    case 'all':
      if (props.pdfPageCount === 0) return 'PDF page count not available'
      break
  }
  return undefined
}

const handleMethodChange = () => {
  // Reset values when method changes
  if (splitOptions.value.method === 'pages') {
    splitOptions.value.pageRange = ''
  } else if (splitOptions.value.method === 'interval') {
    splitOptions.value.interval = 5
  }
}

// Watch for validation changes and emit
watch(isValid, () => {
  emit('validate', isValid.value, getValidationError())
}, { immediate: true })
</script>