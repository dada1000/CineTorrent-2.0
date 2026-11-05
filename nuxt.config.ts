export default defineNuxtConfig({
  ssr: false,
  devtools: { enabled: false },
  app: {
    head: {
      title: 'CineTorrent Next'
    }
  },
  devServer: {
    port: 3000
  },
  nitro: {
    preset: 'static'
  },
  vite: {
    server: {
      watch: {
        ignored: ['**/src-tauri/**']
      }
    }
  }
})