// [frensense]
// observation: A fetch call inside a useEffect hook does not use an AbortController. When the component unmounts or navigates away, the in-flight request continues and may call setState on an unmounted component.
// impact: Fast navigation between pages triggers stale response handlers that update unmounted component state, causing memory leaks, React warnings, and race-condition bugs with stale data appearing in the wrong view.
// improvement: Create an AbortController, pass its signal to fetch, and abort on cleanup.

import { useEffect, useState } from 'react';

interface Product {
  id: string;
  name: string;
  price: number;
}

export function ProductList({ category }: { category: string }) {
  const [products, setProducts] = useState<Product[]>([]);

  useEffect(() => {
    fetch(`/api/products?category=${encodeURIComponent(category)}`)
      .then((res) => res.json())
      .then((data) => setProducts(data));
  }, [category]);

  return (
    <ul>
      {products.map((p) => (
        <li key={p.id}>{p.name} — ${p.price}</li>
      ))}
    </ul>
  );
}
