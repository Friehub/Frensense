// [frensense]
// observation: User-controlled input is interpolated into a Tailwind arbitrary z-index value (`z-[${userInput}]`). An attacker can set an extremely high z-index value to overlay UI elements that are meant to be "unclickable" or "behind" security chrome (e.g., browser security indicators, permission dialogs, or trusted UI overlays). This enables clickjacking and UI redressing attacks.
// impact: An attacker can overlay fake UI elements on top of security indicators, address bars (in PWAs), permission dialogs, or trusted UI regions. This enables clickjacking where the user thinks they are clicking a legitimate element but actually interacts with a malicious overlay. In standalone/installed PWAs, this can overlay the browser's security chrome.
// improvement: Never use user-controlled values for `z-index`. Use a predefined set of z-index constants from a design system or CSS variables. If dynamic z-index is necessary, validate and clamp the value to a safe range (e.g., 1–100).
// cwe: CWE-200
// cvss: 4.3
// owasp: 
// severity: Low

'use client';

import { useSearchParams } from 'next/navigation';

export function OverlayAd() {
  const searchParams = useSearchParams();
  const zLevel = searchParams.get('z') ?? '50';

  return (
    <div
      className={`fixed inset-0 z-[${zLevel}] bg-white/90 flex items-center justify-center`}
    >
      <div className="p-6 bg-white rounded-lg shadow-2xl">
        <h2>Claim your prize!</h2>
        <button className="bg-yellow-400 text-black px-6 py-2 rounded">
          Click Here
        </button>
      </div>
    </div>
  );
}
