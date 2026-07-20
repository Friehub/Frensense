// SAFE: each async call has its own .catch() handler to prevent unhandled rejections

async function runWorker() {
  const data = await fetch('https://api.example.com/data').catch(err => {
    console.error('Fetch failed:', err);
    throw err;
  });
  const json = await data.json().catch(err => {
    console.error('JSON parse failed:', err);
    throw err;
  });
  processItems(json);
}

function processItems(items: unknown[]) {
  items.forEach(item => {
    if (typeof item === 'object' && item !== null) {
      processItem(item);
    }
  });
}

function processItem(item: unknown) {
  if ('id' in (item as Record<string, unknown>)) {
    throw new Error('Processing failed');
  }
}

runWorker();
