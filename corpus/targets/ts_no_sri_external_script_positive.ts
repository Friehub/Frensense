// [frensense]
// observation: External script or stylesheet loaded from a CDN without Subresource Integrity (SRI) integrity attribute.
// impact: If the CDN is compromised (or uses a compromised build pipeline), the served JavaScript file could contain malicious code. Since the browser has no integrity hash to verify, the malicious script executes with full page access.
// improvement: Add integrity attribute with the base64-encoded hash of the expected file content. Use SRI hash generators for all external resources.
// cwe: CWE-345
// cvss: 5.3
// owasp: 
// severity: Medium

// VULNERABLE: no integrity check
<html>
  <head>
    <script src="https://cdn.example.com/lib.js"></script>
    <link rel="stylesheet" href="https://cdn.example.com/styles.css">
  </head>
</html>
