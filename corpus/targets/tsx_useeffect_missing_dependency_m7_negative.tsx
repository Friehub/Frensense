// SAFE: destructured value is in the dependency array
import { useEffect, useState } from 'react';
export function Counter() {
  const [count, setCount] = useState(0);
  const [step, setStep] = useState(1);
  const { val: s } = { val: step };
  useEffect(() => {
    const timer = setInterval(() => setCount(c => c + s), 1000);
    return () => clearInterval(timer);
  }, [s]);
  return <div>{count}</div>;
}
