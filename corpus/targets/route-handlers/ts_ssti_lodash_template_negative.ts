// SAFE: Replaced _.template() with simple string interpolation using a static template string.

function renderGreeting(req: Request, res: Response) {
    const tpl = "Hello <%= name %>, your role is <%= role %>";
    const html = tpl
        .replace("<%= name %>", req.user.name)
        .replace("<%= role %>", req.user.role);
    res.send(html);
}

function renderMessage(req: Request, res: Response) {
    const result = `User: ${req.user.name}, Data: ${JSON.stringify(req.body)}`;
    res.json({ message: result });
}
