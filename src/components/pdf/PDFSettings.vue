<template>
  <div class="space-y-6">
    <!-- Action-specific settings -->
    <SplitSettings
      v-if="selectedAction === 'split'"
      :split-options="splitOptions"
      :pdf-page-count="pdfPageCount"
      @update:split-options="handleSplitOptionsUpdate"
      @validate="handleSplitValidation"
    />

    <CompressSettings
      v-if="selectedAction === 'compress'"
      :compression-level="compressionLevel"
      @update:compression-level="handleCompressionLevelUpdate"
      @validate="handleCompressValidation"
    />

    <DeleteSettings
      v-if="selectedAction === 'delete'"
      :delete-pages="deletePages"
      :pdf-page-count="pdfPageCount"
      @update:delete-pages="handleDeletePagesUpdate"
      @validate="handleDeleteValidation"
    />

    <!-- Output settings (common for all actions) -->
    <OutputSettings
      :output-path="outputPath"
      @update:output-path="handleOutputPathUpdate"
      @validate="handleOutputValidation"
    />
  </div>
</template>

<script setup lang="ts">
import { ref, computed, watch } from 'vue'
import SplitSettings from './SplitSettings.vue'
import CompressSettings from './CompressSettings.vue'
import DeleteSettings from './DeleteSettings.vue'
import OutputSettings from './OutputSettings.vue'
import type { SplitOptions } from './SplitSettings.vue'
import type { CompressionLevel } from './CompressSettings.vue'
import type { PDFSettings } from './types'

interface Props {
  selectedAction: 'split' | 'compress' | 'delete' | ''
  settings: PDFSettings
  pdfPageCount: number
}

interface Emits {
  (e: 'update:settings', value: PDFSettings): void
  (e: 'validate', isValid: boolean, errors: string[]): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

// Local settings state
const splitOptions = ref<SplitOptions>({ ...props.settings.splitOptions })
const compressionLevel = ref<CompressionLevel>(props.settings.compressionLevel)
const deletePages = ref<string>(props.settings.deletePages)
const outputPath = ref<string>(props.settings.outputPath)

// Validation states
const splitValidation = ref({ isValid: true, error: undefined as string | undefined })
const compressValidation = ref({ isValid: true, error: undefined as string | undefined })
const deleteValidation = ref({ isValid: true, error: undefined as string | undefined })
const outputValidation = ref({ isValid: true, error: undefined as string | undefined })

// Watch for external settings changes
watch(() => props.settings, (newSettings) => {
  splitOptions.value = { ...newSettings.splitOptions }
  compressionLevel.value = newSettings.compressionLevel
  deletePages.value = newSettings.deletePages
  outputPath.value = newSettings.outputPath
}, { deep: true })

// Computed overall validation
const isValid = computed(() => {
  // Output path is always required
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
  
  if (!outputValidation.value.isValid && outputValidation.value.error) {
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
  }

  return errors
})

// Emit settings and validation changes
const emitChanges = () => {
  const updatedSettings: PDFSettings = {
    splitOptions: { ...splitOptions.value },
    compressionLevel: compressionLevel.value,
    deletePages: deletePages.value,
    outputPath: outputPath.value
  }
  
  emit('update:settings', updatedSettings)
  emit('validate', isValid.value, validationErrors.value)
}

// Handler functions
const handleSplitOptionsUpdate = (newSplitOptions: SplitOptions) => {
  splitOptions.value = { ...newSplitOptions }
  emitChanges()
}

const handleCompressionLevelUpdate = (newCompressionLevel: CompressionLevel) => {
  compressionLevel.value = newCompressionLevel
  emitChanges()
}

const handleDeletePagesUpdate = (newDeletePages: string) => {
  deletePages.value = newDeletePages
  emitChanges()
}

const handleOutputPathUpdate = (newOutputPath: string) => {
  outputPath.value = newOutputPath
  emitChanges()
}

// Validation handlers
const handleSplitValidation = (valid: boolean, error?: string) => {
  splitValidation.value = { isValid: valid, error }
  emitChanges()
}

const handleCompressValidation = (valid: boolean, error?: string) => {
  compressValidation.value = { isValid: valid, error }
  emitChanges()
}

const handleDeleteValidation = (valid: boolean, error?: string) => {
  deleteValidation.value = { isValid: valid, error }
  emitChanges()
}

const handleOutputValidation = (valid: boolean, error?: string) => {
  outputValidation.value = { isValid: valid, error }
  emitChanges()
}

// Watch for validation changes
watch([isValid, validationErrors], () => {
  emit('validate', isValid.value, validationErrors.value)
})

// Initial validation
emitChanges()
</script>