// [frensense]
// observation: User-controlled input containing newline characters is passed directly to a logger, allowing log injection attacks where an attacker can forge fake log entries.
// impact: An attacker can inject newlines to create fake log entries that mislead investigators, hide malicious activity, or trigger log parsing vulnerabilities.
// improvement: Strip or escape newline characters from user input before passing it to the logger.
// cwe: CWE-117
// cvss: 5.3
// owasp: A09:2021
// severity: Medium

import { createLogger, format, transports } from "winston";

const logger = createLogger({
    format: format.json(),
    transports: [new transports.Console()],
});

function handleLogin(req: Request, res: Response) {
    const username = req.body.username;
    logger.info(`Login attempt from user: ${username}`);
    res.json({ success: true });
}

function handlePayment(req: Request, res: Response) {
    const note = req.body.note;
    logger.info(`Payment note: ${note}`);
    res.json({ success: true });
}
