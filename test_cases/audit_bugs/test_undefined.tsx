export function CheckoutSummary({ basket }: { basket: any }) {
    return (
        <div className="summary">
            <h3>Summary</h3>
            <p>Subtotal: ${basket.price * basket.qty}</p>
        </div>
    );
}
