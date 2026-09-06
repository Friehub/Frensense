// SAFE: ref pattern avoids stale closure without deps on step
import { useEffect, useState, useRef } from 'react';
export function Counter() {
  const [count, setCount] = useState(0);
  const [step, setStep] = useState(1);
  const stepRef = useRef(step);
  stepRef.current = step;
  useEffect(() => {
    const timer = setInterval(() => setCount(c => c + stepRef.current), 1000);
    return () => clearInterval(timer);
  }, []);
  return <div>{count}</div>;
}
