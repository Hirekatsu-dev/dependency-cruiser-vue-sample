<template>
  <table>
    <thead>
      <tr>
        <th>ID</th>
        <th>名前</th>
      </tr>
    </thead>
    <tbody>
      <UserTableRow
        v-for="user in users"
        :key="user.id"
        :user="user"
      />
    </tbody>
  </table>
</template>

<script lang="ts" setup>
import { users } from '../models/user';
</script>
