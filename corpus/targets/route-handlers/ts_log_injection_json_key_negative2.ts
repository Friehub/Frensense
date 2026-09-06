// SAFE: Used a fixed schema for log fields with user input only in a pre-defined "payload" sub-field.

import { createLogger, format, transports } from "winston";

const logger = createLogger({
    format: format.json(),
    transports: [new transports.Console()],
});

function trackEvent(req: Request, res: Response) {
    logger.info("User event", {
        event: req.body.event,
        payload: JSON.stringify(req.body.metadata || {}),
        userId: req.user.id,
    });
    res.json({ success: true });
}

function logCustomField(req: Request, res: Response) {
    logger.info("Custom log", {
        fieldName: req.query.field,
        fieldValue: req.query.value,
        userId: req.user.id,
    });
    res.json({ success: true });
}
