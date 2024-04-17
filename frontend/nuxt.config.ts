// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  // devtools: { enabled: true },
  modules: [
    '@vueuse/nuxt', '@nuxtjs/google-fonts',
  ],
  googleFonts: {
    families: { 
      'Fira Code': [400, 700],
      'Sixtyfour': 400,
    }
  },
  css: ['~/styles/main.scss'],
})
