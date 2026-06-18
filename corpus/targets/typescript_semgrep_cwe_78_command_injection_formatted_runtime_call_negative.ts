// Fixed: use execFile with args array
const { execFile } = require('child_process');
execFile('cmd', ['/c', 'echo', userInput]);
