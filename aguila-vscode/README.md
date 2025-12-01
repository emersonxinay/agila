# Águila - Programación en Español, Potencia de Rust 🦅

![Logo Águila](https://raw.githubusercontent.com/emersonxinay/aguila/main/logo_aguila.svg)

> **"Vuela alto, programa simple."**

Esta es la extensión oficial para **Visual Studio Code** del lenguaje de programación **Águila**. Diseñada para ofrecer una experiencia de desarrollo de primera clase, combinando la simplicidad de Python con la velocidad de Rust, todo en tu idioma.

## ✨ Características Principales

### 🎨 Resaltado de Sintaxis Completo
Disfruta de una lectura de código clara y moderna. La extensión reconoce:
*   **Palabras clave en español:** `si`, `mientras`, `funcion`, `clase`, `imprime`.
*   **Tipos de datos y literales:** Números, textos, booleanos (`verdadero`, `falso`).
*   **Interpolación de cadenas:** `f"Hola {nombre}"`.
*   **Comentarios:** `# Esto es un comentario`.

### ⚡ Snippets Inteligentes
Escribe código a la velocidad del pensamiento con nuestros atajos:
*   `imprime` ➝ `imprime("...")`
*   `func` ➝ Estructura completa de función.
*   `si` / `sino` ➝ Estructuras de control condicionales.
*   `clase` ➝ Plantilla para Programación Orientada a Objetos.
*   `para` / `mientras` ➝ Bucles optimizados.

---

## 🚀 El Potencial de Águila

Águila no es solo un lenguaje educativo; es una herramienta poderosa. Mira lo que puedes hacer:

### 1. Algoritmos Clásicos (Sintaxis Limpia)
```aguila
funcion fibonacci(n) {
    si n <= 1 { retornar n }
    retornar fibonacci(n - 1) + fibonacci(n - 2)
}

imprime(f"Fibonacci de 10 es: {fibonacci(10)}")
```

### 2. Programación Orientada a Objetos
```aguila
clase Animal {
    funcion sonido() {
        imprime("Hace un sonido")
    }
}

clase Perro hereda Animal {
    funcion sonido() {
        imprime("Guau!")
    }
}

mi_perro = nuevo Perro()
mi_perro.sonido() # Salida: Guau!
```

### 3. Alto Rendimiento
Gracias a su **JIT Compiler** y gestión de memoria optimizada (Generational Arena), Águila ejecuta código complejo a velocidades comparables con lenguajes compilados modernos.

---

## 📦 Instalación

Para sacar el máximo provecho, necesitas el compilador de Águila instalado en tu sistema:

1.  **Instalar Compilador (vía NPM):**
    ```bash
    npm install -g aguila-lang
    ```
2.  **Instalar esta Extensión:**
    Busca "Águila" en el Marketplace de VS Code e instala.

---

## 🔗 Enlaces y Recursos

*   **Repositorio Oficial:** [github.com/emersonxinay/aguila](https://github.com/emersonxinay/aguila)
*   **Reportar Errores:** [Issues](https://github.com/emersonxinay/aguila/issues)
*   **Autor:** [Emerson Espinoza](https://github.com/emersonxinay)

## 📄 Licencia

Este proyecto está bajo la licencia **MIT**. Eres libre de usarlo, modificarlo y compartirlo.
Consulta el archivo `LICENSE` en el repositorio para más detalles.

---
Hecho con ❤️ por **Emerson Espinoza**.
