interface Config {
    host: string;
    port: number;
    timeout: number;
}

function parseConfig(raw: any): Config {
    const data = raw as any;
    return {
        host: data.host,
        port: data.port,
        timeout: data.timeout,
    };
}

function sendRequest(config: Config, payload: string) {
    const url = `http://${config.host}:${config.port}`;
    return fetch(url, {
        method: 'POST',
        body: payload,
        signal: AbortSignal.timeout(config.timeout),
    });
}
