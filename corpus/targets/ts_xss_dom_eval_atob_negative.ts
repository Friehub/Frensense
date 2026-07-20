// SAFE: Removes eval() entirely; uses safe JSON parsing for structured data
export function processPayload() {
    const encoded = location.hash.slice(1);
    try {
        const decoded = atob(encoded);
        const data = JSON.parse(decoded);
        renderData(data);
    } catch {
        console.error("Invalid payload");
    }
}

function renderData(data: any) {
    const el = document.getElementById("output");
    if (el) el.textContent = JSON.stringify(data);
}

export function runCommand() {
    const cmd = new URLSearchParams(location.search).get("cmd");
    if (cmd) {
        try {
            const decoded = atob(cmd);
            const data = JSON.parse(decoded);
            executeSafe(data);
        } catch {
            console.error("Invalid command");
        }
    }
}

function executeSafe(data: any) {
    console.log("Executing:", data);
}
