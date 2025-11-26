# ÁGUILA Language Extension for VS Code

Extensión oficial de VS Code para el lenguaje de programación ÁGUILA.

## Características

- **Resaltado de sintaxis** para archivos `.ag`
- **Icono personalizado** para archivos ÁGUILA en el explorador
- **Auto-cierre** de paréntesis, llaves y corchetes
- **Comentarios** con `//`

## Instalación

### Opción 1: Instalación Manual (Desarrollo)
1. Copia la carpeta `aguila-vscode` a `~/.vscode/extensions/`
2. Reinicia VS Code

### Opción 2: Desde VSIX (Próximamente)
```bash
code --install-extension aguila-vscode-0.1.0.vsix
```

## Uso

Abre cualquier archivo con extensión `.ag` y el resaltado de sintaxis se aplicará automáticamente.

## Palabras Clave Soportadas

- Control de flujo: `si`, `sino`, `mientras`, `para`, `retornar`
- Funciones: `funcion`, `asincrono`, `esperar`
- Clases: `clase`, `nuevo`, `this`
- Módulos: `importar`
- Manejo de errores: `intentar`, `capturar`
- Constantes: `verdadero`, `falso`, `nulo`

## Licencia

MIT
