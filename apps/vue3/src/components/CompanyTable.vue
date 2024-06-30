<template>
  <table>
    <thead>
      <tr>
        <th>ID</th>
        <th>名前</th>
      </tr>
    </thead>
    <tbody>
      <CompanyTableRow
        v-for="company in companies"
        :key="company.id"
        :company="company"
      />
    </tbody>
  </table>
</template>

<script lang="ts" setup>
import { companies } from '../models/company';
</script>
