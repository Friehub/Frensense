// [frensense]
// observation: A postMessage event handler receives data and directly sets innerHTML without validating the sender's origin or sanitizing the content.
// impact: Any cross-origin iframe or popup can send a malicious message to the window, injecting arbitrary HTML/JavaScript into the page.
// improvement: Always validate event.origin before processing messages, and avoid innerHTML when setting content from postMessage.
// cwe: CWE-79
// cvss: 6.1
// owasp: A03:2021
// severity: Medium
// runtime_probe: xss

window.addEventListener("message", (event) => {
    document.getElementById("display")!.innerHTML = event.data;
});

export function setupWidget() {
    window.addEventListener("message", (event) => {
        const el = document.getElementById("widget");
        if (el) el.innerHTML = event.data.html;
    });
}
