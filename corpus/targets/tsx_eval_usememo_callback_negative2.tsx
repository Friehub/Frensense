// SAFE: Uses a config-driven approach with a factory function instead of dynamic eval.

import { useMemo } from 'react';

type BinaryOp = (a: number, b: number) => number;

const OPS: Record<string, BinaryOp> = {
  add: (a, b) => a + b,
  subtract: (a, b) => a - b,
  multiply: (a, b) => a * b,
  divide: (a, b) => a / b,
};

interface EvalConfig {
  operator: keyof typeof OPS;
  operands: number[];
}

export function SafeEval({ config }: { config: EvalConfig }) {
  const result = useMemo(() => {
    const op = OPS[config.operator];
    if (!op) return NaN;
    return config.operands.reduce(op);
  }, [config]);

  return <div>Result: {result}</div>;
}
