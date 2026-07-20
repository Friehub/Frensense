// SAFE: Values are sanitized with DOMPurify before injection
import DOMPurify from "dompurify";

export function renderPage() {
    const params = new URLSearchParams(window.location.search);
    const message = params.get("message");
    const el = document.getElementById("output");
    if (el && message) el.innerHTML = DOMPurify.sanitize(message);
}

export function displayError() {
    const hash = window.location.hash.slice(1);
    const el = document.getElementById("errorBox");
    if (el && hash) el.innerHTML = DOMPurify.sanitize(hash);
}
