// [frensense]
// observation: setInterval started but never cleared with clearInterval(). Accumulates over time.
// impact: Each request or component mount creates a new interval that runs forever. CPU usage grows linearly, memory accumulates, and HTTP connections leak. Eventually the process runs out of resources.
// improvement: Store the interval ID and clear it in a teardown. Use setTimeout recursion if the interval should only run while the session is active.

app.get('/start-polling', (req, res) => {
  // VULNERABLE: interval never cleared
  setInterval(async () => {
    const notifications = await db.fetchNotifications(req.user.id);
    if (notifications.length > 0) {
      res.write(`data: ${JSON.stringify(notifications)}\n\n`);
    }
  }, 5000);
  res.writeHead(200, { 'Content-Type': 'text/event-stream' });
});

function startAutoSave(docId: string) {
  // VULNERABLE: interval leaks across page navigations
  setInterval(() => {
    autoSave(docId);
  }, 30000);
}
