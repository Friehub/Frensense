// SAFE: dependency array includes the last-hop variable
import { useEffect, useState } from 'react';
export function Counter() {
  const [count, setCount] = useState(0);
  const [step, setStep] = useState(1);
  const a = step;
  const b = a;
  useEffect(() => {
    const timer = setInterval(() => setCount(c => c + b), 1000);
    return () => clearInterval(timer);
  }, [b]);
  return <div>{count}</div>;
}
