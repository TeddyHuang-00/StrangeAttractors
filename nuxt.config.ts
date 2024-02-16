// https://nuxt.com/docs/api/configuration/nuxt-config
import wasmPack from "vite-plugin-wasm-pack";

export default defineNuxtConfig({
  app: {
    head: {
      link: [
        {
          rel: "icon",
          sizes: "64x64",
          type: "image/x-icon",
          href: "favicon.ico",
        },
        {
          rel: "icon",
          sizes: "32x32",
          type: "image/png",
          href: "favicon-32x32.png",
        },
        {
          rel: "icon",
          sizes: "16x16",
          type: "image/png",
          href: "favicon-16x16.png",
        },
        {
          rel: "apple-touch-icon",
          sizes: "180x180",
          href: "apple-touch-icon.png",
        },
        {
          rel: "manifest",
          href: "site.webmanifest",
        },
      ],
    },
  },
  devtools: { enabled: true },
  modules: [
    "@nuxt/ui",
    "@nuxtjs/tailwindcss",
    "@tresjs/nuxt",
    "@vueuse/nuxt",
    "nuxt-icon",
    "nuxt-og-image",
  ],
  site: {
    // production URL
    url: "https://teddyhuang-00.github.io/StrangeAttractors/",
  },
  vite: {
    plugins: [wasmPack("./attractors")],
    optimizeDeps: {
      exclude: ["attractors"],
    },
  },
});
