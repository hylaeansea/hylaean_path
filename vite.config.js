import { defineConfig } from 'vite';

export default defineConfig({
  base: '/hylaean_path/', // Replace <REPO_NAME> with the name of your GitHub repository
  build: {
    outDir: 'dist', // Output directory for the build
    target: 'esnext', // Target modern browsers
  },
  server: {
    open: true, // Opens the browser on `npm run dev`
  },
});