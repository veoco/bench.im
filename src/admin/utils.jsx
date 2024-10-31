async function fetchWithAuth(resource, init) {
  const token = sessionStorage.getItem("token");
  const r = await fetch(resource, {
    headers: {
      'Authorization': `Bearer ${token}`,
      ...init
    }
  })
  if (!r.ok) {
    error.info = await res.json()
    error.status = res.status
    throw error
  }
  return r.json();
}

export default fetchWithAuth;