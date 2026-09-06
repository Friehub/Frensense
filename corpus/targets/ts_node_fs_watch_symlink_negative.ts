// SAFE: The directory path is resolved to its real path, and symlinks are rejected.

import { watch, realpathSync, lstatSync } from "node:fs";

function watchDirectory(dirPath: string) {
    const stat = lstatSync(dirPath);
    if (stat.isSymbolicLink()) {
        throw new Error("Symlinks are not allowed");
    }
    const realPath = realpathSync(dirPath);
    const watcher = watch(realPath, (eventType, filename) => {
        console.log(`${eventType}: ${filename}`);
    });
    watcher.on("error", err => console.error(err));
}
