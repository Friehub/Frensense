// [frensense]
// observation: A state value flows through a template literal before being used in useEffect with empty deps.
// impact: Stale closure — the numeric conversion captures initial value.
// improvement: Include the source state in deps array.
import { useEffect, useState } from 'react';
export function Counter() {
  const [count, setCount] = useState(0);
  const [step, setStep] = useState(1);
  const s = Number(`${step}`);
  useEffect(() => {
    const timer = setInterval(() => setCount(c => c + s), 1000);
    return () => clearInterval(timer);
  }, []);
  return <div>{count}</div>;
}
