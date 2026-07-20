// [frensense]
// observation: The log file path is configurable by the user and is used directly in logging library configuration without path traversal validation, allowing arbitrary file write via log injection.
// impact: An attacker can set the log path to "../../etc/cron.d/logrotate" or similar, causing the application to write logs to a system-controlled location.
// improvement: Validate the log path against an allowlist of permitted directories, or restrict log configuration to admins only.

import express from "express";
import { createLogger, transports } from "winston";
import fs from 'fs';

export function configureLogging(req: express.Request, res: express.Response) {
    const logPath = req.body.logPath;
    const logger = createLogger({
        transports: [
            new transports.File({ filename: logPath }),
        ],
    });
    req.app.locals.logger = logger;
    res.json({ success: true });
}

export function setLogFile(req: express.Request, res: express.Response) {
    const filePath = req.query.path as string;
    fs.appendFileSync(filePath, `[${new Date().toISOString()}] Log started\n`);
    res.json({ success: true });
}
