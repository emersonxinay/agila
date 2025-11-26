# ROADMAP COMPLETO – ÁGUILA (Lenguaje de Programación en Español)
Versión 1.0 – Ultra Detallado – Paso a Paso

## 🟦 FASE 1 — NÚCLEO DEL LENGUAJE (ÁGUILA Core)
Objetivo: obtener el lenguaje funcional mínimo, con POO, bucles, funciones y tipado dual.

### 1.1 Diseño del lenguaje (✅ Completado)
- [x] Sintaxis base en español
- [x] Tipado: dinámico + opcional (Dual)
- [x] Clases, herencia, métodos y self
- [x] Funciones globales
- [x] Control de flujo
- [x] Bucles (para, mientras)
- [x] Tipos primitivos
- [x] Comentarios
- [x] Imprimir básico

### 1.2 Implementación técnica del lenguaje
En Rust para velocidad y futuro compilador:
- [x] Crear proyecto Rust
- [x] CLI del lenguaje
- [x] Lexer completo
- [x] Parser (AST)
- [x] Intérprete inicial
- [x] Tabla de símbolos
- [x] Sistema de tipos dual (Sintaxis + Runtime Check)
- [x] Sistema de clases y objetos
- [ ] Herencia (Básica implementada, falta verificar robustez)
- [x] Bucles completos
- [x] Funciones
- [x] Llamadas de métodos
- [x] REPL en español
- [ ] Módulo de errores legible en español (`intentar/capturar`)

### 1.3 Tests de ÁGUILA Core
- [x] Pruebas de cada tipo
- [x] Pruebas de clases (Básicas)
- [x] Pruebas de for, while
- [x] Pruebas de errores de sintaxis y tipo

### 1.4 Documentación esencial
- [x] Manual ÁGUILA Core (README.md cubre lo básico)
- [ ] Ejemplos (Más allá de las pruebas)
- [ ] Primeros tutoriales

---

## 🟩 FASE 2 — COMPILADOR A JAVASCRIPT / WEBASSEMBLY
Para permitir frontend real, backend, móviles y deploys.

### 2.1 Compilador inicial (ÁGUILA → JS)
- [ ] Traductor de AST ÁGUILA a AST JavaScript
- [ ] Emisión de JS moderno (ES2023)
- [ ] Conversión de clases
- [ ] Conversión de estructuras de control
- [ ] Mapeo de tipos

### 2.2 Compilador avanzado
- [ ] Optimización
- [ ] Deducción de tipos mejorada
- [ ] Soporte para módulos
- [ ] Soporte para librerías estándar

### 2.3 Compilación a WASM (opcional futuro)
- [ ] Convertir el intérprete a WASM

---

## 🟧 FASE 3 — ESTÁNDAR DE LIBRERÍAS (ÁGUILA Standard Library)
### 3.1 Librerías base
- [ ] Matemáticas
- [/] Texto (Módulo `cadena` iniciado)
- [ ] Fechas
- [x] Archivos (Módulo `fs`)
- [ ] Concurrencia (futuro)
- [x] Estructuras avanzadas (Listas/Diccionarios nativos)

### 3.2 Módulo HTTP (para backend)
- [x] Servidor TCP (Módulo `red`)
- [/] Servidor HTTP (Script `http_server.ag` funcional, falta módulo nativo o stdlib)

---

## 🟨 FASE 4 — ÁGUILA WEB (Frontend)
### 4.1 Modelo similar a React
- [ ] Componentes y Estado
- [ ] Sintaxis `vista`

### 4.2 Compilación a JS + DOM
### 4.3 Sistema de estado
### 4.4 Sistema de componentes
### 4.5 Routing
### 4.6 Empaquetador

---

## 🟫 FASE 5 — ÁGUILA MÓVIL (Mobile)
- [ ] Compilar ÁGUILA → JS → móvil

---

## 🟪 FASE 6 — DEPLOY
- [ ] Deploy web/backend/móvil

---

## 🟥 FASE 7 — COMPILADOR NATIVO (Futuro)
- [ ] Compilar a binario (LLVM)
