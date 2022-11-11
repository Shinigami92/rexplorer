<script setup lang="ts">
import { invoke } from '@tauri-apps/api/tauri';
import { ref } from 'vue';

defineProps<{ msg: string }>();

const count = ref(0);

async function backendAdd(): Promise<void> {
  count.value = await invoke('backend_add', { num: count.value });
}
</script>

<template lang="pug">
.mx-auto(class="w-9/12")
  h1 {{ msg }}

  .justify-center.my-6.text-center.border(class="border-white/10")
    .p-8.text-lg.font-bold Count is: {{ count }}

    .w-full.space-x-2
      button.btn(type="button", @click="count++") Add 1

      button.btn(type="button", @click="backendAdd") Add 2 in backend
</template>
