export function CartItem({ item }: { item: any }) {
    return (
        <div className="cart-item">
            <span className="title">{item.name}</span>
            <span className="total">
                Total: {Number(item.priceSnapshot ?? 0) * item.quantity}
            </span>
        </div>
    );
}
