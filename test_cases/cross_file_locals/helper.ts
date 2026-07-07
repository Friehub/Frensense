import * as fs from 'fs';

export function serveFile(filename: string) {
    // Path traversal vulnerability here
    return fs.readFileSync('/var/www/uploads/' + filename, 'utf8');
}
