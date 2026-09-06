// SAFE: Each list item has a unique layoutId based on the item.id, so layout animations always target the correct element

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
          layoutId={`list-item-${item.id}`}
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
