// [frensense]
// observation: The eval function is called inside a useMemo or useCallback hook, executing arbitrary code derived from runtime values.
// impact: An attacker who controls any input that reaches the evaluated expression can achieve arbitrary code execution in the client's browser.
// improvement: Remove the eval call entirely. Use pure functions, lookup tables, or factory patterns instead of dynamic code evaluation.

import { useMemo, useCallback } from 'react';

export function ExpressionEval({ expr }: { expr: string }) {
  const result = useMemo(() => {
    return eval(expr);
  }, [expr]);

  return <div>Result: {result}</div>;
}

export function Formatter({ code, value }: { code: string; value: number }) {
  const format = useCallback((v: number) => {
    return eval(code.replace('x', String(v)));
  }, [code]);

  return <span>{format(value)}</span>;
}
