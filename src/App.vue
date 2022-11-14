<script setup lang="ts">
import RFolderTableView from '@/components/RFolderTableView.vue';
import type { PathResponse } from '@/shared/PathResponse';
import { invoke } from '@tauri-apps/api/tauri';
import { onMounted, ref } from 'vue';

const currentPath = ref<PathResponse>();

onMounted(async () => {
  currentPath.value = await invoke('get_paths');
});
</script>

<template lang="pug">
header
  h1 Rexplorer

main
  aside
    span Side navigation

  section
    RFolderTableView(v-if="currentPath", v-model:current-path="currentPath")
main
</template>
