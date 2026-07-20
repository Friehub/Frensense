// SAFE: uses a discriminated Result union type that forces callers to check

type Result<T, E = Error> =
  | { success: true; data: T }
  | { success: false; error: E };

async function fetchUserData(userId: string): Promise<Result<{ name: string }>> {
  const response = await fetch(`/api/users/${userId}`);
  if (!response.ok) return { success: false, error: new Error('Failed to fetch') };
  const data = await response.json();
  return { success: true, data };
}

async function displayUser(userId: string) {
  const result = await fetchUserData(userId);
  if (!result.success) {
    return { name: 'Unknown', error: result.error.message };
  }
  return { name: result.data.name.toUpperCase() };
}
