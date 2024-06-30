<template>
  <tr>
    <td> {{ company.id }}</td>
    <td> {{ company.name }}</td>
  </tr>
</template>

<script lang="ts" setup>
import { PropType } from 'vue';
import { Company } from '../models/company';

defineProps({
  company: {
    type: Object as PropType<Company>,
    required: true
  }
})
</script>
