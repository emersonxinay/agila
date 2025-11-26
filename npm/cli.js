#!/usr/bin/env node

const { spawn } = require('child_process');
const path = require('path');

const BIN_NAME = process.platform === 'win32' ? 'aguila.exe' : 'aguila';
const binPath = path.join(__dirname, BIN_NAME);

// Pasar todos los argumentos al binario nativo
const child = spawn(binPath, process.argv.slice(2), {
    stdio: 'inherit' // Conectar entrada/salida directamente
});

child.on('exit', (code) => {
    process.exit(code);
});
