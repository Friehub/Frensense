// SAFE: remove listeners in cleanup
app.get('/subscribe', (req, res) => {
  const handler = (payload: any) => {
    res.write(`data: ${JSON.stringify(payload)}\n\n`);
  };
  eventEmitter.on('data', handler);
  res.writeHead(200, { 'Content-Type': 'text/event-stream' });
  req.on('close', () => {
    eventEmitter.off('data', handler);
  });
});

function mountWidget() {
  window.addEventListener('resize', handleResize);
  window.addEventListener('scroll', handleScroll);
}

function unmountWidget() {
  window.removeEventListener('resize', handleResize);
  window.removeEventListener('scroll', handleScroll);
}
