// [frensense]
// observation: A cryptographic key is written to a file with world-readable permissions (0o644 or default), exposing it to other processes on the system.
// impact: Any user or process on the system can read the key file, leading to compromise of encrypted data or authentication.
// improvement: Write the key with restricted permissions (0o600) using the mode option in writeFile, or use a dedicated key store.
// cwe: CWE-327
// cvss: 7.5
// owasp: A02:2021
// severity: High

import { writeFileSync } from "node:fs";
import { generateKeyPairSync } from "node:crypto";

function storeKeyPair() {
    const { publicKey, privateKey } = generateKeyPairSync("rsa", {
        modulusLength: 4096,
    });
    writeFileSync("/etc/app/private.pem", privateKey.export({ type: "pkcs1", format: "pem" }));
    writeFileSync("/etc/app/public.pem", publicKey.export({ type: "spki", format: "pem" }));
}
