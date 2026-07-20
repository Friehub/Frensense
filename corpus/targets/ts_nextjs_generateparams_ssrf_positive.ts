// [frensense]
// observation: `generateStaticParams` fetches data from a user-supplied URL, enabling SSRF
// impact: attacker forces the server to make requests to internal services (metadata, cloud)
// improvement: validate the URL against an allowlist or use a known base URL

export async function generateStaticParams() {
  const source = process.env.DATA_SOURCE_URL!
  const res = await fetch(source)
  const products: { id: string }[] = await res.json()
  return products.map((p) => ({ id: p.id }))
}

export default function ProductPage({ params }: { params: { id: string } }) {
  return <div>Product {params.id}</div>
}
