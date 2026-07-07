import { pingServer } from './utils';

export function handleDownload(host: string, res: any) {
    if (host) {
        // Business logic wrapper
        pingServer(host);
        res.send("Download initiated");
    } else {
        res.status(400).send("Host required");
    }
}
