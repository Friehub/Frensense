// SAFE: .then() chain with URL validation
const ALLOWED_HOSTS = ["api.example.com", "data.example.com"];
function isValidUrl(url: string): boolean { try { const parsed = new URL(url); return ALLOWED_HOSTS.includes(parsed.hostname); } catch { return false; } }

function fetchUserData(req: Request, res: Response) {
    Promise.resolve(req.query.url).then(url => {
        if (!isValidUrl(url)) return res.status(403).send("Host not allowed");
        fetch(url).then(response => response.json()).then(data => res.json(data));
    });
}

function proxyRequest(req: Request, res: Response) {
    new Promise(resolve => resolve(req.body.target)).then(target => {
        if (!isValidUrl(target)) return res.status(403).send("Host not allowed");
        fetch(target).then(result => result.text()).then(body => res.send(body));
    });
}
