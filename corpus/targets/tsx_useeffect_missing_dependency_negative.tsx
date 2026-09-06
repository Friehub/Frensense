// SAFE: All reactive values used in the effect are included in the dependency array

import { useEffect, useState } from 'react';

export function Counter() {
  const [count, setCount] = useState(0);
  const [step, setStep] = useState(1);

  useEffect(() => {
    const timer = setInterval(() => {
      setCount(c => c + step);
    }, 1000);
    return () => clearInterval(timer);
  }, [step]);

  return <div>{count}</div>;
}
