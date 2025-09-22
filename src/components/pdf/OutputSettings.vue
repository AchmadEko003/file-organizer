<template>
  <div class="space-y-4">
    <USeparator label="Output Settings" />

    <UFormGroup label="Save Location" help="Choose where to save the processed files">
      <div class="flex gap-2">
        <UInput 
          v-model="localOutputPath" 
          placeholder="Select output folder..." 
          readonly 
          class="flex-1"
          @update:model-value="handleOutputPathChange"
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

    <div v-if="localOutputPath" class="text-sm text-gray-600 dark:text-gray-400">
      <UIcon name="i-heroicons-folder" class="inline mr-1" />
      Files will be saved to: {{ localOutputPath }}
    </div>

    <div v-if="!localOutputPath" class="p-3 bg-amber-50 dark:bg-amber-950 rounded-lg">
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

interface Props {
  outputPath: string
}

interface Emits {
  (e: 'update:outputPath', value: string): void
  (e: 'validate', isValid: boolean, error?: string): void
}

const props = defineProps<Props>()
const emit = defineEmits<Emits>()

const localOutputPath = ref<string>(props.outputPath)
const isSelecting = ref(false)

// Watch for external changes
watch(() => props.outputPath, (newValue) => {
  localOutputPath.value = newValue
})

const selectOutputPath = async () => {
  try {
    isSelecting.value = true
    
    const selected = await open({
      directory: true,
      multiple: false,
      title: 'Select Output Folder'
    })

    if (selected && typeof selected === 'string') {
      localOutputPath.value = selected
      handleOutputPathChange()
    }
  } catch (error) {
    console.error('Error selecting output path:', error)
  } finally {
    isSelecting.value = false
  }
}

const handleOutputPathChange = () => {
  emit('update:outputPath', localOutputPath.value)
  const isValid = !!localOutputPath.value.trim()
  emit('validate', isValid, isValid ? undefined : 'Please select an output folder')
}

// Initial validation
handleOutputPathChange()
</script>