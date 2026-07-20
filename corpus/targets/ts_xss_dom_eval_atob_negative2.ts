// SAFE: No eval or setTimeout with string; uses postMessage-based architecture with origin validation
export function processPayload() {
    const encoded = location.hash.slice(1);
    try {
        const decoded = atob(encoded);
        const data = JSON.parse(decoded);
        window.postMessage({ type: "RENDER", data }, window.location.origin);
    } catch {
        console.error("Invalid payload");
    }
}

window.addEventListener("message", (event) => {
    if (event.origin !== window.location.origin) return;
    if (event.data.type === "RENDER") {
        const el = document.getElementById("output");
        if (el) el.textContent = JSON.stringify(event.data.data);
    }
});
