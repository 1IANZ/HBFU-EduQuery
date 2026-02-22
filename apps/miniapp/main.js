// #ifdef VUE3
import { createSSRApp } from "vue";
import App from "./App";
import share from "@/utils/share.js";
import initPermission from "./utils/permission";

export function createApp() {
  const app = createSSRApp(App);
  app.mixin(share);
  initPermission();
  return {
    app,
  };
}
// #endif
