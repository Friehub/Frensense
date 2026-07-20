// SAFE alternative: recursive setTimeout for self-cancelling
app.get('/start-polling', (req, res) => {
  res.writeHead(200, { 'Content-Type': 'text/event-stream' });
  let cancelled = false;
  async function poll() {
    if (cancelled) return;
    const notifications = await db.fetchNotifications(req.user.id);
    if (notifications.length > 0) {
      res.write(`data: ${JSON.stringify(notifications)}\n\n`);
    }
    setTimeout(poll, 5000);
  }
  poll();
  req.on('close', () => { cancelled = true; });
});
