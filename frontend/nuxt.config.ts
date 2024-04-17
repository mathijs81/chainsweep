// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  devtools: { enabled: true },
  modules: [
    '@vueuse/nuxt', '@nuxtjs/google-fonts',
    '@vue-dapp/nuxt',
    [ '@pinia/nuxt', { autoImports: ['defineStore', 'storeToRefs'] }]
  ],
  googleFonts: {
    families: { 
      'Fira Code': [400, 700],
      'Sixtyfour': 400,
    }
  },
  css: ['~/styles/main.scss'],
})
