// [frensense]
// observation: A state value is accessed via array index before being used inside useEffect with empty deps.
// impact: Stale closure — array captures initial value.
// improvement: Include the array-source in deps or use ref.
import { useEffect, useState } from 'react';
export function Counter() {
  const [count, setCount] = useState(0);
  const [step, setStep] = useState(1);
  const s = [step][0];
  useEffect(() => {
    const timer = setInterval(() => setCount(c => c + s), 1000);
    return () => clearInterval(timer);
  }, []);
  return <div>{count}</div>;
}
