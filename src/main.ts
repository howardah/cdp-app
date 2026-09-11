import { createApp } from "vue";
import "./styles.css";
import "./process.css";
import "./process-overrides.css";
import App from "./App.vue";
import { router } from "./router";

createApp(App).use(router).mount("#app");
