// [frensense]
// observation: An intermediate variable captures a state value but is missing from useEffect deps.
// impact: Stale closure — the effect uses the captured value from the initial render.
// improvement: Include the intermediate variable's source in the dependency array.
import { useEffect, useState } from 'react';
export function Counter() {
  const [count, setCount] = useState(0);
  const [step, setStep] = useState(1);
  const s = step;
  useEffect(() => {
    const timer = setInterval(() => setCount(c => c + s), 1000);
    return () => clearInterval(timer);
  }, []);
  return <div>{count}</div>;
}
