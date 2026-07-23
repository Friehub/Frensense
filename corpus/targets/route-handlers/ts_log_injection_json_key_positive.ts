// [frensense]
// observation: User-controlled values are used as keys in a JSON log object, allowing injection of extra log fields or overriding existing fields.
// impact: An attacker can inject arbitrary JSON keys into the structured log output, potentially overriding severity levels, adding fake entries, or polluting log analytics.
// improvement: Validate or whitelist the keys that can be set by user input before logging.

import { createLogger, format, transports } from "winston";

const logger = createLogger({
    format: format.json(),
    transports: [new transports.Console()],
});

function trackEvent(req: Request, res: Response) {
    const eventData = req.body;
    logger.info("User event", { event: eventData.event, ...eventData.metadata });
    res.json({ success: true });
}

function logCustomField(req: Request, res: Response) {
    const fieldName = req.query.field as string;
    const fieldValue = req.query.value;
    logger.info("Custom log", { [fieldName]: fieldValue, userId: req.user.id });
    res.json({ success: true });
}
