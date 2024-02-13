// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  devtools: { enabled: true },
  modules: [
    "@nuxt/ui",
    "@nuxtjs/tailwindcss",
    "@tresjs/nuxt",
    "@vueuse/nuxt",
    "nuxt-icon",
  ],
});
