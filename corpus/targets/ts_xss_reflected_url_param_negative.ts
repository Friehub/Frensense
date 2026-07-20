// SAFE: URL parameter values are set via textContent, not innerHTML, preventing HTML injection
export function renderPage() {
    const params = new URLSearchParams(window.location.search);
    const message = params.get("message");
    const el = document.getElementById("output");
    if (el) el.textContent = message;
}

export function displayError() {
    const hash = window.location.hash.slice(1);
    const el = document.getElementById("errorBox");
    if (el) el.textContent = hash;
}
