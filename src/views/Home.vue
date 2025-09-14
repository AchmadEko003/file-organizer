<template>
    <div class="min-h-screen">
        <!-- Browser Card -->
        <div class="p-3">
            <!-- Path Input Section -->
            <div class="pb-6 pt-3 bg-slate-100">
                <div class="grid grid-cols-12 gap-3">
                    <UButtonGroup class="col-span-9">
                        <div
                            class="h-auto w-full flex items-center px-1 rounded-l-lg bg-white border-[1px] border-slate-300">
                            <div v-for="(value, i) in reformatedPath" :key="value.name"
                                class="flex items-center gap-1 ml-1 overflow-x-auto">
                                <UBadge color="neutral" variant="soft"
                                    class="items-center px-2 py-1 !rounded-lg text-xs font-bold text-gray-800 hover:bg-gray-200 cursor-pointer active:scale-95 transition-transform"
                                    @click="handleBadgeClick(value.path, $event)">
                                    {{ value.name }}
                                </UBadge>
                                <UIcon name="i-heroicons-chevron-right" v-if="i < reformatedPath.length - 1"
                                    class="h-3">
                                </UIcon>
                            </div>
                        </div>

                        <!-- <UInput v-if="showPathInput" :ui="{
                            base: 'bg-white font-bold text-gray-700'
                        }" v-model="selectedPath" size="lg" placeholder="Select a folder to browse..."
                            class="rounded-xl w-full" @keyup.enter="loadFiles; hidePathInput()"
                            @keyup.escape="hidePathInput()" @blur="hidePathInput()">
                            <template #leading>
                                <UIcon name="i-heroicons-folder" class="w-5 h-5 text-gray-400" />
                            </template>
<template v-if="selectedPath?.length" #trailing>
                                <UButton color="neutral" variant="ghost" size="sm" icon="i-heroicons-x-mark"
                                    @click="clearPath" />
                            </template>
</UInput> -->

                        <UButton @click="browse" size="lg"
                            class="rounded-lg active:scale-[98%] cursor-pointer transition-transform">
                            <UIcon name="i-heroicons-folder-open" class="w-5 h-5 mr-1" />
                            Browse
                        </UButton>
                    </UButtonGroup>

                    <UButtonGroup class="col-span-3">
                        <UInput :ui="{
                            base: 'bg-white font-bold text-gray-700'
                        }" v-model="fileName" size="lg"
                            :placeholder="'Search ' + reformatedPath[reformatedPath.length - 1]?.name"
                            class="rounded-xl w-full" @keydown.enter="searchFiles">
                            <template v-if="fileName?.length" #trailing>
                                <UButton color="neutral" variant="ghost" size="sm" icon="i-heroicons-x-mark"
                                    @click="clearSearch" />
                            </template>
                        </UInput>

                        <UButton @click="searchFiles" icon="i-heroicons-magnifying-glass"
                            class="rounded-lg active:scale-[98%] cursor-pointer transition-transform">
                        </UButton>
                    </UButtonGroup>
                </div>
            </div>

            <!-- Stats Cards -->
            <div v-if="fileList.length > 0" class="grid grid-cols-1 md:grid-cols-3 gap-4 mb-6">
                <div class="bg-gradient-to-br from-blue-50 to-blue-100 rounded-xl p-4 border border-blue-200">
                    <div class="flex items-center justify-between">
                        <div>
                            <p class="text-blue-600 text-sm font-medium">Total Files</p>
                            <p class="text-2xl font-bold text-blue-900">{{ fileList.length }}</p>
                        </div>
                        <UIcon name="i-heroicons-document-text" class="w-8 h-8 text-blue-500" />
                    </div>
                </div>

                <div class="bg-gradient-to-br from-green-50 to-green-100 rounded-xl p-4 border border-green-200">
                    <div class="flex items-center justify-between">
                        <div>
                            <p class="text-green-600 text-sm font-medium">File Types</p>
                            <p class="text-2xl font-bold text-green-900">{{ uniqueFileTypes }}</p>
                        </div>
                        <UIcon name="i-heroicons-squares-2x2" class="w-8 h-8 text-green-500" />
                    </div>
                </div>

                <div class="bg-gradient-to-br from-purple-50 to-purple-100 rounded-xl p-4 border border-purple-200">
                    <div class="flex items-center justify-between">
                        <div>
                            <p class="text-purple-600 text-sm font-medium">Total Size</p>
                            <p class="text-2xl font-bold text-purple-900">{{ formatTotalSize }}</p>
                        </div>
                        <UIcon name="i-heroicons-chart-pie" class="w-8 h-8 text-purple-500" />
                    </div>
                </div>
            </div>

            <!-- File List -->
            <div v-if="fileList.length > 0" class="space-y-4">
                <div class="flex items-center justify-between">
                    <h3 class="text-lg font-semibold text-gray-900">Files</h3>
                    <div class="flex gap-2">
                        <UButton variant="outline" color="neutral" @click="organize"
                            class="rounded-lg cursor-pointer active:scale-95 transition-transform">
                            Organize Folder
                        </UButton>
                        <UButton variant="outline" color="neutral" @click="toggleView"
                            class="rounded-lg cursor-pointer active:scale-95 transition-transform">
                            <UIcon :name="viewMode === 'grid' ? 'i-heroicons-list-bullet' : 'i-heroicons-squares-2x2'"
                                class="w-4 h-4" />
                        </UButton>
                        <UButton variant="outline" color="neutral" @click="loadFiles"
                            class="rounded-lg cursor-pointer active:scale-95 transition-transform">
                            <UIcon name="i-heroicons-arrow-path" class="w-4 h-4" />
                        </UButton>
                    </div>
                </div>

                <!-- Grid View -->
                <div v-if="viewMode === 'grid'"
                    class="grid grid-cols-1 sm:grid-cols-2 lg:grid-cols-3 xl:grid-cols-4 gap-2">
                    <div v-for="(file, index) in fileList" :key="index" @dblclick="openFolder(file.name, file.is_dir)"
                        class="group bg-white border border-gray-200 rounded-xl px-3 py-2 hover:shadow-lg hover:border-blue-300 transition-all duration-200 cursor-pointer">
                        <div class="flex items-start space-x-3">
                            <div class="flex-shrink-0">
                                <div class="w-10 h-10 rounded-lg flex items-center justify-center"
                                    :class="getFileIconBg(file)">
                                    <UIcon :name="getFileIcon(file)" class="w-5 h-5" :class="getFileIconColor(file)" />
                                </div>
                            </div>
                            <div class="flex-1 min-w-0">
                                <p class="text-sm font-medium text-gray-900 truncate" :title="getFileName(file.name)">
                                    {{ getFileName(file.name) }}
                                </p>
                                <p class="text-xs text-gray-500 mt-1">
                                    {{ file.is_dir ? '' : formatFileSize(file.size) }}
                                </p>
                                <div class="flex items-center mt-2">
                                    <span v-if="file.is_file"
                                        class="inline-flex items-center px-2 py-0.5 rounded text-xs font-medium"
                                        :class="getFileTypeBadge(file)">
                                        {{ getFileExtension(file.name) }}
                                    </span>
                                </div>
                            </div>
                        </div>
                    </div>
                </div>

                <!-- Table View -->
                <div v-else class="bg-white border border-gray-200 rounded-xl overflow-hidden">
                    <div class="overflow-x-auto">
                        <table class="min-w-full divide-y divide-gray-200">
                            <thead class="bg-gray-50">
                                <tr>
                                    <th
                                        class="px-3 py-2 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                        Name
                                    </th>
                                    <th
                                        class="px-3 py-2 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                        Type
                                    </th>
                                    <th
                                        class="px-3 py-2 text-left text-xs font-medium text-gray-500 uppercase tracking-wider">
                                        Size
                                    </th>
                                </tr>
                            </thead>
                            <tbody class="bg-white divide-y divide-gray-200">
                                <tr v-for="(file, index) in fileList" :key="index"
                                    @dblclick="openFolder(file.name, file.is_dir)"
                                    class="hover:bg-gray-100 transition-colors cursor-pointer">
                                    <td class="px-3 py-2 whitespace-nowrap">
                                        <div class="flex items-center space-x-3">
                                            <div class="flex-shrink-0">
                                                <div class="w-8 h-8 rounded-lg flex items-center justify-center"
                                                    :class="getFileIconBg(file)">
                                                    <UIcon :name="getFileIcon(file)" class="w-4 h-4"
                                                        :class="getFileIconColor(file)" />
                                                </div>
                                            </div>
                                            <div>
                                                <p class="font-medium text-gray-900">{{ getFileName(file.name) }}</p>
                                            </div>
                                        </div>
                                    </td>
                                    <td class="px-3 py-2 whitespace-nowrap">
                                        <span v-if="file.is_file"
                                            class="inline-flex items-center px-2.5 py-0.5 rounded-full text-xs font-medium"
                                            :class="getFileTypeBadge(file)">
                                            {{ getFileExtension(file.name) }}
                                        </span>
                                    </td>
                                    <td class="px-3 py-2 whitespace-nowrap text-sm text-gray-900">
                                        {{ file.is_dir ? '' : formatFileSize(file.size) }}
                                    </td>
                                </tr>
                            </tbody>
                        </table>
                    </div>
                </div>
            </div>

            <!-- Empty State -->
            <div v-else class="text-center py-16">
                <div class="w-24 h-24 mx-auto mb-6 bg-gray-100 rounded-full flex items-center justify-center">
                    <UIcon name="i-heroicons-folder-open" class="w-12 h-12 text-gray-400" />
                </div>
                <h3 class="text-lg font-semibold text-gray-900 mb-2">No folder selected</h3>
                <p class="text-gray-500 mb-6">Choose a folder to start exploring your files</p>
                <UButton @click="browse" size="lg" class="rounded-xl">
                    <UIcon name="i-heroicons-folder-open" class="w-5 h-5 mr-2" />
                    Browse Folders
                </UButton>
            </div>
        </div>
    </div>
</template>

<script setup lang="ts">
import { ref, computed, onMounted } from 'vue';
import { invoke } from '@tauri-apps/api/core';
import { open, message, ask } from '@tauri-apps/plugin-dialog';

interface FileItem {
    name: string;
    path: string;
    size: number;
    is_dir: boolean;
    is_file: boolean;
}

const toast = useToast();
const selectedPath = ref<string>('');
const fileList = ref<Array<FileItem>>([]);
const viewMode = ref<'grid' | 'table'>('table');
// const showPathInput = ref<boolean>(false);
const fileName = ref<string>('');
const os = ref<string>('');

onMounted(() => {
    loadFiles();
});

// Computed properties
const uniqueFileTypes = computed(() => {
    const types = new Set(fileList.value.map(file => getFileExtension(file.name)));
    return types.size;
});

const reformatedPath = computed(() => {
    let tempPath = ''
    console.log(selectedPath.value);
    const pathSeparator = os.value === 'linux' || os.value === 'macos' ? '/' : '\\';
    const result = selectedPath.value.split(pathSeparator).filter(Boolean).map(part => {
        console.log(part);
        tempPath += part + pathSeparator;
        return {
            name: part,
            path: tempPath
        }
    });

    return result;
});

const formatTotalSize = computed(() => {
    const totalBytes = fileList.value.reduce((sum, file) => sum + (file.size || 0), 0);
    return formatFileSize(totalBytes);
});

const organize = async () => {
    try {
        const confirmed = await ask(
            'This will organize files in the current folder by moving them into categorized subdirectories (images, documents, videos, others). This action cannot be undone.\n\nDo you want to continue?',
            {
                title: 'Confirm Folder Organization',
                kind: 'warning'
            }
        );

        if (!confirmed) {
            return;
        }

        const result = await invoke("organize_folder", { folderPath: selectedPath.value });

        if (result) {
            toast.add({
                title: 'Failed',
                description: 'Folder organization failed!',
                duration: 3000,
                color: 'error',
                ui: {
                    root: 'bg-white'
                }
            });
        } else {
            toast.add({
                title: 'Success',
                description: 'Folder organized successfully!',
                duration: 3000,
                color: 'success',
                ui: {
                    root: 'bg-white'
                }
            });
            await loadFiles();
        }
    } catch (error) {
        toast.add({
            title: 'Error',
            description: 'An unexpected error occurred during organization.',
            duration: 3000,
            color: 'error',
            ui: {
                root: 'bg-white'
            }
        });
    }
}

const searchFiles = async () => {
    if (fileName.value.trim() === '') {
        await loadFiles();
        return;
    }

    try {
        const result = await invoke("search_file", { folderPath: selectedPath.value, searchTerm: fileName.value });

        fileList.value = result as Array<FileItem>;
    } catch (error) {
        if ((error as string).includes('denied')) {
            selectedPath.value = selectedPath.value.split(/[\\/]/).slice(0, -1).join('/');
            await message((error as string).replace(' (os error 5)', ''), { title: 'DocuTools', kind: 'error' });
        } else {
            fileList.value = [];
        }
    }
}

const clearSearch = async () => {
    fileName.value = '';
    await loadFiles();
}

const browse = async () => {
    try {
        const file = await open({
            multiple: false,
            directory: true,
            file: false
        });

        if (file) {
            selectedPath.value = Array.isArray(file) ? file[0] : file as string;
            await loadFiles();
        }
    } catch (error) {
        console.error('Error selecting folder:', error);
    }
};

const openFolder = async (name: string, is_dir: boolean = true) => {
    try {
        if (!is_dir) return;

        selectedPath.value = selectedPath.value + name;
        await loadFiles();
    } catch (error) {
        console.error('Error opening folder:', error);
    }
};

const toPath = async (path: string) => {
    selectedPath.value = path;
    await loadFiles();
};

// const clearPath = () => {
//     selectedPath.value = '';
//     fileList.value = [];
// };

const loadFiles = async () => {
    if (selectedPath.value.trim() === '') {
        os.value = await invoke('get_root_path')

        if (os.value === 'linux') {
            selectedPath.value = '/home';
        } else if (os.value === 'macos') {
            selectedPath.value = '/Users';
        } else {
            selectedPath.value = 'C:\\';
        }
        // selectedPath.value = await invoke("get_root_path");
    }

    try {
        fileList.value = await invoke("get_list_of_files_in_folder", { folderPath: selectedPath.value });
    } catch (error) {
        if ((error as string).includes('denied')) {
            selectedPath.value = selectedPath.value.split(/[\\/]/).slice(0, -1).join('/');
            await message((error as string).replace(' (os error 5)', ''), { title: 'DocuTools', kind: 'error' });
        } else {
            fileList.value = [];
        }
    }
};

const toggleView = () => {
    viewMode.value = viewMode.value === 'grid' ? 'table' : 'grid';
};

// const togglePathInput = () => {
//     showPathInput.value = !showPathInput.value;
//     if (showPathInput.value) {
//         // Auto-focus the input when it appears
//         nextTick(() => {
//             const inputElement = document.querySelector('input[placeholder="Select a folder to browse..."]') as HTMLInputElement;
//             if (inputElement) {
//                 inputElement.focus();
//                 inputElement.select();
//             }
//         });
//     }
// };

const handleBadgeClick = (path: string, event: Event) => {
    event.stopPropagation();
    toPath(os.value === 'linux' || os.value === 'macos' ? '/' + path : path);
};

// const hidePathInput = () => {
//     showPathInput.value = false;
// };

const getFileName = (fullPath: string): string => {
    return fullPath.split(/[\\/]/).pop() || fullPath;
};

const getFileExtension = (filename: string): string => {
    const ext = filename.toLowerCase().split('.').pop() || '';
    return ext.toUpperCase() || 'FILE';
};

const getFileIcon = (file: FileItem): string => {
    if (file.is_dir) {
        return 'i-heroicons-folder';
    }

    const ext = file.name.toLowerCase().split('.').pop() || '';
    const iconMap: { [key: string]: string } = {
        pdf: 'i-heroicons-document-text',
        doc: 'i-heroicons-document-text',
        docx: 'i-heroicons-document-text',
        txt: 'i-heroicons-document-text',
        jpg: 'i-heroicons-photo',
        jpeg: 'i-heroicons-photo',
        png: 'i-heroicons-photo',
        gif: 'i-heroicons-photo',
        svg: 'i-heroicons-photo',
        mp3: 'i-heroicons-musical-note',
        wav: 'i-heroicons-musical-note',
        flac: 'i-heroicons-musical-note',
        mp4: 'i-heroicons-film',
        avi: 'i-heroicons-film',
        mov: 'i-heroicons-film',
        zip: 'i-heroicons-archive-box',
        rar: 'i-heroicons-archive-box',
        '7z': 'i-heroicons-archive-box',
        js: 'i-heroicons-code-bracket',
        ts: 'i-heroicons-code-bracket',
        html: 'i-heroicons-code-bracket',
        css: 'i-heroicons-code-bracket',
        json: 'i-heroicons-code-bracket',
        exe: 'i-heroicons-cpu-chip',
        msi: 'i-heroicons-cpu-chip'
    };
    return iconMap[ext] || 'i-heroicons-document';
};

const getFileIconBg = (file: FileItem): string => {
    if (file.is_dir) {
        return 'bg-blue-100';
    }

    const ext = file.name.toLowerCase().split('.').pop() || '';
    const bgMap: { [key: string]: string } = {
        pdf: 'bg-red-100',
        doc: 'bg-blue-100',
        docx: 'bg-blue-100',
        txt: 'bg-gray-100',
        jpg: 'bg-green-100',
        jpeg: 'bg-green-100',
        png: 'bg-green-100',
        gif: 'bg-green-100',
        svg: 'bg-green-100',
        mp3: 'bg-purple-100',
        wav: 'bg-purple-100',
        flac: 'bg-purple-100',
        mp4: 'bg-pink-100',
        avi: 'bg-pink-100',
        mov: 'bg-pink-100',
        zip: 'bg-yellow-100',
        rar: 'bg-yellow-100',
        '7z': 'bg-yellow-100',
        js: 'bg-orange-100',
        ts: 'bg-orange-100',
        html: 'bg-orange-100',
        css: 'bg-orange-100',
        json: 'bg-orange-100'
    };
    return bgMap[ext] || 'bg-gray-100';
};

const getFileIconColor = (file: FileItem): string => {
    if (file.is_dir) {
        return 'text-blue-600';
    }

    const ext = file.name.toLowerCase().split('.').pop() || '';
    const colorMap: { [key: string]: string } = {
        pdf: 'text-red-600',
        doc: 'text-blue-600',
        docx: 'text-blue-600',
        txt: 'text-gray-600',
        jpg: 'text-green-600',
        jpeg: 'text-green-600',
        png: 'text-green-600',
        gif: 'text-green-600',
        svg: 'text-green-600',
        mp3: 'text-purple-600',
        wav: 'text-purple-600',
        flac: 'text-purple-600',
        mp4: 'text-pink-600',
        avi: 'text-pink-600',
        mov: 'text-pink-600',
        zip: 'text-yellow-600',
        rar: 'text-yellow-600',
        '7z': 'text-yellow-600',
        js: 'text-orange-600',
        ts: 'text-orange-600',
        html: 'text-orange-600',
        css: 'text-orange-600',
        json: 'text-orange-600'
    };
    return colorMap[ext] || 'text-gray-600';
};

const getFileTypeBadge = (file: FileItem): string => {
    if (file.is_dir) {
        return 'bg-blue-100 text-blue-800';
    }

    const ext = file.name.toLowerCase().split('.').pop() || '';
    const badgeMap: { [key: string]: string } = {
        pdf: 'bg-red-100 text-red-800',
        doc: 'bg-blue-100 text-blue-800',
        docx: 'bg-blue-100 text-blue-800',
        txt: 'bg-gray-100 text-gray-800',
        jpg: 'bg-green-100 text-green-800',
        jpeg: 'bg-green-100 text-green-800',
        png: 'bg-green-100 text-green-800',
        gif: 'bg-green-100 text-green-800',
        svg: 'bg-green-100 text-green-800',
        mp3: 'bg-purple-100 text-purple-800',
        wav: 'bg-purple-100 text-purple-800',
        flac: 'bg-purple-100 text-purple-800',
        mp4: 'bg-pink-100 text-pink-800',
        avi: 'bg-pink-100 text-pink-800',
        mov: 'bg-pink-100 text-pink-800',
        zip: 'bg-yellow-100 text-yellow-800',
        rar: 'bg-yellow-100 text-yellow-800',
        '7z': 'bg-yellow-100 text-yellow-800',
        js: 'bg-orange-100 text-orange-800',
        ts: 'bg-orange-100 text-orange-800',
        html: 'bg-orange-100 text-orange-800',
        css: 'bg-orange-100 text-orange-800',
        json: 'bg-orange-100 text-orange-800'
    };
    return badgeMap[ext] || 'bg-gray-100 text-gray-800';
};

const formatFileSize = (bytes: number): string => {
    if (bytes === 0) return '0 B';
    const k = 1024;
    const sizes = ['B', 'KB', 'MB', 'GB', 'TB'];
    const i = Math.floor(Math.log(bytes) / Math.log(k));
    return parseFloat((bytes / Math.pow(k, i)).toFixed(1)) + ' ' + sizes[i];
};
</script>

<style scoped>
.fade-in {
    animation: fadeIn 0.5s ease-in-out;
}

@keyframes fadeIn {
    from {
        opacity: 0;
        transform: translateY(10px);
    }

    to {
        opacity: 1;
        transform: translateY(0);
    }
}

.hover-lift:hover {
    transform: translateY(-2px);
    transition: transform 0.2s ease-in-out;
}

.gradient-border {
    background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
    padding: 1px;
    border-radius: 0.75rem;
}

.gradient-border>* {
    background: white;
    border-radius: calc(0.75rem - 1px);
}

/* Smooth transitions for all interactive elements */
.transition-all {
    transition: all 0.2s ease-in-out;
}

/* Custom scrollbar for table */
.overflow-x-auto::-webkit-scrollbar {
    height: 6px;
}

.overflow-x-auto::-webkit-scrollbar-track {
    background: #f1f5f9;
    border-radius: 3px;
}

.overflow-x-auto::-webkit-scrollbar-thumb {
    background: #cbd5e1;
    border-radius: 3px;
}

.overflow-x-auto::-webkit-scrollbar-thumb:hover {
    background: #94a3b8;
}
</style>
