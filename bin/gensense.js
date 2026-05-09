#!/usr/bin/env node

const { spawn } = require('child_process');
const path = require('path');
const os = require('os');

/**
 * GenSense CLI Wrapper
 * This script delegates to the native Rust CLI binary but handles path resolution
 * and basic Node-level environment setup.
 */

// Determine the binary path (local development vs installed package)
let binPath = path.join(__dirname, '..', 'dist', 'gensense');
if (os.platform() === 'win32') binPath += '.exe';

// Fallback to local build if dist is missing
if (!require('fs').existsSync(binPath)) {
    binPath = path.join(__dirname, '..', 'target', 'release', 'gensense');
}

const args = process.argv.slice(2);
const child = spawn(binPath, args, { stdio: 'inherit' });

child.on('close', (code) => {
  process.exit(code);
});

child.on('error', (err) => {
  console.error(`[GenSense CLI] Failed to launch native binary at ${binPath}`);
  console.error(err.message);
  process.exit(1);
});
