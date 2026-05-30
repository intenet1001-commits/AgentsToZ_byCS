import { defineConfig } from 'vite';
import react from '@vitejs/plugin-react';
import tailwindcss from 'tailwindcss';

export default defineConfig({
  plugins: [react()],
  css: {
    postcss: {
      plugins: [tailwindcss],
    },
  },
  optimizeDeps: {
    include: ['@supabase/supabase-js', '@supabase/realtime-js'],
  },
  build: {
    rollupOptions: {
      output: {
        // 벤더 라이브러리를 안정적인 별도 청크로 분리 — lazy 청크(SetupWizard 등)가 벤더를 중복 포함하지 않게 하고 캐싱 granularity 확보
        manualChunks: {
          'react-vendor': ['react', 'react-dom'],
          'supabase': ['@supabase/supabase-js', '@supabase/realtime-js'],
          'icons': ['lucide-react'],
        },
      },
    },
  },
  server: {
    port: 9000,
    watch: {
      ignored: ['**/src-tauri/**', '**/dist/**'],
    },
    proxy: {
      '/api': {
        target: 'http://localhost:3001',
        changeOrigin: true,
      },
    },
  },
});
