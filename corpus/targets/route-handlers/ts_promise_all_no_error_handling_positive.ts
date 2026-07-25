// [frensense]
// observation: Promise.all() used without a .catch() handler or try/catch around it. If any promise rejects, the entire Promise.all rejects immediately.
// impact: A single failing parallel operation causes the entire batch to fail without partial handling. The unhandled rejection may crash the process in Node.js 15+.
// improvement: Add .catch() to individual promises or wrap in try/catch with Promise.allSettled() for partial success handling.
// cwe: CWE-209
// cvss: 4.3
// owasp: A05:2021
// severity: Medium

async function fetchUsers(userIds: string[]): Promise<User[]> {
  // VULNERABLE: if one fetch fails, all results lost
  return Promise.all(userIds.map(id => fetch(`/api/user/${id}`).then(r => r.json())));
}

app.get('/dashboard', async (req, res) => {
  const [profile, notifications, settings] = await Promise.all([
    fetchProfile(req.user.id),
    fetchNotifications(req.user.id),
    fetchSettings(req.user.id),
  ]);
  // VULNERABLE: no error handling on parallel fetches
  res.json({ profile, notifications, settings });
});
