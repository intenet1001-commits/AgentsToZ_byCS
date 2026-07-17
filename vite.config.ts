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
    port: Number(process.env.PORT) || 9000,
    // 9000 점유 시 9001/9002로 조용히 폴백하며 고아 vite 서버가 쌓이는 것 방지 — 즉시 실패
    strictPort: true,
    watch: {
      ignored: ['**/src-tauri/**', '**/dist/**'],
    },
    proxy: {
      '/api': {
        // api-server.ts의 리스닝 포트와 동일한 API_PORT 환경변수를 읽어야
        // 워크트리 인스턴스에서 API_PORT를 오버라이드해도 프록시가 올바른 인스턴스로 향한다
        target: `http://localhost:${Number(process.env.API_PORT) || 3001}`,
        changeOrigin: true,
      },
    },
  },
});
