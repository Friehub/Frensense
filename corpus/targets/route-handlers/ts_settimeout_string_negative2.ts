// SAFE: Used a task queue system with job handlers instead of setTimeout with string code.

const TASK_HANDLERS: Record<string, (data: any) => void> = {
    "cleanup": (data) => cleanupLogs(data.retentionDays),
    "backup": (data) => backupDatabase(data.path),
    "notify": (data) => sendPushNotification(data.userId, data.message),
};

function scheduleTask(req: Request, res: Response) {
    const taskName = req.body.task;
    const handler = TASK_HANDLERS[taskName];
    if (!handler) throw new Error("Unknown task");
    const delay = req.body.delay || 1000;
    setTimeout(() => handler(req.body.data), delay);
    res.json({ scheduled: true });
}

function scheduleNotification(req: Request, res: Response) {
    const message = req.body.message;
    const delay = parseInt(req.query.delay as string) || 5000;
    setTimeout(() => sendNotification(message), delay);
    res.json({ scheduled: true });
}
