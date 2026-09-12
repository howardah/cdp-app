import { createApp } from "vue";
import "./styles.css";
import "./process.css";
import App from "./App.vue";
import { router } from "./router";
import ui from "@nuxt/ui/vue-plugin";

createApp(App).use(router).use(ui).mount("#app");
