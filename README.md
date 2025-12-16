# 🦅 Lenguaje de Programación Águila (v2.7.5)

![Version](https://img.shields.io/badge/versión-2.7.5-blue)
![JIT](https://img.shields.io/badge/JIT-Nativo-green)
![Status](https://img.shields.io/badge/estado-Estable-success)

**Águila** es un lenguaje de programación dinámico, moderno y en español, diseñado para ser rápido y productivo. Su núcleo está escrito en **Rust** e incluye una Máquina Virtual (VM) de alto rendimiento con compilación Just-In-Time (JIT).

---

## 🚀 Lo Nuevo: Optimización JIT & Recursión Nativa

En la versión **v2.7.5**, hemos roto la barrera del rendimiento en algoritmos recursivos.

### ⚡️ Recursión Directa (Zero-Overhead)
El compilador JIT ahora detecta patrones recursivos (como Fibonacci) y genera código de máquina que se llama a sí mismo directamente, evitando por completo la sobrecarga de frames del intérprete.

| Benchmark | Águila v2.6 (Interpretado) | Águila v2.7.5 (JIT Nativo) | Mejora |
| :--- | :--- | :--- | :--- |
| **Fibonacci(35)** | 5.2 seg | **0.08 seg** | **65x** |
| **Fibonacci(40)** | > 30 seg | **~0.6 seg** | **🚀 50x+** |

> *El JIT utiliza "Integer Mode Optimization" para usar aritmética de CPU pura (i64/i32) cuando detecta operaciones matemáticas en bucles calientes.*

---

## 📚 Documentación Oficial

Hemos preparado documentación de nivel ingeniería para acelerar tu dominio del lenguaje:

*   📘 **[Manual de Referencia](docs/MANUAL.md)**: La especificación completa. Sintaxis, tipos, clases y módulos.
*   🐍 **[Águila vs Python](docs/GLOSARIO_PYTHON.md)**: Guía de migración directa para desarrolladores Python.
*   🎓 **[Ejemplos Profesionales](docs/EJEMPLOS_PROFESIONALES.md)**: Algoritmos avanzados (Dijkstra), Concurrencia y Patrones.
*   🪺 **[Guía Framework Nido](docs/guia_fullstack_nido.md)**: Desarrollo web Full-Stack profesional con Águila.

---

## ✅ Qué Funciona (Estado Actual)

### 1. Núcleo del Lenguaje
*   **Tipado Dinámico:** Variables flexibles (`let x = 10`, `x = "hola"`).
*   **Estructuras de Control:** `si/sino`, `mientras`, `para`, `segun`.
*   **Funciones:** Soporte de primera clase, closures y recursión nativa.
*   **POO:** Clases, Instancias, Herencia simple y Métodos.

### 2. Biblioteca Estándar (Stdlib)
Módulos nativos integrados y listos para usar:
*   `mate`: Funciones matemáticas, trigonométricas y estadísticas.
*   `http`: Cliente y Servidor HTTP robusto (basado en Hyper).
*   `json`: Parsing y serialización de alta velocidad.
*   `db`: Conectores para SQLite y PostgreSQL (con pool de conexiones).
*   `tiempo`, `os`, `archivo`, `net`, `thread`: Utilidades de sistema.

### 3. Framework Web "Nido" 🪺
Un framework MVC inspirado en Laravel/Rails, integrado en el lenguaje.
*   **CLI Potente:** `aguila crear api Usuario` genera Modelos, Controladores y Rutas automáticamente.
*   **ORM Ligero:** Mapeo automático de resultados de BD a objetos Águila.
*   **Inyección de Dependencias:** Gestión automática de conexiones a BD en Controladores.

### 4. Herramientas de Desarrollo (DX)
*   **VS Code Extension:** Resaltado de sintaxis, snippets y soporte para interpolación `a"Hola {nombre}"`.
*   **LSP (Language Server):** Autocompletado y detección de errores en tiempo real.
*   **REPL:** Consola interactiva con coloreado de sintaxis y autocompletado.

---

## 💻 Ejemplos

### Cálculo de Fibonacci (Recursivo)
```aguila
funcion fib(n) {
    si n <= 1 { retornar n }
    retornar fib(n - 1) + fib(n - 2)
}

let inicio = reloj()
imprimir("Calculando fib(40)...")
imprimir(fib(40)) 
imprimir("Tiempo: " + (reloj() - inicio) + "s")
```

### Servidor Web con Nido
```aguila
usar "http"

funcion controlador_home(req) {
    retornar {
        "estado": 200,
        "cuerpo": json.stringify({"mensaje": "Hola desde Águila v2.7.5 🦅"})
    }
}

let servidor = http.servidor(3000)
servidor.ruta("GET", "/", controlador_home)
servidor.iniciar()
```

---

## 📦 Instalación

Para instalar la última versión estable (macOS/Linux):

```bash
curl -fsSL https://aguila-lang.org/install.sh | sh
```

Para usuarios de Windows, descargar el ejecutable desde los [Releases de GitHub](https://github.com/emersonxinay/aguila/releases).
