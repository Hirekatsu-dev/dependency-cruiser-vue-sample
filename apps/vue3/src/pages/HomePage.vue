<template>
  <div class="home">
    <img alt="Vue logo" src="../assets/logo.png">
    <HelloWorld msg="Welcome to Your Vue.js + TypeScript App"/>
    <composition-component message="hello" />
  </div>
</template>

<script lang="ts">
import HelloWorld from '@/components/HelloWorld.vue'; // @ is an alias to /src
import CompositionComponent from '@/components/CompositionComponent.vue'; // @ is an alias to /src
export default {
  components: {
    HelloWorld,
    CompositionComponent,
  },
};
</script>
