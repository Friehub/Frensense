// SAFE: An AbortController is created per effect run, its signal is passed to fetch, and aborted on cleanup.

import { useEffect, useState } from 'react';

interface Product {
  id: string;
  name: string;
  price: number;
}

export function ProductList({ category }: { category: string }) {
  const [products, setProducts] = useState<Product[]>([]);

  useEffect(() => {
    const abortController = new AbortController();

    fetch(`/api/products?category=${encodeURIComponent(category)}`, {
      signal: abortController.signal,
    })
      .then((res) => res.json())
      .then((data) => setProducts(data))
      .catch((err) => {
        if (err instanceof DOMException && err.name === 'AbortError') return;
        console.error('Fetch failed:', err);
      });

    return () => abortController.abort();
  }, [category]);

  return (
    <ul>
      {products.map((p) => (
        <li key={p.id}>{p.name} — ${p.price}</li>
      ))}
    </ul>
  );
}
