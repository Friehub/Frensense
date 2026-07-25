// [frensense]
// observation: A URL query parameter or hash fragment value is read and directly written into the DOM or response body without encoding.
// impact: An attacker can craft a malicious link with a JavaScript payload in the URL parameter, causing XSS when the victim clicks it.
// improvement: Always HTML-encode values from URL parameters before rendering them in the page.
// cwe: CWE-79
// cvss: 6.1
// owasp: A03:2021
// severity: Medium
// runtime_probe: xss

export function renderPage() {
    const params = new URLSearchParams(window.location.search);
    const message = params.get("message");
    document.getElementById("output")!.innerHTML = message;
}

export function displayError() {
    const hash = window.location.hash.slice(1);
    const el = document.getElementById("errorBox");
    if (el) el.innerHTML = hash;
}
