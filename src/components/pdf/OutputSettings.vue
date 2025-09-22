<template>
  <div class="space-y-4">
    <USeparator label="Output Settings" />

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
          :loading="isSelecting"
        >
          <UIcon name="i-heroicons-folder" class="mr-2" />
          Browse
        </UButton>
      </div>
    </UFormGroup>

    <div v-if="outputPath" class="text-sm text-gray-600 dark:text-gray-400">
      <UIcon name="i-heroicons-folder" class="inline mr-1" />
      Files will be saved to: {{ outputPath }}
    </div>

    <div v-if="!outputPath" class="p-3 bg-amber-50 dark:bg-amber-950 rounded-lg">
      <div class="text-sm text-amber-700 dark:text-amber-300">
        <UIcon name="i-heroicons-exclamation-triangle" class="inline mr-1" />
        Please select an output folder to save the processed files.
      </div>
    </div>
  </div>
</template>

<script setup lang="ts">
import { ref, watch } from 'vue'
import { open } from '@tauri-apps/plugin-dialog'

interface Emits {
  (e: 'validate', isValid: boolean, error?: string): void
}

const emit = defineEmits<Emits>()

const outputPath = defineModel<string>('outputPath', {
  required: true,
  default: ''
})

const isSelecting = ref(false)

const selectOutputPath = async () => {
  try {
    isSelecting.value = true
    
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
  } finally {
    isSelecting.value = false
  }
}

// Watch outputPath changes and emit validation
watch(outputPath, () => {
  const isValid = !!outputPath.value.trim()
  emit('validate', isValid, isValid ? undefined : 'Please select an output folder')
}, { immediate: true })
</script>