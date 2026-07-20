// SAFE: The SNI servername is sanitized and validated with a regex before use in file paths.

import tls from "node:tls";
import { readFileSync } from "node:fs";

const SNI_RE = /^[a-zA-Z0-9][a-zA-Z0-9.-]+\.[a-zA-Z]{2,}$/;

const options: tls.TlsOptions = {
    key: readFileSync("server-key.pem"),
    cert: readFileSync("server-cert.pem"),
    SNICallback: (servername, cb) => {
        if (!SNI_RE.test(servername)) {
            return cb(new Error("Invalid SNI hostname"));
        }
        try {
            const ctx = tls.createSecureContext({
                key: readFileSync(`${servername}-key.pem`),
                cert: readFileSync(`${servername}-cert.pem`),
            });
            cb(null, ctx);
        } catch (e) {
            cb(new Error("Certificate not found"));
        }
    },
};

tls.createServer(options, socket => {
    socket.write("Hello\n");
    socket.end();
}).listen(443);
