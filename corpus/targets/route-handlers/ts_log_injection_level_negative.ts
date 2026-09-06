// SAFE: Validated the log level against an allowlist of permitted severity values.

import { createLogger, format, transports } from "winston";

const logger = createLogger({
    format: format.json(),
    transports: [new transports.Console()],
});

const ALLOWED_LEVELS = new Set(["error", "warn", "info", "debug", "verbose"]);

function logWithUserLevel(req: Request, res: Response) {
    const userLevel = req.body.severity || "info";
    if (!ALLOWED_LEVELS.has(userLevel)) {
        res.status(400).json({ error: "Invalid severity level" });
        return;
    }
    logger.log(userLevel, req.body.message);
    res.json({ success: true });
}

function logWithDynamicLevel(req: Request, res: Response) {
    const severity = (req.query.level as string) || "info";
    if (!ALLOWED_LEVELS.has(severity)) throw new Error("Invalid level");
    logger.info(`Payment processed: ${req.body.amount}`);
    res.json({ success: true });
}
