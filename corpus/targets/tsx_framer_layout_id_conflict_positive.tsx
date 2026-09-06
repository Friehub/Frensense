// [frensense]
// observation: Multiple list items share the same `layoutId` value in a Framer Motion `AnimatePresence` + `motion.div` layout animation, causing the wrong element to animate when items are added, removed, or reordered.
// impact: Visual confusion and UI inconsistency — when a user adds/removes items, the wrong element stretches, shrinks, or flies across the screen. In a voting/selection UI, the incorrect item animates as if it was selected, misleading the user about which action was taken.
// improvement: Use unique `layoutId` values per element (e.g., derived from item.id), or avoid `layoutId` for lists where elements have distinct identities.

'use client';

import { useState } from 'react';
import { motion, AnimatePresence } from 'framer-motion';

interface Item {
  id: string;
  text: string;
}

export function RemovableList({ items }: { items: Item[] }) {
  const [list, setList] = useState(items);

  const removeItem = (id: string) => {
    setList((prev) => prev.filter((item) => item.id !== id));
  };

  return (
    <AnimatePresence>
      {list.map((item) => (
        <motion.div
          key={item.id}
          layoutId="list-item"
          exit={{ opacity: 0, x: -100 }}
          style={{ padding: '12px', margin: '4px', background: '#eee' }}
        >
          <span>{item.text}</span>
          <button onClick={() => removeItem(item.id)} style={{ marginLeft: 8 }}>X</button>
        </motion.div>
      ))}
    </AnimatePresence>
  );
}
