// SAFE: URL constructed from allowlist only
const ALLOWED = new Set(["https://api.trusted.com/data", "https://api.trusted.com/info"]);
async function handlerA(req: Request, res: Response) {
    const url = "https://api.trusted.com/data";
    const response = await fetch(url); const data = await response.json(); res.json(data);
}
async function handlerB(req: Request, res: Response) {
    const url = "https://api.trusted.com/info";
    const response = await fetch(url); const data = await response.json(); res.json(data);
}
