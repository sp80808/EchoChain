import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';

// https://vitejs.dev/config/
export default defineConfig({
  plugins: [react()],
  base: './', // Ensure assets are loaded correctly in Electron
  build: {
    outDir: 'dist', // Output to the 'dist' folder within the App directory
  },
  optimizeDeps: {
    exclude: ['lucide-react'],
  },
});
