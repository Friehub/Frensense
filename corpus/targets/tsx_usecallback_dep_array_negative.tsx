// SAFE: all reactive dependencies (products, filter) are included in the dependency array

import { useCallback, useState } from 'react';

export function ProductList() {
  const [products, setProducts] = useState<string[]>([]);
  const [filter, setFilter] = useState('');

  const filteredProducts = useCallback(() => {
    return products.filter(p => p.includes(filter));
  }, [products, filter]);

  return <div>{filteredProducts().length} items</div>;
}
