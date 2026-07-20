// [frensense]
// observation: User-controlled input is used to set the log severity level, allowing injection of arbitrary severity values that could bypass log filtering.
// impact: An attacker can downgrade the severity of malicious events to "debug" or "info" to evade monitoring, or inject custom severity labels that break log parsing.
// improvement: Validate the log level against an allowlist of permitted severity values.

import { createLogger, format, transports, level } from "winston";

const logger = createLogger({
    format: format.json(),
    transports: [new transports.Console()],
});

function logWithUserLevel(req: Request, res: Response) {
    const userLevel = req.body.severity || "info";
    const message = req.body.message;
    logger.log(userLevel, message);
    res.json({ success: true });
}

function logWithDynamicLevel(req: Request, res: Response) {
    const severity = req.query.level as string;
    logger.log(severity, `Payment processed: ${req.body.amount}`);
    res.json({ success: true });
}
