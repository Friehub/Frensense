// [frensense]
// observation: A state value flows through two assignments before being used inside useEffect with empty deps.
// impact: Stale closure — multi-hop variable captures initial value only.
// improvement: Include the source state in the dependency array.
import { useEffect, useState } from 'react';
export function Counter() {
  const [count, setCount] = useState(0);
  const [step, setStep] = useState(1);
  const a = step;
  const b = a;
  useEffect(() => {
    const timer = setInterval(() => setCount(c => c + b), 1000);
    return () => clearInterval(timer);
  }, []);
  return <div>{count}</div>;
}
