import { createApp } from "vue";
import App from "./App.vue";
import router from "@/routers/Router";

import "animate.css";
import "@/styles/ResetCSS.css";
import "@/styles/DarkSkin.css";

const app = createApp(App);

app.use(router);

app.mount("#app");

