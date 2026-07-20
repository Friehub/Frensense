// SAFE: Log path is restricted to a pre-defined directory, and user input is used only as a filename within that directory
import express from "express";
import path from "path";
import { createLogger, transports } from "winston";

const LOG_DIR = path.resolve("/var/log/app");

function safeLogFilename(name: string): string {
    return path.basename(name).replace(/[^a-zA-Z0-9._-]/g, "_");
}

export function configureLogging(req: express.Request, res: express.Response) {
    const logFile = safeLogFilename(req.body.logPath || "app.log");
    const logPath = path.join(LOG_DIR, logFile);
    if (!logPath.startsWith(LOG_DIR)) {
        return res.status(403).json({ error: "Invalid log path" });
    }
    const logger = createLogger({
        transports: [
            new transports.File({ filename: logPath }),
        ],
    });
    req.app.locals.logger = logger;
    res.json({ success: true });
}
