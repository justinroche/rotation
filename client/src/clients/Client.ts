const checkServerConnection = async () => {
  const response = await fetch('/server/health')
  if (!response.ok) {
    throw new Error('Failed to connect')
  }
}

export default { checkServerConnection }
