// SAFE: dependency array includes the template-literal-derived value
import { useEffect, useState } from 'react';
export function Counter() {
  const [count, setCount] = useState(0);
  const [step, setStep] = useState(1);
  const s = Number(`${step}`);
  useEffect(() => {
    const timer = setInterval(() => setCount(c => c + s), 1000);
    return () => clearInterval(timer);
  }, [s]);
  return <div>{count}</div>;
}
