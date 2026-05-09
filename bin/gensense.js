#!/usr/bin/env node

const { spawn } = require('child_process');
const path = require('path');
const os = require('os');
const fs = require('fs');

/**
 * GenSense CLI Wrapper
 * This script delegates to the native Rust CLI binary.
 */

const isWin = os.platform() === 'win32';
const binName = isWin ? 'gensense.exe' : 'gensense';

// Resolution priority:
// 1. Local dist/ (installed package or after 'make dist')
// 2. Local target/release/ (after 'cargo build --release')
// 3. System PATH (if globally installed via other means)
// 4. 'cargo run' (dev fallback)

let binPath = path.join(__dirname, '..', 'dist', binName);

if (!fs.existsSync(binPath)) {
    binPath = path.join(__dirname, '..', 'target', 'release', binName);
}

const args = process.argv.slice(2);

if (fs.existsSync(binPath)) {
    run(binPath, args);
} else if (fs.existsSync(path.join(__dirname, '..', 'Cargo.toml'))) {
    console.warn('[GenSense] Native binary not found. Falling back to "cargo run"...');
    run('cargo', ['run', '--release', '--features', 'cli', '--', ...args]);
} else {
    console.error(`[GenSense CLI] Critical Error: Native binary not found at ${binPath}`);
    console.error('Please ensure the package was installed correctly or run "npm run build" if in development.');
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
