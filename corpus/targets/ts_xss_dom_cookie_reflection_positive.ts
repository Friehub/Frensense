// [frensense]
// observation: The document.cookie value is read and assigned directly to innerHTML, which exposes session cookies and allows XSS if the cookie value contains HTML.
// impact: An attacker who can set a cookie value (e.g., via a subdomain vulnerability or cookie injection) can execute arbitrary JavaScript when the cookie is rendered on the page.
// improvement: Never render cookie values directly in the DOM; use textContent or escape the value first.
// cwe: CWE-79
// cvss: 6.1
// owasp: A03:2021
// severity: Medium
// runtime_probe: xss

export function showCookieDebug() {
    const el = document.getElementById("debug");
    if (el) el.innerHTML = `Cookies: ${document.cookie}`;
}

export function displayUserPreference() {
    const theme = document.cookie
        .split("; ")
        .find(row => row.startsWith("theme="))
        ?.split("=")[1];
    const el = document.getElementById("theme-display");
    if (el && theme) el.innerHTML = `Current theme: ${theme}`;
}
