// [frensense]
// observation: User-controlled input is used to construct Tailwind arbitrary animation values (`animate-[spin_${duration}s]`, `animate-[pulse_${duration}s]`). An attacker can supply extremely small or large durations, or inject additional animation properties that cause excessive CPU usage, rapid animation, or browser freezes.
// impact: An attacker can cause a denial of service (DoS) by injecting an arbitrary animation that triggers rapid or infinite re-rendering, consumes excessive CPU resources, or causes the browser tab to become unresponsive. This can be used to degrade the user experience, drain battery, or crash the browser tab.
// improvement: Never use user input directly in arbitrary animation values. Use predefined animation names with fixed, validated parameters. Cap animation durations to a safe range if user-controlled timing is absolutely necessary.
// cwe: CWE-400
// cvss: 7.5
// owasp: 
// severity: High

'use client';

import { useSearchParams } from 'next/navigation';

export function LoadingSpinner() {
  const searchParams = useSearchParams();
  const speed = searchParams.get('speed') ?? '1';

  return (
    <div className="flex items-center justify-center p-8">
      <div
        className={`w-8 h-8 border-4 border-blue-500 border-t-transparent rounded-full animate-[spin_${speed}s_linear_infinite]`}
      />
    </div>
  );
}
