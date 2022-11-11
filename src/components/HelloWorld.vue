<script setup lang="ts">
import { invoke } from "@tauri-apps/api/tauri";
import { ref } from "vue";

defineProps<{ msg: string }>();

const count = ref(0);

async function backendAdd() {
  count.value = await invoke("backend_add", { number: count.value });
}
</script>

<template>
  <div class="w-9/12 mx-auto">
    <h1>{{ msg }}</h1>

    <div class="justify-center my-6 text-center border border-white/10">
      <div class="p-8 text-lg font-bold">Count is: {{ count }}</div>

      <div class="w-full space-x-2">
        <button type="button" class="btn" @click="count++">Add 1</button>

        <button type="button" class="btn" @click="backendAdd">
          Add 2 in backend
        </button>
      </div>
    </div>
  </div>
</template>
