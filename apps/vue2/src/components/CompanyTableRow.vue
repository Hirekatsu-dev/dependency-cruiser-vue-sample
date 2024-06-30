<template>
  <tr>
    <td> {{ company.id }}</td>
    <td> {{ company.name }}</td>
  </tr>
</template>

<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import { Company } from '../models/company';

@Component
export default class ClassStyleComponent extends Vue {
  @Prop()
  private company!: Company;
}
</script>
