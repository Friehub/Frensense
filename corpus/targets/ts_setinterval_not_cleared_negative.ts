// SAFE: clear interval on connection close
app.get('/start-polling', (req, res) => {
  res.writeHead(200, { 'Content-Type': 'text/event-stream' });
  const intervalId = setInterval(async () => {
    const notifications = await db.fetchNotifications(req.user.id);
    if (notifications.length > 0) {
      res.write(`data: ${JSON.stringify(notifications)}\n\n`);
    }
  }, 5000);
  req.on('close', () => clearInterval(intervalId));
});

function startAutoSave(docId: string): number {
  const id = setInterval(() => autoSave(docId), 30000);
  return id;
}

function stopAutoSave(id: number) {
  clearInterval(id);
}
