
import * as vscode from 'vscode';
import * as https from 'https';
import * as fs from 'fs';
import * as path from 'path';
import { execSync } from 'child_process';
import {
    LanguageClient,
    LanguageClientOptions,
    ServerOptions,
    Executable
} from 'vscode-languageclient/node';

let client: LanguageClient;

export async function activate(context: vscode.ExtensionContext) {
    console.log('Águila extension is activating...');

    // 1. Verificación e Instalación
    const binaryPath = await ensureAguilaInstalled(context);

    if (!binaryPath) {
        console.log('Águila binary not available. Extension features limited.');
        return;
    }

    // 2. Iniciar LSP (si está habilitado)
    const config = vscode.workspace.getConfiguration('aguila');
    if (config.get<boolean>('enableLSP')) {
        await startLanguageServer(context, binaryPath);
    }
}

export function deactivate(): Thenable<void> | undefined {
    if (!client) {
        return undefined;
    }
    return client.stop();
}

async function startLanguageServer(context: vscode.ExtensionContext, command: string) {
    // Configuración del Servidor
    const serverOptions: ServerOptions = {
        run: { command: command, args: ["lsp"] },
        debug: { command: command, args: ["lsp"] } // Podríamos añadir flags de debug si fuese necesario
    };

    // Configuración del Cliente
    const clientOptions: LanguageClientOptions = {
        documentSelector: [{ scheme: 'file', language: 'aguila' }],
        synchronize: {
            fileEvents: vscode.workspace.createFileSystemWatcher('**/.clientrc')
        }
    };

    // Crear y arrancar cliente
    client = new LanguageClient(
        'aguilaLanguageServer',
        'Servidor de Lenguaje Águila',
        serverOptions,
        clientOptions
    );

    // Iniciar
    console.log(`Starting LSP with command: ${command} lsp`);
    await client.start();

    // Mostrar status item
    const statusBarItem = vscode.window.createStatusBarItem(vscode.StatusBarAlignment.Right, 100);
    statusBarItem.text = "$(check) Águila LSP";
    statusBarItem.tooltip = "Servidor de lenguaje Águila activo";
    statusBarItem.show();
    context.subscriptions.push(statusBarItem);
}

// --- Lógica de Instalación (Refactorizada y Mejorada) ---

async function ensureAguilaInstalled(context: vscode.ExtensionContext): Promise<string | null> {
    const config = vscode.workspace.getConfiguration('aguila');
    let customPath = config.get<string>('serverPath');

    // 1. Si el usuario definió un path, usar ese.
    if (customPath && customPath.trim() !== "") {
        if (fs.existsSync(customPath)) return customPath;
        vscode.window.showErrorMessage(`Ruta de Águila configurada no válida: ${customPath}`);
        return null; // O fallback? Mejor fallar explícitamente si el usuario lo configuró.
    }

    // 2. Intentar usar binary del sistema (PATH)
    if (isAguilaAvailable('aguila')) {
        return 'aguila';
    }

    // 3. Revisar instalación local de la extensión (bin folder)
    const binDir = path.join(context.globalStorageUri.fsPath, 'bin');
    const platformBin = process.platform === 'win32' ? 'aguila.exe' : 'aguila';
    const localBinPath = path.join(binDir, platformBin);

    if (fs.existsSync(localBinPath)) {
        // Asegurar permisos
        if (process.platform !== 'win32') {
            try { fs.chmodSync(localBinPath, '755'); } catch { }
        }
        return localBinPath;
    }

    // 4. No encontrado. Preguntar si instalar.
    const selection = await vscode.window.showInformationMessage(
        'El compilador de Águila no se encontró. ¿Deseas instalarlo automáticamente para habilitar autocompletado y errores?',
        'Sí, instalar', 'No'
    );

    if (selection === 'Sí, instalar') {
        return await installAguila(context, binDir, platformBin);
    }

    return null;
}

function isAguilaAvailable(cmd: string): boolean {
    try {
        execSync(`${cmd} --version`, { stdio: 'ignore' });
        return true;
    } catch {
        return false;
    }
}

async function installAguila(context: vscode.ExtensionContext, binDir: string, binaryName: string): Promise<string | null> {
    const platform = process.platform;
    let downloadUrl = '';

    // HARDCODED VERSION por ahora (Idealmente buscar latest de GitHub API)
    const AGUILA_VERSION = 'v2.7.6'; // Versión a descargar
    const BASE_URL = `https://github.com/emersonxinay/aguila/releases/download/${AGUILA_VERSION}`;

    if (platform === 'darwin') {
        downloadUrl = `${BASE_URL}/aguila-macos`;
    } else if (platform === 'linux') {
        downloadUrl = `${BASE_URL}/aguila-linux`;
    } else if (platform === 'win32') {
        downloadUrl = `${BASE_URL}/aguila-windows.exe`;
    } else {
        vscode.window.showErrorMessage(`Plataforma ${platform} no soportada para instalación auto.`);
        return null;
    }

    return await vscode.window.withProgress({
        location: vscode.ProgressLocation.Notification,
        title: `Instalando Águila ${AGUILA_VERSION}...`,
        cancellable: false
    }, async (progress) => {
        try {
            if (!fs.existsSync(binDir)) {
                fs.mkdirSync(binDir, { recursive: true });
            }
            const destPath = path.join(binDir, binaryName);

            progress.report({ message: "Descargando..." });
            await downloadFile(downloadUrl, destPath);

            if (platform !== 'win32') {
                fs.chmodSync(destPath, '755');
            }
            vscode.window.showInformationMessage(`Águila ${AGUILA_VERSION} descargado y configurado correctamente.`);
            return destPath;
        } catch (error) {
            vscode.window.showErrorMessage(`Error instalando Águila: ${error}`);
            return null;
        }
    });
}

function downloadFile(url: string, dest: string): Promise<void> {
    return new Promise((resolve, reject) => {
        const file = fs.createWriteStream(dest);
        const request = https.get(url, (response) => {
            if (response.statusCode === 302 || response.statusCode === 301) {
                // Handle redirect
                downloadFile(response.headers.location!, dest).then(resolve).catch(reject);
                return;
            }

            if (response.statusCode !== 200) {
                reject(new Error(`Falló la descarga: ${response.statusCode} ${response.statusMessage}`));
                return;
            }
            response.pipe(file);
            file.on('finish', () => {
                file.close();
                resolve();
            });
        });
        request.on('error', (err) => {
            fs.unlink(dest, () => { });
            reject(err);
        });
    });
}
