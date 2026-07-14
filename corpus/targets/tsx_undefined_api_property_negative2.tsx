// [frensense]
// observation: Defensive rendering with null checks and fallbacks for potentially missing properties.
// impact: None — rendering is safe even if properties are missing.
// improvement: N/A — this is the correct pattern.

export function SafeCartItem({ item }: { item: any }) {
    if (!item) return null;

    // Defensive check: handle potentially undefined values safely
    const price = typeof item.price === 'number' ? item.price : 0;
    const quantity = typeof item.quantity === 'number' ? item.quantity : 1;
    const total = price * quantity;

    return (
        <div className="cart-item">
            <span className="title">{item.name || 'Unknown Item'}</span>
            <span className="total">
                Total: {total.toFixed(2)}
            </span>
            {item.priceSnapshot && (
                 <span className="snapshot">Snapshot: {item.priceSnapshot}</span>
            )}
        </div>
    );
}
