<script setup lang="ts">
import { ref } from 'vue'
import type { NavigationMenuItem } from '@nuxt/ui'

const items = ref<NavigationMenuItem[][]>([
  [
    {
      label: 'File Organizer',
      icon: 'i-lucide-home',
      to: '/',
    },
    {
      label: 'Other Tools',
      type: 'label',
      ui: {
        item: 'active:scale-none cursor-default'
      },
      disabled: true
    },
    {
      label: 'PDF',
      icon: 'i-heroicons-document-text',
      to: '/pdf'
    }
  ]
])
</script>

<template>
  <UNavigationMenu orientation="vertical" :items="items" class="data-[orientation=vertical]:w-48" :ui="{
    item: 'cursor-pointer active:scale-95 transition-transform hover:bg-white rounded-lg',
  }" />
</template>
