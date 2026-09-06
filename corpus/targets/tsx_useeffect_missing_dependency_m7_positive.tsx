// [frensense]
// observation: A state value is destructured before being used inside useEffect with empty deps.
// impact: Stale closure — destructured value captures initial render value.
// improvement: Include the destructured value in deps.
import { useEffect, useState } from 'react';
export function Counter() {
  const [count, setCount] = useState(0);
  const [step, setStep] = useState(1);
  const { val: s } = { val: step };
  useEffect(() => {
    const timer = setInterval(() => setCount(c => c + s), 1000);
    return () => clearInterval(timer);
  }, []);
  return <div>{count}</div>;
}
