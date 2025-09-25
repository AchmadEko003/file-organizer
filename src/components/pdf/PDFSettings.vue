<template>
  <div class="space-y-6">
    <!-- Action-specific settings -->
    <SplitSettings
      v-if="selectedAction === 'split'"
      v-model:split-options="settings.splitOptions"
      :pdf-page-count="pdfPageCount"
      @validate="handleSplitValidation"
    />

    <CompressSettings
      v-if="selectedAction === 'compress'"
      v-model:compression-level="settings.compressionLevel"
      @validate="handleCompressValidation"
    />

    <DeleteSettings
      v-if="selectedAction === 'delete'"
      v-model:delete-pages="settings.deletePages"
      :pdf-page-count="pdfPageCount"
      @validate="handleDeleteValidation"
    />

    <MergeSettings
      v-if="selectedAction === 'merge'"
      v-model:merge-options="settings.mergeOptions"
      :selected-files="selectedFiles || []"
      @validate="handleMergeValidation"
      @reorder="handleFileReorder"
    />

    <!-- Output settings (common for all actions except merge) -->
    <OutputSettings
      v-if="selectedAction !== 'merge'"
      v-model:output-path="settings.outputPath"
      @validate="handleOutputValidation"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import SplitSettings from './SplitSettings.vue'
import CompressSettings from './CompressSettings.vue'
import DeleteSettings from './DeleteSettings.vue'
import MergeSettings from './MergeSettings.vue'
import OutputSettings from './OutputSettings.vue'
import type { PDFSettings } from './types'

interface Props {
  selectedAction: 'split' | 'compress' | 'delete' | 'merge' | ''
  pdfPageCount: number
  selectedFiles?: { path: string; name: string; size?: number; pageCount?: number }[]
}

interface Emits {
  (e: 'validate', isValid: boolean, errors: string[]): void
  (e: 'file-reorder', files: { path: string; name: string; size?: number; pageCount?: number }[]): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const settings = defineModel<PDFSettings>('settings', {
  required: true,
  default: () => ({
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
})

// Validation states
const splitValidation = ref({ isValid: true, error: undefined as string | undefined })
const compressValidation = ref({ isValid: true, error: undefined as string | undefined })
const deleteValidation = ref({ isValid: true, error: undefined as string | undefined })
const mergeValidation = ref({ isValid: true, error: undefined as string | undefined })
const outputValidation = ref({ isValid: true, error: undefined as string | undefined })

// Computed overall validation
const isValid = computed(() => {
  // For merge action, output path is handled within merge settings
  if (props.selectedAction === 'merge') {
    return mergeValidation.value.isValid
  }

  // Output path is always required for other actions
  if (!outputValidation.value.isValid) return false

  // Check action-specific validation
  switch (props.selectedAction) {
    case 'split':
      return splitValidation.value.isValid
    case 'compress':
      return compressValidation.value.isValid
    case 'delete':
      return deleteValidation.value.isValid
    default:
      return false
  }
})

const validationErrors = computed(() => {
  const errors: string[] = []
  
  // Only check output validation for non-merge actions
  if (props.selectedAction !== 'merge' && !outputValidation.value.isValid && outputValidation.value.error) {
    errors.push(outputValidation.value.error)
  }

  switch (props.selectedAction) {
    case 'split':
      if (!splitValidation.value.isValid && splitValidation.value.error) {
        errors.push(splitValidation.value.error)
      }
      break
    case 'compress':
      if (!compressValidation.value.isValid && compressValidation.value.error) {
        errors.push(compressValidation.value.error)
      }
      break
    case 'delete':
      if (!deleteValidation.value.isValid && deleteValidation.value.error) {
        errors.push(deleteValidation.value.error)
      }
      break
    case 'merge':
      if (!mergeValidation.value.isValid && mergeValidation.value.error) {
        errors.push(mergeValidation.value.error)
      }
      break
  }

  return errors
})

// Emit settings and validation changes
const emitValidation = () => {
  emit('validate', isValid.value, validationErrors.value)
}

// Validation handlers
const handleSplitValidation = (valid: boolean, error?: string) => {
  splitValidation.value = { isValid: valid, error }
  emitValidation()
}

const handleCompressValidation = (valid: boolean, error?: string) => {
  compressValidation.value = { isValid: valid, error }
  emitValidation()
}

const handleDeleteValidation = (valid: boolean, error?: string) => {
  deleteValidation.value = { isValid: valid, error }
  emitValidation()
}

const handleMergeValidation = (valid: boolean, errors: string[]) => {
  mergeValidation.value = { isValid: valid, error: errors.join(', ') }
  emitValidation()
}

const handleFileReorder = (files: { path: string; name: string; size?: number; pageCount?: number }[]) => {
  emit('file-reorder', files)
}

const handleOutputValidation = (valid: boolean, error?: string) => {
  outputValidation.value = { isValid: valid, error }
  emitValidation()
}

// Watch for validation changes
watch([isValid, validationErrors], () => {
  emit('validate', isValid.value, validationErrors.value)
})
</script>