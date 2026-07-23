// SAFE: URL is validated against a known base with URL constructor

export async function generateStaticParams() {
  const base = 'https://api.internal/trusted'
  const url = new URL('/products', base)
  const res = await fetch(url)
  const products: { id: string }[] = await res.json()
  return products.map((p) => ({ id: p.id }))
}

export default function ProductPage({ params }: { params: { id: string } }) {
  return <div>Product {params.id}</div>
}
