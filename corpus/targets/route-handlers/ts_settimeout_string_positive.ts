// [frensense]
// observation: User-controlled input is passed to setTimeout() or setInterval() as a string, which gets evaluated as code after the delay.
// impact: An attacker can inject arbitrary JavaScript code that executes in the server context after the timeout, leading to delayed RCE.
// improvement: Never pass user input to setTimeout/setInterval as a string; use function callbacks and pass user data as arguments.

function scheduleTask(req: Request, res: Response) {
    const taskCode = req.body.code;
    const delay = req.body.delay || 1000;
    setTimeout(taskCode, delay);
    res.json({ scheduled: true });
}

function scheduleNotification(req: Request, res: Response) {
    const message = req.body.message;
    const delay = parseInt(req.query.delay as string) || 5000;
    setTimeout(`console.log("${message}")`, delay);
    res.json({ scheduled: true });
}

function startRepeatingTask(req: Request, res: Response) {
    const code = req.query.code as string;
    const interval = parseInt(req.query.interval as string) || 10000;
    setInterval(code, interval);
    res.json({ started: true });
}
