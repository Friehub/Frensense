// SAFE: Log path is not configurable by users; only a fixed set of log files exists
import express from "express";
import path from "path";
import { createLogger, transports } from "winston";

const LOG_DIR = path.resolve("/var/log/app");

const VALID_LOG_FILES = new Set(["app.log", "error.log", "audit.log"]);

export function configureLogging(req: express.Request, res: express.Response) {
    return res.status(403).json({ error: "Log configuration is read-only" });
}

export function getLogList() {
    return Array.from(VALID_LOG_FILES);
}
