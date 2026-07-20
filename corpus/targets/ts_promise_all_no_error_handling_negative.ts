// SAFE: try/catch around Promise.all or individual .catch on each promise
async function fetchUsers(userIds: string[]): Promise<User[]> {
  const results = await Promise.allSettled(userIds.map(id =>
    fetch(`/api/user/${id}`).then(r => r.json())
  ));
  return results
    .filter(r => r.status === 'fulfilled')
    .map(r => (r as PromiseFulfilledResult<User>).value);
}

app.get('/dashboard', async (req, res) => {
  try {
    const [profile, notifications, settings] = await Promise.all([
      fetchProfile(req.user.id),
      fetchNotifications(req.user.id),
      fetchSettings(req.user.id),
    ]);
    res.json({ profile, notifications, settings });
  } catch (err) {
    console.error('Dashboard load failed:', err);
    res.status(500).json({ error: 'Failed to load dashboard' });
  }
});
