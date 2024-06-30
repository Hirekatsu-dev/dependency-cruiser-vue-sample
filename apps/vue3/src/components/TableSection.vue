<template>
  <div>
    <select v-model="table">
      <option value="UserTable">UserTable</option>
      <option value="CompanyTable">CompanyTable</option>
    </select>
    <component
      :is="table === 'UserTable' ? UserTable : CompanyTable"
    />
  </div>
</template>

<script lang="ts" setup>
import UserTable from './UserTable.vue';
import CompanyTable from './CompanyTable.vue';

const table: 'UserTable' | 'CompanyTable' = 'UserTable';
</script>
