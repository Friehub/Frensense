// [frensense]
// observation: fs.watch is used on a directory without checking for symbolic links, allowing a symlink to redirect file change detection outside the watched directory.
// impact: An attacker can create a symlink inside the watched directory pointing to a sensitive location, and fs.watch will track changes there, enabling directory traversal via file events.
// improvement: Resolve the real path of the watched directory and reject symlinks, or use fs.watch with the followSymlinks option set to false where available.

import { watch } from "node:fs";

function watchDirectory(dirPath: string) {
    const watcher = watch(dirPath, (eventType, filename) => {
        console.log(`${eventType}: ${filename}`);
    });
    watcher.on("error", err => console.error(err));
}
