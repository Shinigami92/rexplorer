<script lang="ts">
export interface FileResponse {
  path: string;
  name: string;
  size: number;
  is_dir: boolean;
}
</script>

<script setup lang="ts">
import { invoke } from '@tauri-apps/api/tauri';
import { onMounted, ref } from 'vue';

defineProps<{ msg: string }>();

const paths = ref<FileResponse[]>([]);

async function backendPaths(): Promise<void> {
  const result = await invoke<FileResponse[]>('get_paths');
  // Sort by is_dir, then name
  paths.value = result.sort((a, b) => {
    if (a.is_dir === b.is_dir) {
      return a.name.localeCompare(b.name);
    }
    return a.is_dir ? -1 : 1;
  });
}

onMounted(backendPaths);
</script>

<template lang="pug">
.mx-auto(class="w-9/12")
  h1 {{ msg }}

  .justify-center.my-6.border(class="border-white/10")
    .w-full.space-x-2
      table
        tr
          th Name
          th Size
          th Is Directory
        tr(v-for="path in paths", :key="path.path")
          td {{ path.name }}
          td.text-right {{ path.is_dir ? '' : path.size }}
          td {{ path.is_dir }}
</template>
