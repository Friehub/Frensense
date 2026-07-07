import { exec } from 'child_process';

export function pingServer(hostname: string) {
    // Dangerous sink without validation
    exec("ping -c 1 " + hostname, (error, stdout, stderr) => {
        if (error) {
            console.error(`exec error: ${error}`);
            return;
        }
        console.log(`stdout: ${stdout}`);
    });
}
