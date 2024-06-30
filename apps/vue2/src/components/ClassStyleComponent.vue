<template>
  <div style="background-color: #a0ffff;">
    <h1>ClassStyleComponent</h1>
    <div>{{ message }}</div>
    <button @click="countup">
      カウント
    </button>
    <button @click="init">
      リセット
    </button>
    <TableSection />
  </div>
</template>

<script lang="ts">
import { Component, Prop, Vue } from 'vue-property-decorator';
import TableSection from './TableSection.vue'
@Component({
  components: {
    TableSection
  }
})
export default class ClassStyleComponent extends Vue {
  @Prop()
  private message!: string;

  saySomething(): string {
    const something = 'something';
    return something;
  }

  countup() {
    this.$store.dispatch('increment');
  }

  init() {
    this.$store.dispatch('init');
  }
}
</script>
