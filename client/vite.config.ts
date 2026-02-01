import { defineConfig } from 'vite'
import vue from '@vitejs/plugin-vue'
import {
  client_host,
  client_port,
} from './src/config/config.json'

// https://vite.dev/config/
export default defineConfig({
  plugins: [vue()],
  server: {
    host: client_host,
    port: client_port,
  },
})
