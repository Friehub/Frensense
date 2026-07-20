// SAFE: Animation props are validated against an allowlist of permitted property values before being passed to the motion component

'use client';

import { motion } from 'framer-motion';
import { useSearchParams } from 'next/navigation';

const ALLOWED_COLORS = ['#fff', '#f00', '#00f', '#0f0', '#ff0'];
const MAX_SCALE = 2;
const MAX_ROTATE = 360;
const MAX_OFFSET = 500;

function clamp(value: number, min: number, max: number): number {
  return Math.min(Math.max(value, min), max);
}

export function AnimatedBanner() {
  const searchParams = useSearchParams();

  const rawColor = searchParams.get('color') ?? '#fff';
  const color = ALLOWED_COLORS.includes(rawColor) ? rawColor : '#fff';
  const scale = clamp(Number(searchParams.get('scale')) || 1, 0.1, MAX_SCALE);
  const rotate = clamp(Number(searchParams.get('rotate')) || 0, 0, MAX_ROTATE);
  const x = clamp(Number(searchParams.get('x')) || 0, -MAX_OFFSET, MAX_OFFSET);
  const y = clamp(Number(searchParams.get('y')) || 0, -MAX_OFFSET, MAX_OFFSET);

  return (
    <motion.div
      animate={{ x, y, scale, rotate, backgroundColor: color }}
      transition={{ type: 'spring', stiffness: 300, damping: 20 }}
      style={{ width: 200, height: 100 }}
    >
      Animated Banner
    </motion.div>
  );
}
