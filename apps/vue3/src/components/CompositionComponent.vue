<template>
  <div style="background-color: #a0ffff;">
    <h1>CompositionComponent</h1>
    <div>{{ message }}</div>
    <TableSection />
  </div>
</template>

<script lang="ts" setup>
import TableSection from './TableSection.vue'

defineProps({
  message: {
    type: String,
    required: true
  },
})
</script>
