// [frensense]
// observation: The application does not register a process.on('unhandledRejection') handler, allowing unhandled Promise rejections to silently disappear.
// impact: In Node.js, unhandled rejections cause the process to exit with a non-zero exit code (as of Node 15+), crashing the application. In browsers, the error is silently swallowed, making debugging extremely difficult and leaving the application in an inconsistent state.
// improvement: Register an unhandledRejection handler that logs the error and optionally exits gracefully.

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
