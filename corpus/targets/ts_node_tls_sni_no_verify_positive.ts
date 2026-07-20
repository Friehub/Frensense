// [frensense]
// observation: The SNI callback in a TLS server accepts the server name from the client without verification, allowing an attacker to connect using arbitrary hostnames.
// impact: An attacker can request any server name via SNI, potentially bypassing virtual hosting restrictions or triggering incorrect certificate selection.
// improvement: Validate the SNI server name against an allowlist of expected hostnames before selecting a certificate.

import tls from "node:tls";
import { readFileSync } from "node:fs";

const options: tls.TlsOptions = {
    key: readFileSync("server-key.pem"),
    cert: readFileSync("server-cert.pem"),
    SNICallback: (servername, cb) => {
        const ctx = tls.createSecureContext({
            key: readFileSync(`${servername}-key.pem`),
            cert: readFileSync(`${servername}-cert.pem`),
        });
        cb(null, ctx);
    },
};

tls.createServer(options, socket => {
    socket.write("Hello\n");
    socket.end();
}).listen(443);
