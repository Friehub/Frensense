// [frensense]
// observation: An anchor's href, iframe's src, or form's action attribute is set to a user-controlled URL that uses the "javascript:" protocol, enabling XSS when clicked.
// impact: When a victim clicks the crafted link, the browser executes the JavaScript in the href, leading to XSS in the page context.
// improvement: Validate that the URL protocol is http or https before assigning it to href, src, or action attributes.
// cwe: CWE-79
// cvss: 6.1
// owasp: A03:2021
// severity: Medium
// runtime_probe: xss

export function createUserLink(username: string) {
    const a = document.createElement("a");
    a.href = new URLSearchParams(location.search).get("url")!;
    a.textContent = `Visit ${username}`;
    document.body.appendChild(a);
}

export function setAvatar() {
    const avatarUrl = new URLSearchParams(location.search).get("avatar")!;
    document.getElementById("avatar")!.setAttribute("src", avatarUrl);
}
