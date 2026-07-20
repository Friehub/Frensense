// SAFE: Uses predefined Framer Motion variants instead of dynamic animate objects, preventing property injection entirely

'use client';

import { motion } from 'framer-motion';

const variants = {
  hidden: { opacity: 0, x: -100, scale: 0.8 },
  visible: { opacity: 1, x: 0, scale: 1, transition: { duration: 0.5 } },
  highlighted: { opacity: 1, x: 20, scale: 1.1, backgroundColor: '#ffeb3b', transition: { duration: 0.3 } },
};

type BannerVariant = keyof typeof variants;

export function AnimatedBanner({ variant = 'visible' }: { variant?: BannerVariant }) {
  const resolvedVariant = variant in variants ? variant : 'visible';

  return (
    <motion.div
      variants={variants}
      initial="hidden"
      animate={resolvedVariant}
      style={{ width: 200, height: 100, background: '#fff' }}
    >
      Animated Banner
    </motion.div>
  );
}
