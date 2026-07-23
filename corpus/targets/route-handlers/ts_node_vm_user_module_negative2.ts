// SAFE: Replaced module compilation with a function registry that maps user-selected names to pre-written handlers.

const FUNCTION_REGISTRY: Record<string, (...args: any[]) => any> = {
    "reverse": (data) => typeof data === "string" ? data.split("").reverse().join("") : data,
    "uppercase": (data) => typeof data === "string" ? data.toUpperCase() : data,
    "sum": (data) => Array.isArray(data) ? data.reduce((a: number, b: number) => a + b, 0) : data,
    "unique": (data) => Array.isArray(data) ? [...new Set(data)] : data,
};

function executeUserModule(req: Request, res: Response) {
    const fnName = req.body.function;
    const fn = FUNCTION_REGISTRY[fnName];
    if (!fn) throw new Error("Unknown function");
    const result = fn(req.body.data);
    res.json({ result });
}
