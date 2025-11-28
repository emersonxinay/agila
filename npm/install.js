const fs = require('fs');
const path = require('path');
const https = require('https');
const { execSync } = require('child_process');

// Configuración
const REPO = 'emersonxinay/aguila';
const BIN_NAME = process.platform === 'win32' ? 'aguila.exe' : 'aguila';

console.log('🦅 Detectando última versión de ÁGUILA...');

// Obtener la última release desde GitHub API
function getLatestRelease() {
    return new Promise((resolve, reject) => {
        const options = {
            hostname: 'api.github.com',
            path: `/repos/${REPO}/releases/latest`,
            method: 'GET',
            headers: {
                'User-Agent': 'aguila-installer'
            }
        };

        https.get(options, (res) => {
            let data = '';

            res.on('data', (chunk) => {
                data += chunk;
            });

            res.on('end', () => {
                if (res.statusCode === 200) {
                    const release = JSON.parse(data);
                    resolve(release);
                } else {
                    reject(new Error(`Error al obtener release: ${res.statusCode}`));
                }
            });
        }).on('error', reject);
    });
}

// Detectar asset correcto según plataforma
function getAssetForPlatform(assets) {
    const platform = process.platform;
    let assetName;

    if (platform === 'win32') assetName = 'aguila-windows.exe';
    else if (platform === 'darwin') assetName = 'aguila-macos';
    else if (platform === 'linux') assetName = 'aguila-linux';
    else throw new Error(`Plataforma no soportada: ${platform}`);

    const asset = assets.find(a => a.name === assetName);
    if (!asset) {
        throw new Error(`No se encontró binario para ${platform}`);
    }

    return asset.browser_download_url;
}

const finalDest = path.join(__dirname, BIN_NAME);

function download(url, dest) {
    return new Promise((resolve, reject) => {
        const req = https.get(url, (res) => {
            if (res.statusCode === 301 || res.statusCode === 302) {
                // Seguir redirección
                download(res.headers.location, dest).then(resolve).catch(reject);
                return;
            }

            if (res.statusCode !== 200) {
                reject(new Error(`Falló la descarga con código: ${res.statusCode}`));
                return;
            }

            const file = fs.createWriteStream(dest);
            res.pipe(file);

            file.on('finish', () => {
                file.close();
                resolve();
            });

            file.on('error', (err) => {
                fs.unlink(dest, () => { });
                reject(err);
            });
        });

        req.on('error', (err) => {
            reject(err);
        });
    });
}

// Proceso principal
getLatestRelease()
    .then((release) => {
        console.log(`✨ Última versión: ${release.tag_name}`);
        console.log(`📦 Instalando ÁGUILA para ${process.platform}...`);

        const downloadUrl = getAssetForPlatform(release.assets);
        return download(downloadUrl, finalDest).then(() => release);
    })
    .then((release) => {
        if (process.platform !== 'win32') {
            execSync(`chmod +x ${finalDest}`);
        }
        console.log('\n=============================================');
        console.log(`🦅  ¡ÁGUILA SE HA ACTUALIZADO EXITOSAMENTE!`);
        console.log(`✨  Versión instalada: ${release.tag_name}`);
        console.log('=============================================');
        console.log('👉  Ejecuta "aguila --version" para verificar.');
        console.log('👉  Ejecuta "aguila --ayuda" para comenzar.\n');
    })
    .catch((err) => {
        console.error(`❌ Error: ${err.message}`);
        console.error('💡 Intenta instalar manualmente desde: https://github.com/emersonxinay/aguila/releases');
        process.exit(1);
    });
