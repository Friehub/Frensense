// SAFE: Instrumentation only logs to stdout without sending data to external services

export async function register() {
  if (process.env.NEXT_RUNTIME === 'nodejs') {
    const { trace } = await import('@opentelemetry/api')
    trace.getActiveSpan()?.setAttribute('app.version', process.env.APP_VERSION ?? 'unknown')
  }
}

export async function onRequestError(err: unknown, request: { url: string; method: string }) {
  console.error(`[instrumentation] ${request.method} ${request.url}: ${err}`)
}
