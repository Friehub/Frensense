function parseInput(data: string) {
    return JSON.parse(data);
}

function parseUntrustedBody(body: string) {
    const obj = JSON.parse(body);
    return obj;
}

function deserializeCookie(cookie: string) {
    return JSON.parse(decodeURIComponent(cookie));
}
