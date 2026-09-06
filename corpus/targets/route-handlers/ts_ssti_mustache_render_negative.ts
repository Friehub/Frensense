// SAFE: Used static Mustache templates from the filesystem, never accepting templates from user input.

import Mustache from "mustache";
import { readFileSync } from "fs";

const docTemplate = readFileSync("./templates/document.mustache", "utf-8");
const previewTemplate = readFileSync("./templates/preview.mustache", "utf-8");

function renderDocument(req: Request, res: Response) {
    const html = Mustache.render(docTemplate, { user: req.user, doc: req.body.doc });
    res.send(html);
}

function renderPreview(req: Request, res: Response) {
    const html = Mustache.render(previewTemplate, { data: req.query });
    res.send(html);
}
