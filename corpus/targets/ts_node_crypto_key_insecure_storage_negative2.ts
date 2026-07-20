// SAFE: The private key is stored using fs.open with exclusive mode and restricted permissions.

import { openSync, writeFileSync } from "node:fs";
import { generateKeyPairSync } from "node:crypto";

function storeKeyPair() {
    const { publicKey, privateKey } = generateKeyPairSync("rsa", {
        modulusLength: 4096,
    });
    const fd = openSync("/etc/app/private.pem", "wx", 0o600);
    writeFileSync(fd, privateKey.export({ type: "pkcs1", format: "pem" }));
    writeFileSync("/etc/app/public.pem", publicKey.export({ type: "spki", format: "pem" }), { mode: 0o644 });
}
