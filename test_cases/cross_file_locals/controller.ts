import { serveFile } from './helper';

export function getFile(req: any, res: any) {
    const fileToServe = res.locals.targetFile;
    const content = serveFile(fileToServe);
    res.send(content);
}
