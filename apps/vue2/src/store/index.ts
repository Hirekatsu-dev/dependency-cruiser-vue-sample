import Vue from 'vue';
import Vuex from 'vuex';

Vue.use(Vuex);

interface State {
  count: number;
}

export default new Vuex.Store({
  state: {
    count: 0,
  },
  getters: {
    count: (state: State) => {
      return state.count;
    },
  },
  mutations: {
    increment: (state: State) => {
      state.count += 1;
    },
    init: (state: State) => {
      state.count = 0;
    },
  },
  actions: {
    increment: (ctx) => {
      ctx.commit('increment');
    },
    init: (ctx) => {
      ctx.commit('init');
    },
  },
  modules: {},
});
