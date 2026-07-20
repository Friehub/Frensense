// SAFE: The watched directory is checked for symlinks and each event is validated against the allowed base path.

import { watch, realpathSync } from "node:fs";
import { resolve } from "node:path";

const BASE_DIR = resolve("/var/app/data");

function watchDirectory(dirPath: string) {
    const resolved = realpathSync(resolve(dirPath));
    if (!resolved.startsWith(BASE_DIR)) {
        throw new Error("Path outside allowed base directory");
    }
    const watcher = watch(resolved, (eventType, filename) => {
        if (!filename) return;
        const fullPath = resolve(resolved, filename);
        if (!fullPath.startsWith(BASE_DIR)) {
            console.warn("Ignored event outside base dir:", fullPath);
            return;
        }
        console.log(`${eventType}: ${filename}`);
    });
    watcher.on("error", err => console.error(err));
}
