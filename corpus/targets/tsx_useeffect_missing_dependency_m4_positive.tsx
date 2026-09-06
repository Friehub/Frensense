// [frensense]
// observation: A helper function returns a state-derived value that is used inside useEffect with empty deps.
// impact: Stale closure — the helper return is captured at initial render.
// improvement: Include the return value in the dependency array.
import { useEffect, useState } from 'react';
function getStep(s: number): number { return s; }
export function Counter() {
  const [count, setCount] = useState(0);
  const [step, setStep] = useState(1);
  const s = getStep(step);
  useEffect(() => {
    const timer = setInterval(() => setCount(c => c + s), 1000);
    return () => clearInterval(timer);
  }, []);
  return <div>{count}</div>;
}
