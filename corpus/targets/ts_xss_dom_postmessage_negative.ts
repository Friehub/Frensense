// SAFE: Validates event.origin and uses textContent instead of innerHTML
const ALLOWED_ORIGINS = new Set(["https://trusted.example.com"]);

window.addEventListener("message", (event) => {
    if (!ALLOWED_ORIGINS.has(event.origin)) return;
    document.getElementById("display")!.textContent = event.data;
});

export function setupWidget() {
    window.addEventListener("message", (event) => {
        if (!ALLOWED_ORIGINS.has(event.origin)) return;
        const el = document.getElementById("widget");
        if (el) el.textContent = event.data.html;
    });
}
