<script setup lang="ts">
import { computed } from 'vue';

const props = defineProps<{ fileSize: number }>();

const sizeText = computed(() => {
  const fileSize = props.fileSize;
  if (fileSize < 1024) {
    return `${fileSize} B`;
  } else if (fileSize < 1024 * 1024) {
    return `${(fileSize / 1024).toFixed(2)} KiB`;
  } else if (fileSize < 1024 * 1024 * 1024) {
    return `${(fileSize / 1024 / 1024).toFixed(2)} MiB`;
  } else if (fileSize < 1024 * 1024 * 1024 * 1024) {
    return `${(fileSize / 1024 / 1024 / 1024).toFixed(2)} GiB`;
  } else {
    return `${(fileSize / 1024 / 1024 / 1024 / 1024).toFixed(2)} TiB`;
  }
});
</script>

<template lang="pug">
| {{ sizeText }}
</template>
