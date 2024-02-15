// https://nuxt.com/docs/api/configuration/nuxt-config
import wasmPack from "vite-plugin-wasm-pack";

export default defineNuxtConfig({
  devtools: { enabled: true },
  modules: [
    "@nuxt/ui",
    "@nuxtjs/tailwindcss",
    "@tresjs/nuxt",
    "@vueuse/nuxt",
    "nuxt-icon",
  ],
  vite: {
    plugins: [wasmPack("./attractors")],
    optimizeDeps: {
      exclude: ["attractors"],
    },
  },
});
