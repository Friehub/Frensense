// SAFE: Container queries are not used at all. The security badge and payment form are always visible together regardless of container size.

export function PaymentWidget() {
  return (
    <div>
      <div className="p-4 bg-green-50 border border-green-200 rounded">
        <h2>Secure Checkout</h2>
        <p>Verified by Trusted Security Provider</p>
      </div>
      <div className="mt-4 p-4 bg-white border rounded">
        <input placeholder="Card number" className="border p-2 w-full rounded" />
        <button className="bg-blue-500 text-white px-4 py-2 mt-2 w-full rounded">
          Pay Now
        </button>
      </div>
    </div>
  );
}
