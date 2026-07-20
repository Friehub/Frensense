// SAFE: uses error.digest to show a reference code while hiding internals

'use client'

export default function GlobalError({ error }: { error: Error & { digest?: string } }) {
  return (
    <html>
      <body>
        <h1>Unexpected error</h1>
        {error.digest && <p>Reference: {error.digest}</p>}
      </body>
    </html>
  )
}
