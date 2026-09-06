// SAFE: callback depends only on stable identity by using a ref for the filter value

import { useCallback, useState, useRef } from 'react';

export function ProductList() {
  const [products, setProducts] = useState<string[]>([]);
  const [filter, setFilter] = useState('');
  const filterRef = useRef(filter);
  filterRef.current = filter;

  const filteredProducts = useCallback(() => {
    return products.filter(p => p.includes(filterRef.current));
  }, [products]);

  return <div>{filteredProducts().length} items</div>;
}
