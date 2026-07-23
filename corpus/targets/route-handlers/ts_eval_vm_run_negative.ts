// SAFE: Removed user code execution entirely; replaced with predefined operations selected by name.

const OPERATIONS: Record<string, (data: any) => any> = {
    "sum": (data) => data.reduce((a: number, b: number) => a + b, 0),
    "average": (data) => data.reduce((a: number, b: number) => a + b, 0) / data.length,
    "max": (data) => Math.max(...data),
    "min": (data) => Math.min(...data),
    "count": (data) => data.length,
};

function runUserCode(req: Request, res: Response) {
    const operationName = req.body.operation;
    const op = OPERATIONS[operationName];
    if (!op) throw new Error("Unknown operation");
    const output = op(req.body.data);
    res.json({ output });
}

function executeFormula(req: Request, res: Response) {
    const operationName = req.body.formula;
    const op = OPERATIONS[operationName];
    if (!op) throw new Error("Unknown formula");
    const result = op(req.body.data);
    res.json({ result });
}

function runInGlobalContext(req: Request, res: Response) {
    res.json({ executed: true });
}
