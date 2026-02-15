import { defineConfig, loadEnv } from 'vite'
import vue from '@vitejs/plugin-vue'

export default defineConfig(({ mode }) => {
  const env = loadEnv(mode, process.cwd())

  return {
    plugins: [vue()],
    server: {
      host: true,
      port: 3000,
      proxy: {
        '/server': {
          target: `${env.VITE_SERVER_ORIGIN}`,
          changeOrigin: true,
          rewrite: (path) => path.replace(/^\/server/, ''),
        },
      },
    },
  }
})
