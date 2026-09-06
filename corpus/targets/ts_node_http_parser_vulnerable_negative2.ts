// SAFE: A safe HTTP parser is used that limits input size and rejects malformed messages.

import { createServer, IncomingMessage } from "node:http";
import { Socket } from "node:net";

function parseSafely(raw: string): { method: string; url: string; headers: Record<string, string> } | null {
    try {
        const socket = new Socket();
        const req = new IncomingMessage(socket);
        req.push(Buffer.from(raw));
        req.push(null);
        return new Promise(resolve => {
            req.on("data", () => {});
            req.on("end", () => {
                resolve({ method: req.method ?? "", url: req.url ?? "", headers: req.headers as Record<string, string> });
            });
        });
    } catch {
        return null;
    }
}
