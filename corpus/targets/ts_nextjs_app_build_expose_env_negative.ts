// SAFE: Only NEXT_PUBLIC_ prefixed environment variables are exposed to the client; private vars are never sent

export default function DashboardPage() {
  return (
    <div>
      <h1>Dashboard</h1>
      <script
        dangerouslySetInnerHTML={{
          __html: `window.__PUBLIC_ENV = ${JSON.stringify({
            apiUrl: process.env.NEXT_PUBLIC_API_URL,
          })}`,
        }}
      />
    </div>
  )
}
