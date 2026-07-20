// SAFE: Container queries are only used for cosmetic layout adjustments (padding, font size). Security indicators and payment forms are shown consistently at all container sizes, ensuring the user always sees the same trusted UI.

'use client';

export function PaymentWidget() {
  return (
    <div className="@container">
      <div className="p-4 bg-green-50 border border-green-200 rounded @max-sm:p-2 @max-sm:text-sm">
        <h2>Secure Checkout</h2>
        <p>Verified by Trusted Security Provider</p>
        <div className="mt-2">
          <input placeholder="Card number" className="border p-2 w-full rounded" />
          <button className="bg-blue-500 text-white px-4 py-2 mt-2 w-full rounded">
            Pay Now
          </button>
        </div>
      </div>
    </div>
  );
}
