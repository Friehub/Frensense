// SAFE: Replace eval with a pure function or lookup table that computes the same result safely.

import { useMemo } from 'react';

const FORMATTERS: Record<string, (v: number) => string> = {
  double: (v) => String(v * 2),
  triple: (v) => String(v * 3),
  squared: (v) => String(v * v),
  half: (v) => String(v / 2),
};

export function ExpressionEval({ expr }: { expr: string }) {
  const result = useMemo(() => {
    return computeExpression(expr);
  }, [expr]);

  return <div>Result: {result}</div>;
}

function computeExpression(expr: string): number {
  const [a, op, b] = expr.split(' ');
  const left = parseFloat(a);
  const right = parseFloat(b);
  switch (op) {
    case '+': return left + right;
    case '-': return left - right;
    case '*': return left * right;
    case '/': return left / right;
    default: return NaN;
  }
}

export function Formatter({ code, value }: { code: string; value: number }) {
  const format = (v: number) => {
    const fn = FORMATTERS[code];
    return fn ? fn(v) : String(v);
  };

  return <span>{format(value)}</span>;
}
