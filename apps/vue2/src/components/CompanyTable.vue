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

<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import CompanyTableRow from './CompanyTableRow.vue';
import { companies } from '../models/company';

@Component({
  components: {
    CompanyTableRow
  }
})
export default class ClassStyleComponent extends Vue {

  get companies() {
    return companies;
  }
}
</script>
