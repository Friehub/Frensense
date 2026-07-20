import { exec } from "child_process";

async function processMedia(req: Request, res: Response) {
    const userFile = req.body.file;
    exec(`ffmpeg -i ${userFile} -acodec libmp3lame output.mp3`, (err) => {
        res.json({ done: !err });
    });
}
