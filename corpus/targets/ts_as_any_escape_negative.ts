interface Config {
    host: string;
    port: number;
    timeout: number;
}

function parseConfig(raw: unknown): Config {
    if (typeof raw !== 'object' || raw === null) {
        throw new Error('Config must be an object');
    }
    const obj = raw as Record<string, unknown>;

    if (typeof obj.host !== 'string') throw new Error('host must be a string');
    if (typeof obj.port !== 'number' || obj.port <= 0) throw new Error('port must be a positive number');
    if (typeof obj.timeout !== 'number' || obj.timeout <= 0) throw new Error('timeout must be positive');

    return { host: obj.host, port: obj.port, timeout: obj.timeout };
}

function sendRequest(config: Config, payload: string) {
    const url = `http://${config.host}:${config.port}`;
    return fetch(url, {
        method: 'POST',
        body: payload,
        signal: AbortSignal.timeout(config.timeout),
    });
}
