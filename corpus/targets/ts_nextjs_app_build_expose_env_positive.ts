// [frensense]
// observation: The application build output includes client-side bundles that reference `process.env` for non-prefixed private variables, because a misconfigured webpack DefinePlugin or env replacement plugin injects them as compile-time literals.
// impact: Private environment variables such as database URLs, API tokens, and internal service endpoints are exposed in the browser's /_next/static/ JavaScript bundles.
// improvement: Only use `NEXT_PUBLIC_` prefixed environment variables in client-facing code, and audit the build output for any non-prefixed env references.

export default function DashboardPage() {
  return (
    <div>
      <h1>Dashboard</h1>
      <script
        dangerouslySetInnerHTML={{
          __html: `window.__ENV = ${JSON.stringify({
            apiUrl: process.env.API_URL,
            databaseUrl: process.env.DATABASE_URL,
          })}`,
        }}
      />
    </div>
  )
}
