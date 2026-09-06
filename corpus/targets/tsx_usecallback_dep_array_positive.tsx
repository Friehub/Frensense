// [frensense]
// observation: A useCallback hook has an incorrect or incomplete dependency array, causing the callback to capture stale values.
// impact: Child components receive a stale callback reference, leading to incorrect behavior such as using outdated state, missing re-renders, or infinite loops when the callback is used as a dependency elsewhere.
// improvement: Include all reactive values referenced inside the callback in the dependency array, or restructure to minimize dependencies.

import { useCallback, useState } from 'react';

export function ProductList() {
  const [products, setProducts] = useState<string[]>([]);
  const [filter, setFilter] = useState('');

  const filteredProducts = useCallback(() => {
    return products.filter(p => p.includes(filter));
  }, []);

  return <div>{filteredProducts().length} items</div>;
}
