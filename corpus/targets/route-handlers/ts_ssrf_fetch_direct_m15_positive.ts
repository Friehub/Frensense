// [frensense]
// observation: User-controlled URL is passed to fetch() without validation via a promise .then() chain.
// impact: An attacker can make the server send requests to internal services.
// improvement: Validate URL against allowlist

function fetchUserData(req: Request, res: Response) {
    Promise.resolve(req.query.url).then(url => {
        fetch(url).then(response => response.json()).then(data => res.json(data));
    });
}

function proxyRequest(req: Request, res: Response) {
    new Promise(resolve => resolve(req.body.target)).then(target => {
        fetch(target).then(result => result.text()).then(body => res.send(body));
    });
}
