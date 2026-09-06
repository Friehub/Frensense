// [frensense]
// observation: A state value is concatenated before being used inside useEffect with empty deps.
// impact: Stale closure — captures initial value via the concatenation chain.
// improvement: Include source state in the dependency array.
import { useEffect, useState } from 'react';
export function Counter() {
  const [count, setCount] = useState(0);
  const [step, setStep] = useState(1);
  const s = 0 + step;
  useEffect(() => {
    const timer = setInterval(() => setCount(c => c + s), 1000);
    return () => clearInterval(timer);
  }, []);
  return <div>{count}</div>;
}
