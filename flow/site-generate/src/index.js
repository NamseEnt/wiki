import init, { start } from "/wiki.js";

(async () => {
  await init();
  const title = window.location.pathname
    .split("/")
    .pop()
    .split(".")
    .slice(0, -1)
    .join(".");

  start(title);
})();
