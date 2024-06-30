<template>
  <tr>
    <td> {{ user.id }}</td>
    <td> {{ user.name }}</td>
  </tr>
</template>

<script lang="ts" setup>
import { User } from '../models/user';
import { PropType } from 'vue';

defineProps({
  user: {
    type: Object as PropType<User>,
    required: true
  }
})
</script>
