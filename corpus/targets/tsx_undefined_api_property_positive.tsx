// [frensense]
// observation: A React component attempts to render a mathematical calculation using an object property (e.g., item.price) that does not exist in the API response, which instead uses a different field name (e.g., item.priceSnapshot).
// impact: The calculation evaluates to undefined or NaN, resulting in an incorrect rendering (e.g., displaying a ₦0 total) despite items existing in the data.
// improvement: Use the correct API property field and ensure it is safely parsed into a Number, with a fallback value.
// cwe: CWE-754
// cvss: 5.3
// owasp: 
// severity: Medium

export function CartItem({ item }: { item: any }) {
    return (
        <div className="cart-item">
            <span className="title">{item.name}</span>
            <span className="total">
                Total: {item.price * item.quantity}
            </span>
        </div>
    );
}
