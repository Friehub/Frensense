// [frensense]
// observation: A user-provided parameter is appended to the path of a hardcoded, trusted base URL before making an HTTP request.
// impact: None — the attacker can only control the path or query string, not the destination host, meaning they cannot force the server to connect to arbitrary internal or external systems.
// improvement: N/A — this is the correct pattern.

export async function fetchUserProfile(req: any, res: any) {
    const username = req.params.username;
    // Good: the host is hardcoded, user only controls the path
    const response = await fetch(`https://api.github.com/users/${username}`);
    const data = await response.json();
    res.json(data);
}

export async function getLocalItem(req: any, res: any) {
    const itemId = req.query.id;
    // Good: hitting an internal service where the host is fixed
    const result = await fetch("http://internal-service/items?id=" + encodeURIComponent(itemId));
    res.send(await result.text());
}
