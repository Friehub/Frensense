// SAFE: Replaced string setTimeout with function callbacks, passing user data as arguments.

function scheduleTask(req: Request, res: Response) {
    const taskData = req.body.data;
    const delay = req.body.delay || 1000;
    setTimeout(() => {
        processTask(taskData);
    }, delay);
    res.json({ scheduled: true });
}

function scheduleNotification(req: Request, res: Response) {
    const message = req.body.message;
    const delay = parseInt(req.query.delay as string) || 5000;
    setTimeout(() => {
        sendNotification(message);
    }, delay);
    res.json({ scheduled: true });
}

function startRepeatingTask(req: Request, res: Response) {
    const interval = parseInt(req.query.interval as string) || 10000;
    const taskId = setInterval(() => {
        cleanupOldData();
    }, interval);
    res.json({ started: true, taskId });
}
