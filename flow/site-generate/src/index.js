import init, { start } from "./wiki.js";

(async () => {
  await init();
  start();
})();
