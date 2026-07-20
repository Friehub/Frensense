// SAFE: only displays a generic message; error details are logged server-side

'use client'

export default function GlobalError({ error }: { error: Error & { digest?: string } }) {
  console.error('Unhandled error:', error)
  return (
    <html>
      <body>
        <h1>Unexpected error</h1>
        <p>Please try again later.</p>
      </body>
    </html>
  )
}
