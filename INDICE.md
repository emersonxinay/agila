# ÍNDICE DE DOCUMENTACIÓN - PROYECTO ÁGUILA

## 📖 Documentos Disponibles

### 1. **INICIO_RAPIDO.md** 🚀
**Propósito:** Empezar en 5 minutos
- Pasos de instalación y compilación
- Comandos básicos
- Ejemplos rápidos
- Solución de problemas comunes
- **Leer primero si tienes prisa**

### 2. **INSTRUCCIONES.md** 📚
**Propósito:** Guía completa de instalación y uso
- Requisitos (Rust, Cargo)
- Instalación paso a paso
- Compilación y ejecución
- Sintaxis completa del lenguaje
- Todos los comandos CLI
- **Leer para entender completamente el proyecto**

### 3. **RESUMEN.md** 📋
**Propósito:** Resumen ejecutivo
- Características implementadas
- Componentes principales
- Estadísticas del proyecto
- Requisitos completados
- Pruebas realizadas
- **Leer para obtener una visión general**

### 4. **REFERENCIA.md** 🔍
**Propósito:** Referencia rápida del lenguaje
- Tipos de datos
- Variables y operadores
- Control de flujo
- Funciones y clases
- Ejemplos rápidos
- Mejor práctica
- **Leer cuando necesites recordar sintaxis**

### 5. **ESTRUCTURA.txt** 🏗️
**Propósito:** Estructura técnica del proyecto
- Organización de archivos
- Descripción de cada módulo
- Estadísticas del código
- Flujo de ejecución
- Características implementadas
- **Leer para entender la arquitectura**

### 6. **ENTREGA_FINAL.md** ✅
**Propósito:** Confirmación de entrega
- Contenido completo
- Checklist de características
- Pruebas realizadas
- Estadísticas finales
- Próximos pasos opcionales
- **Leer para confirmar completitud**

### 7. **EJEMPLOS_SALIDA.txt** 💻
**Propósito:** Ejemplos de ejecución y salida
- Salida de cada ejemplo
- Pruebas adicionales
- Manejo de errores
- **Consultar para ver resultados esperados**

### 8. **INDICE.md** 📑
**Propósito:** Este documento
- Guía de documentación
- Explicación de cada archivo
- Recomendaciones de lectura

---

## 🎯 RECOMENDACIONES DE LECTURA

### Si Quieres Empezar Rápido (5 minutos)
1. Lee **INICIO_RAPIDO.md**
2. Ejecuta un ejemplo: `aguila ejecutar ejemplos/basico.ag`
3. Prueba el REPL: `aguila repl`

### Si Quieres Entender Completamente
1. Lee **INICIO_RAPIDO.md** (5 min)
2. Lee **INSTRUCCIONES.md** (15 min)
3. Lee **REFERENCIA.md** (10 min)
4. Explora el código en `aguila/src/`

### Si Quieres Conocer la Arquitectura
1. Lee **ESTRUCTURA.txt**
2. Lee **RESUMEN.md** - Sección "Componentes Principales"
3. Examina el código fuente en orden:
   - `src/ast.rs` - Definiciones
   - `src/lexer.rs` - Tokenización
   - `src/parser.rs` - Análisis sintáctico
   - `src/interpreter.rs` - Ejecución

### Si Necesitas Buscar Sintaxis
1. Consulta **REFERENCIA.md**
2. Mira los ejemplos en `aguila/ejemplos/`
3. Revisa **EJEMPLOS_SALIDA.txt**

---

## 📂 ESTRUCTURA DE ARCHIVOS

```
proyecto_nuevo_lenguaje/
├── 📄 INDICE.md              ← Este archivo
├── 📄 INICIO_RAPIDO.md       ← Comienza aquí
├── 📄 INSTRUCCIONES.md       ← Guía completa
├── 📄 REFERENCIA.md          ← Sintaxis rápida
├── 📄 RESUMEN.md             ← Resumen ejecutivo
├── 📄 ESTRUCTURA.txt         ← Arquitectura
├── 📄 ENTREGA_FINAL.md       ← Confirmación
├── 📄 EJEMPLOS_SALIDA.txt    ← Salidas esperadas
│
└── aguila/                   ← Proyecto Rust
    ├── src/                  ← Código fuente
    ├── ejemplos/             ← 4 ejemplos funcionales
    └── target/               ← Compilados
```

---

## ✨ CARACTERÍSTICAS CLAVE

### Lenguaje
- Variables implícitas y tipadas
- Tipos: numero, texto, logico, lista, diccionario, nulo
- Operadores: +, -, *, /, ==, !=, >, <, >=, <=

### Control de Flujo
- Condicionales: si / sino
- Bucles: mientras, para (rango, lista, diccionario)
- Funciones con parámetros
- Clases con métodos y self

### Entrada/Salida
- `imprimir` para mostrar valores
- Comentarios con `#`
- REPL interactivo
- Ejecución de archivos `.ag`

### CLI
- `aguila ejecutar archivo.ag`
- `aguila repl`
- `aguila --version`
- `aguila --help`

---

## 🔧 HERRAMIENTAS NECESARIAS

- **Rust 1.70+** - Lenguaje de programación
- **Cargo** - Administrador de paquetes
- **Bash/Zsh** - Terminal (Linux/macOS)
- **PowerShell** - Terminal (Windows)

---

## 🚀 COMANDOS RÁPIDOS

```bash
# Instalar Rust
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh

# Compilar
cd aguila && cargo build

# Ejecutar ejemplo
./target/debug/aguila ejecutar ejemplos/basico.ag

# REPL
./target/debug/aguila repl

# Ver versión
./target/debug/aguila --version
```

---

## 📊 ESTADÍSTICAS

| Métrica | Valor |
|---------|-------|
| Líneas de código | ~1,300 |
| Archivos Rust | 7 |
| Documentos | 8 |
| Ejemplos | 4 |
| Palabras clave | 13 |

---

## ❓ PREGUNTAS FRECUENTES

### ¿Por dónde empiezo?
Lee **INICIO_RAPIDO.md** - te toma 5 minutos.

### ¿Cuáles son todos los comandos?
Consulta **REFERENCIAS.md** o ejecuta `aguila --help`

### ¿Cómo compilo el proyecto?
Lee **INSTRUCCIONES.md** - Sección "Compilación"

### ¿Cuál es la sintaxis de las clases?
Consulta **REFERENCIA.md** - Sección "Clases"

### ¿Cuáles son las características completas?
Lee **ENTREGA_FINAL.md** o **RESUMEN.md**

### ¿Cómo funciona internamente?
Lee **ESTRUCTURA.txt** - Sección "Flujo de Ejecución"

---

## 🎓 RUTA DE APRENDIZAJE

### Nivel 1: Principiante (1 hora)
1. Lee INICIO_RAPIDO.md
2. Ejecuta ejemplos
3. Juega con el REPL
4. Modifica ejemplos

### Nivel 2: Intermedio (2 horas)
1. Lee INSTRUCCIONES.md
2. Lee REFERENCIA.md
3. Crea tu primer programa
4. Experimenta con todas las características

### Nivel 3: Avanzado (4 horas)
1. Lee ESTRUCTURA.txt
2. Lee RESUMEN.md
3. Estudia el código fuente
4. Implementa mejoras

---

## 📝 CÓMO CREAR TU PRIMER PROGRAMA

1. Crea archivo `mi_programa.ag`
2. Escribe código ÁGUILA
3. Ejecuta: `aguila ejecutar mi_programa.ag`
4. Consulta REFERENCIA.md para sintaxis

---

## 🐛 SOPORTE Y PROBLEMAS

### Error: "rustc not found"
- Instala Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Recarga shell: `source ~/.zshrc` o `source ~/.bashrc`

### Error: "cargo not found"
- Mismo que arriba - instala Rust

### Error: "command not found: aguila"
- Usa ruta completa: `./target/debug/aguila`
- O crea alias como se describe en INICIO_RAPIDO.md

### Mi programa no compila
- Revisa REFERENCIA.md para sintaxis correcta
- Verifica que uses `imprimir` (no `print`)

---

## ✅ VERIFICACIÓN

Para verificar que todo está instalado y funcionando:

```bash
# Ver estructura
ls -la aguila/src/

# Ver ejemplos
ls -la aguila/ejemplos/

# Compilar
cargo build

# Ejecutar
./target/debug/aguila ejecutar ejemplos/basico.ag
```

---

## 📞 RESUMEN

| Necesidad | Documento | Tiempo |
|-----------|-----------|--------|
| Empezar rápido | INICIO_RAPIDO.md | 5 min |
| Guía completa | INSTRUCCIONES.md | 15 min |
| Referencia sintaxis | REFERENCIA.md | 10 min |
| Visión general | RESUMEN.md | 10 min |
| Arquitectura | ESTRUCTURA.txt | 10 min |
| Confirmación | ENTREGA_FINAL.md | 5 min |
| Ejemplos | EJEMPLOS_SALIDA.txt | - |

---

## 🎉 ¡BIENVENIDO A ÁGUILA!

El proyecto está completamente funcional y documentado.
Elige un documento de arriba y ¡comienza!

Para empezar ahora: **Lee INICIO_RAPIDO.md** 🚀

---

**ÁGUILA v0.1.0** | Proyecto Completo | 20 Noviembre 2025
