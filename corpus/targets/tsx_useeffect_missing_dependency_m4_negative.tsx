// SAFE: dependency array includes the helper return value
import { useEffect, useState } from 'react';
function getStep(s: number): number { return s; }
export function Counter() {
  const [count, setCount] = useState(0);
  const [step, setStep] = useState(1);
  const s = getStep(step);
  useEffect(() => {
    const timer = setInterval(() => setCount(c => c + s), 1000);
    return () => clearInterval(timer);
  }, [s]);
  return <div>{count}</div>;
}
