import { createApp } from 'vue'
import './style.css'
import App from './App.vue'
import client from './clients/Client'

createApp(App).mount('#app')

client.checkServerConnection()
