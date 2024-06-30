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

<script lang="ts">
import { Component, Vue } from 'vue-property-decorator';
import UserTableRow from './UserTableRow.vue';
import { users } from '../models/user';

@Component({
  components: {
    UserTableRow
  }
})
export default class ClassStyleComponent extends Vue {

  get users() {
    return users;
  }
}
</script>
