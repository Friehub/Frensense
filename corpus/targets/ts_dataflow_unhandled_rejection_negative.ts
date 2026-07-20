// SAFE: global unhandledRejection handler ensures all promise rejections are logged

process.on('unhandledRejection', (reason, promise) => {
  console.error('Unhandled Rejection at:', promise, 'reason:', reason);
});

async function runWorker() {
  const data = await fetch('https://api.example.com/data');
  const json = await data.json();
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
