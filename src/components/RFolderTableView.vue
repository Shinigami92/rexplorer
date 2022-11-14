<script setup lang="ts">
import RFileSize from '@/components/RFileSize.vue';
import type { PathResponse } from '@/shared/PathResponse';
import { invoke } from '@tauri-apps/api/tauri';
import { useVModels } from '@vueuse/core';
import { ref, watch } from 'vue';

const props = defineProps<{
  currentPath: PathResponse;
}>();

const emit = defineEmits<{
  (event: 'update:currentPath', path: PathResponse): void;
}>();

const { currentPath } = useVModels(props, emit);

const paths = ref<PathResponse[]>([]);

async function backendPaths(): Promise<void> {
  const result = await invoke<PathResponse[]>('get_paths', {
    path: currentPath.value.path,
  });

  paths.value = result.sort((a, b) => {
    if (a.is_dir === b.is_dir) {
      return a.name.localeCompare(b.name);
    }
    return a.is_dir ? -1 : 1;
  });
}

watch(currentPath, backendPaths, { immediate: true });
</script>

<template lang="pug">
div
  h1 {{ currentPath.path }}

  table
    tr
      th Name
      th Size

    template(v-for="path in paths", :key="path.path")
      tr(v-if="path.is_dir", @click="currentPath = path")
        td
          i.mdi.mdi-folder
          span.ml-2 {{ path.name }}
        td

      tr(v-else)
        td
          i.mdi.mdi-file
          span.ml-2 {{ path.name }}
        td.text-right
          RFileSize(:file-size="path.size")
</template>
