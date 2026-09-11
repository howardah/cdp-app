import { createRouter, createWebHashHistory } from "vue-router";
import { h } from "vue";
const routePlaceholder = { render: () => h("div") };

export const router = createRouter({
  history: createWebHashHistory(),
  routes: [
    { path: "/", name: "navigator", component: routePlaceholder },
    { path: "/process/:processId", name: "process", component: routePlaceholder, props: true },
  ],
});
