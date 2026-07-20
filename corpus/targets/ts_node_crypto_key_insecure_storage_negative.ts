// SAFE: The key is written with restricted file permissions (0o600), readable only by the owner.

import { writeFileSync } from "node:fs";
import { generateKeyPairSync } from "node:crypto";

function storeKeyPair() {
    const { publicKey, privateKey } = generateKeyPairSync("rsa", {
        modulusLength: 4096,
    });
    writeFileSync("/etc/app/private.pem", privateKey.export({ type: "pkcs1", format: "pem" }), { mode: 0o600 });
    writeFileSync("/etc/app/public.pem", publicKey.export({ type: "spki", format: "pem" }), { mode: 0o644 });
}
