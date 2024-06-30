<template>
  <div>
    <select v-model="table">
      <option value="UserTable">UserTable</option>
      <option value="CompanyTable">CompanyTable</option>
    </select>
    <component
      :is="table"
    />
  </div>
</template>

<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import { Company } from '../models/company';
import UserTable from './UserTable.vue';
import CompanyTable from './CompanyTable.vue';

@Component({
  components: {
    UserTable,
    CompanyTable
  }
})
export default class ClassStyleComponent extends Vue {
  table: 'UserTable' | 'CompanyTable' = 'UserTable';
}
</script>
