// SAFE: URL is validated against an allowlist before fetching

const ALLOWED_HOSTS = new Set(['api.internal', 'cdn.trusted.com'])

function isAllowed(urlStr: string): boolean {
  try {
    const url = new URL(urlStr)
    return ALLOWED_HOSTS.has(url.hostname)
  } catch {
    return false
  }
}

export async function generateStaticParams() {
  const source = process.env.DATA_SOURCE_URL!
  if (!isAllowed(source)) throw new Error('Invalid data source')
  const res = await fetch(source)
  const products: { id: string }[] = await res.json()
  return products.map((p) => ({ id: p.id }))
}

export default function ProductPage({ params }: { params: { id: string } }) {
  return <div>Product {params.id}</div>
}
