// SAFE: errors are logged with context before providing a fallback

async function loadUserProfile(userId: string) {
  try {
    const response = await fetch(`/api/users/${userId}`);
    if (!response.ok) {
      throw new Error(`API error: ${response.status}`);
    }
    return response.json();
  } catch (err) {
    console.error('Failed to load user profile:', userId, err);
    return { name: 'Unknown' };
  }
}
