<template>
  <tr>
    <td> {{ user.id }}</td>
    <td> {{ user.name }}</td>
  </tr>
</template>

<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import { User } from '../models/user';

@Component
export default class ClassStyleComponent extends Vue {
  @Prop()
  private user!: User;
}
</script>
