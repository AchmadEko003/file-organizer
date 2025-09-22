<template>
  <div class="space-y-4">
    <USelect 
      v-model="localCompressionLevel" 
      :options="compressionLevels" 
      placeholder="Choose compression level"
      @update:model-value="handleCompressionChange"
    />

    <div class="space-y-3">
      <div class="text-sm text-gray-600 dark:text-gray-400">
        <h4 class="font-medium text-gray-800 dark:text-gray-200 mb-2">Compression Levels:</h4>
        <ul class="space-y-1">
          <li class="flex justify-between">
            <span>• Low:</span>
            <span class="text-gray-500">Better quality, larger file</span>
          </li>
          <li class="flex justify-between">
            <span>• Medium:</span>
            <span class="text-gray-500">Balanced quality and size</span>
          </li>
          <li class="flex justify-between">
            <span>• High:</span>
            <span class="text-gray-500">Smaller file, reduced quality</span>
          </li>
        </ul>
      </div>

      <div v-if="localCompressionLevel" class="p-3 bg-blue-50 dark:bg-blue-950 rounded-lg">
        <div class="text-sm text-blue-700 dark:text-blue-300">
          <UIcon name="i-heroicons-information-circle" class="inline mr-1" />
          {{ getCompressionDescription(localCompressionLevel) }}
        </div>
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'

export type CompressionLevel = 'low' | 'medium' | 'high'

interface Props {
  compressionLevel: CompressionLevel
}

interface Emits {
  (e: 'update:compressionLevel', value: CompressionLevel): void
  (e: 'validate', isValid: boolean, error?: string): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const localCompressionLevel = ref<CompressionLevel>(props.compressionLevel)

const compressionLevels = [
  { value: 'low', label: 'Low (Better Quality)' },
  { value: 'medium', label: 'Medium (Balanced)' },
  { value: 'high', label: 'High (Smaller Size)' }
]

// Watch for external changes
watch(() => props.compressionLevel, (newValue) => {
  localCompressionLevel.value = newValue
})

const getCompressionDescription = (level: CompressionLevel): string => {
  const descriptions = {
    low: 'Minimal compression applied. Document quality will be preserved with only minor size reduction.',
    medium: 'Moderate compression applied. Good balance between file size and document quality.',
    high: 'Maximum compression applied. Significant size reduction but may affect image and text quality.'
  }
  return descriptions[level] || ''
}

const handleCompressionChange = () => {
  emit('update:compressionLevel', localCompressionLevel.value)
  // Compression settings are always valid as long as a level is selected
  emit('validate', !!localCompressionLevel.value, localCompressionLevel.value ? undefined : 'Please select a compression level')
}

// Initial validation
handleCompressionChange()
</script>