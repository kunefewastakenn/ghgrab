#!/usr/bin/env node
const { spawn } = require('child_process');
const path = require('path');
const fs = require('fs');

const binaryName = 'ghgrab' + (process.platform === 'win32' ? '.exe' : '');
const binPath = path.join(__dirname, binaryName);

if (!fs.existsSync(binPath)) {
    console.error(`ghgrab binary not found at ${binPath}`);
    console.error('Try reinstalling: npm install -g @ghgrab/ghgrab');
    process.exit(1);
}

const child = spawn(binPath, process.argv.slice(2), {
    stdio: 'inherit',
    windowsHide: false
});

child.on('exit', (code) => {
    process.exit(code ?? 0);
});

child.on('error', (err) => {
    console.error(`Failed to run ghgrab: ${err.message}`);
    process.exit(1);
});
