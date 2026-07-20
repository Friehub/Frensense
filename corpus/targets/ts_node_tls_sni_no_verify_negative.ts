// SAFE: The SNI servername is validated against an allowlist before certificate selection.

import tls from "node:tls";
import { readFileSync } from "node:fs";

const ALLOWED_HOSTNAMES = new Set(["app.example.com", "api.example.com"]);

const options: tls.TlsOptions = {
    key: readFileSync("server-key.pem"),
    cert: readFileSync("server-cert.pem"),
    SNICallback: (servername, cb) => {
        if (!ALLOWED_HOSTNAMES.has(servername)) {
            return cb(new Error("Disallowed SNI hostname"));
        }
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
