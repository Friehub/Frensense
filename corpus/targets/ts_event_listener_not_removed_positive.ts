// [frensense]
// observation: Event listeners added repeatedly (e.g., in a React useEffect or request handler) without being removed. Each invocation adds a new listener.
// impact: Listener count grows unbounded, causing memory leaks. Each listener also executes on every event, leading to duplicate side effects and degraded performance.
// improvement: Always clean up event listeners in a teardown function (removeEventListener in useEffect return, or .off() on EventEmitter).

app.get('/subscribe', (req, res) => {
  // VULNERABLE: listener added but never removed
  eventEmitter.on('data', (payload) => {
    res.write(`data: ${JSON.stringify(payload)}\n\n`);
  });
  res.writeHead(200, { 'Content-Type': 'text/event-stream' });
});

function mountWidget() {
  // VULNERABLE: each mount adds a new listener
  window.addEventListener('resize', handleResize);
  window.addEventListener('scroll', handleScroll);
}

function handleResize() { /* update layout */ }
function handleScroll() { /* lazyload images */ }
