// Vulnerable: command injection
const { exec } = require('child_process');
exec(userInput);
