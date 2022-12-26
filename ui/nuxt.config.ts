const srcDir = `.`;

export default defineNuxtConfig({
    runtimeConfig: {
        public: {
            apiBase: 'http://localhost:8080/api'
        }
    },
    modules: ['@nuxtjs/tailwindcss'],
    tailwindcss: {
        config: {
            theme: {
                fontFamily: {
                    sans: [`Inter`]
                }
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
                `${srcDir}/error.{js,ts,vue}`
            ],
        }
    }

})
