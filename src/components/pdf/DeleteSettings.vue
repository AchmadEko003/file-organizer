<template>
  <div class="space-y-4">
    <div class="flex gap-2">
      <UInput 
        v-model="deletePages" 
        type="text"
        placeholder="1,3,5-10"
      />
    </div>

    <div class="space-y-3">
      <div v-if="pdfPageCount > 0" class="text-sm text-gray-600 dark:text-gray-400">
        <p class="mb-2">Available pages: 1-{{ pdfPageCount }}</p>
        <div class="space-y-1">
          <p><strong>Examples:</strong></p>
          <ul class="list-disc list-inside space-y-1 ml-2">
            <li>"1,3,5" - Delete specific pages 1, 3, and 5</li>
            <li>"1-5" - Delete pages 1 through 5</li>
            <li>"1,3,7-10" - Delete page 1, 3, and pages 7 through 10</li>
          </ul>
        </div>
      </div>

      <div v-if="deletePages && isValid" class="p-3 bg-amber-50 dark:bg-amber-950 rounded-lg">
        <div class="text-sm text-amber-700 dark:text-amber-300">
          <UIcon name="i-heroicons-exclamation-triangle" class="inline mr-1" />
          <strong>Warning:</strong> {{ getDeleteSummary() }}
        </div>
      </div>

      <div v-if="deletePages && !isValid && validationError" class="p-3 bg-red-50 dark:bg-red-950 rounded-lg">
        <div class="text-sm text-red-700 dark:text-red-300">
          <UIcon name="i-heroicons-x-circle" class="inline mr-1" />
          {{ validationError }}
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { computed, watch } from 'vue'

interface Props {
  pdfPageCount: number
}

interface Emits {
  (e: 'validate', isValid: boolean, error?: string): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const deletePages = defineModel<string>('deletePages', {
  required: true,
  default: ''
})

// Validation logic
const validationResult = computed(() => {
  return validateDeletePages(deletePages.value, props.pdfPageCount)
})

const isValid = computed(() => validationResult.value.isValid)
const validationError = computed(() => validationResult.value.error)

// Parse and get unique pages to delete
const getPagesToDelete = (): number[] => {
  if (!deletePages.value.trim() || !isValid.value) return []

  const pages = new Set<number>()
  const ranges = deletePages.value.split(',').map(r => r.trim()).filter(r => r.length > 0)
  
  for (const range of ranges) {
    if (range.includes('-')) {
      const [startStr, endStr] = range.split('-').map(s => s.trim())
      const start = parseInt(startStr)
      const end = parseInt(endStr)
      
      for (let i = start; i <= end; i++) {
        pages.add(i)
      }
    } else {
      const page = parseInt(range)
      pages.add(page)
    }
  }
  
  return Array.from(pages).sort((a, b) => a - b)
}

const getDeleteSummary = (): string => {
  const pagesToDelete = getPagesToDelete()
  const remainingPages = props.pdfPageCount - pagesToDelete.length
  
  if (pagesToDelete.length === 0) return ''
  
  const pageText = pagesToDelete.length === 1 ? 'page' : 'pages'
  const remainingText = remainingPages === 1 ? 'page' : 'pages'
  
  return `This will delete ${pagesToDelete.length} ${pageText}, leaving ${remainingPages} ${remainingText} in the document.`
}

// Validation function
const validateDeletePages = (deletePages: string, totalPages: number): { isValid: boolean; error?: string } => {
  if (!deletePages.trim()) {
    return { isValid: false, error: 'Please specify pages to delete' }
  }

  if (totalPages === 0) {
    return { isValid: false, error: 'PDF page count not available' }
  }

  try {
    const ranges = deletePages.split(',').map(r => r.trim()).filter(r => r.length > 0)
    const pagesToDelete = new Set<number>()
    
    for (const range of ranges) {
      if (range.includes('-')) {
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

        for (let i = start; i <= end; i++) {
          pagesToDelete.add(i)
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

        pagesToDelete.add(page)
      }
    }

    // Check if trying to delete all pages
    if (pagesToDelete.size >= totalPages) {
      return { isValid: false, error: 'Cannot delete all pages. At least one page must remain.' }
    }
    
    return { isValid: true }
  } catch (error) {
    return { isValid: false, error: 'Invalid page format' }
  }
}

// Watch for validation changes and emit
watch([isValid, validationError], () => {
  emit('validate', isValid.value, validationError.value)
}, { immediate: true })
</script>