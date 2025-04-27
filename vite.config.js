import { defineConfig } from 'vite'

export default defineConfig({
  // base: '/hylaean_path/',
  build: {
    outDir: 'dist',
    assetsDir: 'assets',
    rollupOptions: {
      input: {
        main: './index.html',
      },
    }
  }
})