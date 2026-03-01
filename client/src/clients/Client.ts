const checkServerConnection = async () => {
  const response = await fetch('/server/health')
  if (!response.ok) {
    throw new Error('Failed to connect')
  }
  return response
}

const fetchSimilarArtists = async (artist: string) => {
  return await fetch(`/server/similar_artists?artist=${artist}`)
}

export default { checkServerConnection, fetchSimilarArtists }
