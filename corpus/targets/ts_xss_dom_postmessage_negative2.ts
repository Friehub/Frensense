// SAFE: Validates origin and uses DOMPurify before innerHTML assignment
import DOMPurify from "dompurify";

const ALLOWED_ORIGINS = new Set(["https://trusted.example.com"]);

window.addEventListener("message", (event) => {
    if (!ALLOWED_ORIGINS.has(event.origin)) return;
    const sanitized = DOMPurify.sanitize(event.data);
    document.getElementById("display")!.innerHTML = sanitized;
});

export function setupWidget() {
    window.addEventListener("message", (event) => {
        if (!ALLOWED_ORIGINS.has(event.origin)) return;
        const el = document.getElementById("widget");
        if (el) el.innerHTML = DOMPurify.sanitize(event.data.html);
    });
}
