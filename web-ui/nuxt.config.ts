// https://nuxt.com/docs/api/configuration/nuxt-config
const srcDir = ".";

export default defineNuxtConfig({
  modules: ["@nuxtjs/tailwindcss"],
  runtimeConfig: {
    public: {
      apiBaseUrl: "http://localhost:9999/api/v1",
    },
  },

  tailwindcss: {
    config: {
      theme: {
        fontFamily: {
          sans: [`Inter`],
        },
      },
      content: [
        `${srcDir}/components/**/*.{vue,js,ts}`,
        `${srcDir}/layouts/**/*.vue`,
        `${srcDir}/pages/**/*.vue`,
        `${srcDir}/composables/**/*.{js,ts}`,
        `${srcDir}/plugins/**/*.{js,ts}`,
        `${srcDir}/App.{js,ts,vue}`,
        `${srcDir}/app.{js,ts,vue}`,
        `${srcDir}/Error.{js,ts,vue}`,
        `${srcDir}/error.{js,ts,vue}`,
      ],
    },
  },
});
