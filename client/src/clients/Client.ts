const checkServerConnection = async () => {
  const response = await fetch('/server/health')
  if (!response.ok) {
    throw new Error('Failed to connect')
  }
  return response
}

const fetchArtistMetadata = async (artist_name: string) => {
  return await fetch(`/server/artist/metadata?artist=${artist_name}`)
}

const fetchSimilarArtists = async (artist_mbid: string) => {
  return await fetch(`/server/artist/similar?artist=${artist_mbid}`)
}

const fetchArtistAlbums = async (artist_mbid: string) => {
  return await fetch(`/server/artist/albums?artist=${artist_mbid}`)
}

export default { checkServerConnection, fetchSimilarArtists }
