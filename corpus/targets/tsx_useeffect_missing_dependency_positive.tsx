// [frensense]
// observation: A useEffect hook has an incomplete dependency array, causing it to capture stale state values.
// impact: The effect runs with out-of-date values from an earlier render, leading to incorrect behavior or data loss.
// improvement: Include all reactive values used inside the effect in the dependency array.

import { useEffect, useState } from 'react';

export function Counter() {
  const [count, setCount] = useState(0);
  const [step, setStep] = useState(1);

  useEffect(() => {
    const timer = setInterval(() => {
      setCount(c => c + step);
    }, 1000);
    return () => clearInterval(timer);
  }, []);

  return <div>{count}</div>;
}
