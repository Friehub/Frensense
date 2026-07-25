// [frensense]
// observation: The location.hash or location.search value is assigned directly to element.innerHTML, allowing an attacker to inject HTML via a crafted URL fragment.
// impact: A victim clicking a link with a malicious hash (e.g., #<img src=x onerror=alert(1)>) will execute arbitrary JavaScript in the page context.
// improvement: Use textContent instead of innerHTML, or sanitize the input before assigning it.
// cwe: CWE-79
// cvss: 6.1
// owasp: A03:2021
// severity: Medium
// runtime_probe: xss

export function updateBanner() {
    const hash = location.hash.slice(1);
    document.getElementById("banner")!.innerHTML = hash;
}

export function showSearchResults() {
    const params = new URLSearchParams(location.search);
    const term = params.get("q");
    const el = document.getElementById("results");
    if (el && term) el.innerHTML = `Results for: ${term}`;
}
