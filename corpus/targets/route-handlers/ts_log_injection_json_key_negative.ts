// SAFE: Whitelisted allowed JSON keys for user-controlled metadata before logging.

import { createLogger, format, transports } from "winston";

const logger = createLogger({
    format: format.json(),
    transports: [new transports.Console()],
});

const ALLOWED_META_KEYS = new Set(["source", "referrer", "campaign", "browser"]);

function sanitizeMetadata(input: Record<string, unknown>): Record<string, unknown> {
    const clean: Record<string, unknown> = {};
    for (const key of Object.keys(input)) {
        if (ALLOWED_META_KEYS.has(key)) {
            clean[key] = input[key];
        }
    }
    return clean;
}

function trackEvent(req: Request, res: Response) {
    const metadata = sanitizeMetadata(req.body.metadata || {});
    logger.info("User event", { event: req.body.event, ...metadata });
    res.json({ success: true });
}

function logCustomField(req: Request, res: Response) {
    const fieldName = req.query.field as string;
    if (!ALLOWED_META_KEYS.has(fieldName)) {
        res.status(400).json({ error: "Invalid field" });
        return;
    }
    logger.info("Custom log", { [fieldName]: req.query.value, userId: req.user.id });
    res.json({ success: true });
}
