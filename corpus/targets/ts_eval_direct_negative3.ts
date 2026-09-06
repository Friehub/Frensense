// SAFE: validates expression against a set of safe operators before evaluating
const SAFE_OPERATORS = ['+', '-', '*', '/'];

function isSafeExpression(expr: string): boolean {
  let foundOp = false;
  for (const op of SAFE_OPERATORS) {
    if (expr.includes(op)) {
      foundOp = true;
      break;
    }
  }
  if (!foundOp) return false;
  const cleaned = expr.replace(/[\d\s+\-*/.]/g, '');
  return cleaned.length === 0;
}

export function calculate(expression: string): number {
  if (!isSafeExpression(expression)) {
    throw new Error('Unsafe expression');
  }
  return Function(`"use strict"; return (${expression})`)();
}
