// SAFE: errors are wrapped in a typed Result and propagated to the caller

type ApiResult<T> = { ok: true; data: T } | { ok: false; error: string };

async function loadUserProfile(userId: string): Promise<ApiResult<{ name: string }>> {
  try {
    const response = await fetch(`/api/users/${userId}`);
    if (!response.ok) {
      return { ok: false, error: `HTTP ${response.status}` };
    }
    const data = await response.json();
    return { ok: true, data };
  } catch (err) {
    return { ok: false, error: err instanceof Error ? err.message : 'Unknown error' };
  }
}
