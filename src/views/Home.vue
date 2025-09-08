<template>
    <div class="w-screen flex flex-col gap-4">
        <!-- <div class="flex gap-3">
                <UInput v-model="selectedPath" size="xl" :ui="{ trailing: 'pr-1' }">
                    <template v-if="selectedPath?.length" #trailing>
                        <UButton color="neutral" variant="link" size="sm" icon="i-lucide-circle-x"
                            aria-label="Clear input" @click="selectedPath = ''" />
                    </template>
</UInput>

<UButton @click="browse" class="hover:cursor-pointer">Browse</UButton>
</div> -->

        <div class="flex gap-3 p-3">
            <div class="w-full flex gap-2 bg-white py-2 px-3 rounded-lg">
                <input type="text" class="text-gray-800 font-semibold w-full focus:outline-0 focus:border-0"
                    placeholder="">

                <UIcon name="fluent:search-24-filled"
                    class="cursor-pointer size-6 active:scale-90 transition-transform text-gray-400" />
            </div>

            <UButton @click="browse" class="hover:cursor-pointer">Browse</UButton>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { open } from '@tauri-apps/plugin-dialog';

const selectedPath = ref<string>('');

const browse = async () => {
    const file = await open({
        multiple: false,
        directory: true,
        file: false
    });

    selectedPath.value = Array.isArray(file) ? file[0] : file as string;
    console.log("Selected path:", await invoke("get_list_of_files_in_folder", { folderPath: selectedPath.value }));
}
</script>

<style scoped></style>
