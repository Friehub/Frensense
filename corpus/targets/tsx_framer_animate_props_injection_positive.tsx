// [frensense]
// observation: Framer Motion `animate` props are constructed from user-controlled input (e.g., URL params, API responses), allowing an attacker to inject arbitrary animation properties including `x`, `y`, `scale`, `rotate`, or even `transition` configuration that can manipulate the DOM position or cause denial-of-service via expensive animations.
// impact: An attacker can inject properties that move elements off-screen, cover critical UI, set extreme scale/rotate values causing layout thrashing, or inject `transition` configs that cause performance degradation. In older browsers, prototype pollution via `style` injection can also be exploited.
// improvement: Validate animation props against an allowlist of permitted properties, or use predefined animation variants instead of dynamic animate objects.

'use client';

import { motion } from 'framer-motion';
import { useSearchParams } from 'next/navigation';

export function AnimatedBanner() {
  const searchParams = useSearchParams();

  const animateProps = {
    x: Number(searchParams.get('x')) || 0,
    y: Number(searchParams.get('y')) || 0,
    scale: Number(searchParams.get('scale')) || 1,
    rotate: Number(searchParams.get('rotate')) || 0,
    backgroundColor: searchParams.get('color') || '#fff',
  };

  return (
    <motion.div
      animate={animateProps}
      transition={{ type: 'spring', stiffness: 300, damping: 20 }}
      style={{ width: 200, height: 100 }}
    >
      Animated Banner
    </motion.div>
  );
}
