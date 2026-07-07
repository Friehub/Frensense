import { handleDownload } from './controller';

export function setupRoutes(app: any) {
    app.get('/download', (req: any, res: any) => {
        const targetHost = req.query.host;
        handleDownload(targetHost, res);
    });
}
