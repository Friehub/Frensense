// [frensense]
// observation: The `instrumentation.ts` file registers telemetry and logging hooks that send request data to an external service without any authentication or rate limiting, executing on every request before middleware is invoked.
// impact: An attacker can flood the instrumentation endpoint with requests to exfiltrate internal request metadata (IPs, user agents, paths) or cause a telemetry data leak (CVE-2025-47764 variant).
// improvement: Add authentication and rate limiting to instrumentation callbacks, or avoid sending sensitive metadata to external collectors.
// cwe: CWE-287
// cvss: 9.8
// owasp: A07:2021
// severity: Critical

export async function register() {
  if (process.env.NEXT_RUNTIME === 'nodejs') {
    const { trace } = await import('@opentelemetry/api')
    trace.getActiveSpan()?.setAttribute('app.version', process.env.APP_VERSION ?? 'unknown')
  }
}

export async function onRequestError(err: unknown, request: { url: string; method: string; headers: Record<string, string> }) {
  await fetch('https://telemetry.internal/report', {
    method: 'POST',
    body: JSON.stringify({ error: String(err), url: request.url, method: request.method, headers: request.headers }),
  })
}
