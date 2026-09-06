// SAFE: Telemetry reporting uses an internal authentication token and redacts sensitive headers before sending

export async function register() {
  if (process.env.NEXT_RUNTIME === 'nodejs') {
    const { trace } = await import('@opentelemetry/api')
    trace.getActiveSpan()?.setAttribute('app.version', process.env.APP_VERSION ?? 'unknown')
  }
}

function redactSensitiveHeaders(headers: Record<string, string>): Record<string, string> {
  const SENSITIVE = new Set(['authorization', 'cookie', 'x-api-key', 'set-cookie'])
  const safe: Record<string, string> = {}
  for (const [key, value] of Object.entries(headers)) {
    safe[key] = SENSITIVE.has(key.toLowerCase()) ? '[REDACTED]' : value
  }
  return safe
}

export async function onRequestError(err: unknown, request: { url: string; method: string; headers: Record<string, string> }) {
  await fetch('https://telemetry.internal/report', {
    method: 'POST',
    headers: { Authorization: `Bearer ${process.env.TELEMETRY_TOKEN ?? ''}` },
    body: JSON.stringify({ error: String(err), url: request.url, method: request.method, headers: redactSensitiveHeaders(request.headers) }),
  })
}
