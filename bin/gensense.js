#!/usr/bin/env node

const { spawn } = require('child_process');
const path = require('path');
const os = require('os');
const fs = require('fs');

/**
 * GenSense CLI Wrapper
 * This script selects the correct native binary for the current platform.
 */

const platform = os.platform();
const arch = os.arch();
const isWin = platform === 'win32';

// 1. Determine platform suffix for the bundled binaries
let suffix = '';
if (platform === 'linux' && arch === 'x64') suffix = 'linux-x64';
else if (platform === 'darwin' && arch === 'x64') suffix = 'macos-x64';
else if (platform === 'darwin' && arch === 'arm64') suffix = 'macos-arm64';
else if (platform === 'win32' && arch === 'x64') suffix = 'windows-x64.exe';

// 2. Resolution priority:
// - dist/binaries/gensense-<suffix> (Bundled multi-platform)
// - dist/gensense (Directly bundled single-platform)
// - target/release/gensense (Local development)
// - cargo run (Dev fallback)

const candidates = [
    path.join(__dirname, '..', 'dist', 'binaries', `gensense-${suffix}`),
    path.join(__dirname, '..', 'dist', isWin ? 'gensense.exe' : 'gensense'),
    path.join(__dirname, '..', 'target', 'release', isWin ? 'gensense.exe' : 'gensense')
];

let binPath = candidates.find(p => fs.existsSync(p) && fs.statSync(p).isFile());

const args = process.argv.slice(2);

if (binPath) {
    run(binPath, args);
} else if (fs.existsSync(path.join(__dirname, '..', 'Cargo.toml'))) {
    console.warn('[GenSense] Native binary not found for this platform. Falling back to "cargo run"...');
    run('cargo', ['run', '--release', '--', ...args]);
} else {
    console.error(`[GenSense CLI] Critical Error: Supported native binary not found for ${platform}-${arch}`);
    console.error(`Expected one of:\n${candidates.join('\n')}`);
    process.exit(1);
}

function run(command, runArgs) {
    const child = spawn(command, runArgs, { stdio: 'inherit' });
    child.on('close', (code) => process.exit(code));
    child.on('error', (err) => {
        console.error(`[GenSense CLI] Failed to launch: ${err.message}`);
        process.exit(1);
    });
}
