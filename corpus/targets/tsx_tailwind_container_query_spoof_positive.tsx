// [frensense]
// observation: A component uses Tailwind's container query variants (`@max-sm:`, `@min-md:`) to render different UI content based on the container size. If user input controls the layout or container width, an attacker can manipulate the container size to trigger a different UI state that shows phishing content, fake login forms, or hides security elements.
// impact: An attacker can manipulate container dimensions (via injected styles, resizing, or iframe) to trigger container query breakpoints that swap UI content. This enables phishing attacks where a legitimate-looking form appears only at specific container sizes, or security warnings are hidden at other sizes.
// improvement: Never use container queries to show or hide security-critical content or authentication UI. Use container queries only for cosmetic responsive layout adjustments, and keep security UI consistent across all container sizes.

'use client';

export function PaymentWidget({ containerWidth }: { containerWidth?: string }) {
  return (
    <div
      className="@container"
      style={containerWidth ? { width: containerWidth, maxWidth: '100%' } : undefined}
    >
      <div className="@max-sm:hidden">
        <div className="p-4 bg-green-50 border border-green-200 rounded">
          <h2>Secure Checkout</h2>
          <p>Verified by Trusted Security Provider</p>
        </div>
      </div>
      <div className="hidden @max-sm:block">
        <div className="p-4 bg-white border rounded">
          <h2>Enter your card details</h2>
          <input placeholder="Card number" className="border p-2 w-full" />
          <button className="bg-blue-500 text-white px-4 py-2 mt-2 w-full">
            Pay Now
          </button>
        </div>
      </div>
    </div>
  );
}
