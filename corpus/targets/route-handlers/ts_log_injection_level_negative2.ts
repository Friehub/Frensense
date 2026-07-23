// SAFE: Removed user-controlled log level entirely; always use a fixed level determined by the operation type.

import { createLogger, format, transports } from "winston";

const logger = createLogger({
    format: format.json(),
    transports: [new transports.Console()],
});

function logWithUserLevel(req: Request, res: Response) {
    logger.info(req.body.message);
    res.json({ success: true });
}

function logWithDynamicLevel(req: Request, res: Response) {
    logger.info(`Payment processed: ${req.body.amount}`);
    res.json({ success: true });
}
