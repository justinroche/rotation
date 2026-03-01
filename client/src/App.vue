<script setup lang="ts">
import { ref, onMounted } from 'vue'
import client from './clients/Client'

const connectionStatus = ref<'loading' | 'connected' | 'failed'>('loading')
const inputtedArtist = ref('')

onMounted(async () => {
  try {
    connectionStatus.value = (await client.checkServerConnection())
      ? 'connected'
      : 'failed'
  } catch {
    connectionStatus.value = 'failed'
  }
})

const handleFetchSimilarArtists = async () => {
  const response = await client.fetchSimilarArtists(inputtedArtist.value)

  if (!response.ok) {
    console.error('Request failed:', response.status)
    return
  }

  const data = await response.json()
  console.log(data)
}
</script>

<template>
  <div>
    <h1>Rotation</h1>
    <div>
      <p v-if="connectionStatus === 'loading'">Checking connection…</p>
      <p v-else-if="connectionStatus === 'connected'">Connected to server</p>
      <p v-else>Failed to connect to server</p>
    </div>
    <div>
      <input v-model="inputtedArtist" type="text" />
      <button @click="handleFetchSimilarArtists">
        Log similar artists to console
      </button>
    </div>
  </div>
</template>

<style scoped></style>
