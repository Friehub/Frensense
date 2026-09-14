#!/usr/bin/env node

const os = require('os');
const path = require('path');
const { spawnSync } = require('child_process');

const platform = os.platform();
const arch = os.arch();

let binaryName = 'frensense';
if (platform === 'win32') {
  binaryName += '-windows-x64.exe';
} else if (platform === 'darwin') {
  if (arch === 'arm64') {
    binaryName += '-macos-arm64';
  } else {
    binaryName += '-macos-x64';
  }
} else if (platform === 'linux') {
  // We only build x64 linux in CI currently
  binaryName += '-linux-x64';
} else {
  console.error(`Unsupported platform/architecture: ${platform}-${arch}`);
  process.exit(1);
}

// In development or early builds, 'dist/binaries/' might not exist, fallback to root dist/frensense
const binaryPath = path.join(__dirname, '..', 'dist', 'binaries', binaryName);

const result = spawnSync(binaryPath, process.argv.slice(2), {
  stdio: 'inherit'
});

if (result.error) {
  if (result.error.code === 'ENOENT') {
    console.error(`Failed to find native Frensense binary at ${binaryPath}`);
    console.error('Make sure the package was installed correctly or compiled for your platform.');
  } else {
    console.error(result.error);
  }
  process.exit(1);
}

process.exit(result.status ?? 0);
