// https://nuxt.com/docs/api/configuration/nuxt-config
export default defineNuxtConfig({
  devtools: { enabled: true },
  modules: [
    '@vueuse/nuxt', '@nuxtjs/google-fonts',
  ],
  googleFonts: {
    families: { 
      'Nunito': [400, 700],
      'Vast Shadow': 400,
    }
  },
  css: ['~/styles/main.scss'],
})
