<template>
  <div class="home">
    <img alt="Vue logo" src="../assets/logo.png">
    <HelloWorld msg="Welcome to Your Vue.js + TypeScript App"/>
    <ClassStyleComponent message="hello" />
    <div>{{ JSON.stringify(count) }}</div>
  </div>
</template>

<script lang="ts">
import HelloWorld from '@/components/HelloWorld.vue'; // @ is an alias to /src
import ClassStyleComponent from '@/components/ClassStyleComponent.vue'; // @ is an alias to /src
import { mapGetters } from 'vuex';
export default {
  components: {
    HelloWorld,
    ClassStyleComponent,
  },
  computed: {
    ...mapGetters(['count'])
  }
};
</script>
