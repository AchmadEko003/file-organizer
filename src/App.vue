<template>
  <UApp :class="{ 'production-security': !isDevelopment }" @contextmenu="handleContextMenu" @keydown="handleKeydown">
    <div class="flex">
      <aside class="w-fit px-1 border-r border-gray-200">
        <div class="text-2xl text-center py-6 font-bold">DocuTools</div>
        <Navigation />
      </aside>

      <main class="flex-1 max-h-screen overflow-y-auto">
        <RouterView />
      </main>
    </div>
  </UApp>
</template>

<script setup lang="ts">
import { onMounted, onUnmounted } from 'vue';
import Navigation from './components/Navigation.vue';

const isDevelopment = import.meta.env.DEV;

const handleContextMenu = (e: Event) => {
  if (!isDevelopment) {
    e.preventDefault();
    return false;
  }
};

const handleKeydown = (e: KeyboardEvent) => {
  if (isDevelopment) return;
  if (e.key === 'F12') {
    e.preventDefault();
    return false;
  }
  if (e.ctrlKey && e.shiftKey && e.key === 'I') {
    e.preventDefault();
    return false;
  }
  if (e.metaKey && e.altKey && e.key === 'I') {
    e.preventDefault();
    return false;
  }
  if (e.ctrlKey && e.shiftKey && e.key === 'J') {
    e.preventDefault();
    return false;
  }
  if (e.metaKey && e.altKey && e.key === 'J') {
    e.preventDefault();
    return false;
  }
  if (e.ctrlKey && e.shiftKey && e.key === 'C') {
    e.preventDefault();
    return false;
  }
  if (e.metaKey && e.altKey && e.key === 'C') {
    e.preventDefault();
    return false;
  }
  if (e.ctrlKey && e.key === 'u') {
    e.preventDefault();
    return false;
  }
  if (e.metaKey && e.key === 'u') {
    e.preventDefault();
    return false;
  }
};

const disableConsole = () => {
  if (!isDevelopment) {
    console.log = () => {};
    console.warn = () => {};
    console.error = () => {};
    console.info = () => {};
    console.debug = () => {};
    console.table = () => {};
    console.trace = () => {};
  }
};

const detectDevTools = () => {
  if (!isDevelopment) {
    const threshold = 160;
    setInterval(() => {
      if (
        window.outerHeight - window.innerHeight > threshold ||
        window.outerWidth - window.innerWidth > threshold
      ) {
        console.warn('Developer tools detected!');
      }
    }, 1000);
  }
};

onMounted(() => {
  if (!isDevelopment) {
    disableConsole();
    detectDevTools();
    document.addEventListener('keydown', handleKeydown);
    document.addEventListener('contextmenu', handleContextMenu);
  }
});

onUnmounted(() => {
  if (!isDevelopment) {
    document.removeEventListener('keydown', handleKeydown);
    document.removeEventListener('contextmenu', handleContextMenu);
  }
});
</script>

<style>
/* Production-only: Disable text selection to prevent inspect via selection */
/* Note: CSS doesn't have environment detection, so this applies in all builds */
/* You can conditionally apply this class via JavaScript if needed */
.production-security * {
  -webkit-user-select: none;
  -moz-user-select: none;
  -ms-user-select: none;
  user-select: none;
}

.production-security input, 
.production-security textarea {
  -webkit-user-select: text;
  -moz-user-select: text;
  -ms-user-select: text;
  user-select: text;
}
</style>