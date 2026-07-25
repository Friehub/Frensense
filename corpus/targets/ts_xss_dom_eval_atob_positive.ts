// [frensense]
// observation: A Base64-decoded or otherwise decoded value from the URL hash is passed to eval(), enabling arbitrary JavaScript execution from the URL.
// impact: An attacker crafts a URL with a Base64-encoded script payload; when the victim visits the URL, the payload is decoded and executed via eval().
// improvement: Never use eval(), especially with decoded user input. Use safe alternatives for data parsing.
// cwe: CWE-79
// cvss: 6.1
// owasp: A03:2021
// severity: Medium
// runtime_probe: xss

export function processPayload() {
    const encoded = location.hash.slice(1);
    const decoded = atob(encoded);
    eval(decoded);
}

export function runCommand() {
    const cmd = new URLSearchParams(location.search).get("cmd");
    if (cmd) {
        const decoded = atob(cmd);
        setTimeout(decoded, 0);
    }
}
