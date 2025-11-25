# ÁGUILA - Lenguaje de Programación Mínimo Funcional

## Descripción

ÁGUILA es un lenguaje de programación mínimo funcional implementado en Rust. Incluye un REPL interactivo y ejecución de scripts con extensión `.ag`.

## Características

✅ Ejecución de scripts `.ag`
✅ REPL funcional
✅ CLI completo
✅ Manejo de errores en español
✅ Variables sin tipado explícito
✅ Tipado opcional (`: tipo`)
✅ Tipo booleano con `verdadero` / `falso`
✅ Bucles `while` y `for` (rangos, listas)
✅ Funciones
✅ POO completa: clases, instancias, métodos y `self`

## Requisitos

- Rust 1.70 o superior
- Cargo

## Instalación

### 1. Instalar Rust

Si no tienes Rust instalado, ejecuta:

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

Luego, configura el shell:

```bash
. "$HOME/.cargo/env"
```

### 2. Compilar ÁGUILA

```bash
cd aguila
cargo build --release
```

El ejecutable estará en `target/release/aguila` (o en `target/debug/aguila` si usas `cargo build`).

## Uso

### Ejecutar un archivo

```bash
./target/debug/aguila ejecutar archivo.ag
```

O si lo prefieres:

```bash
./target/debug/aguila ejecutar ejemplos/hola.ag
```

### REPL Interactivo

```bash
./target/debug/aguila repl
```

Luego puedes escribir código línea por línea. Escribe `salir` para terminar.

### Ver versión

```bash
./target/debug/aguila --version
```

## Sintaxis

### Variables

```
x = 10
nombre = "Emerson"
activo = verdadero
```

### Tipado Opcional

```
x: numero = 10
nombre: texto = "Emerson"
activo: logico = verdadero
```

### Tipos Básicos

- `numero` - números flotantes
- `texto` - cadenas de texto
- `logico` - booleano (verdadero/falso)
- `lista` - arrays
- `diccionario` - objetos clave-valor
- `nulo` - valor nulo

### Funciones

```
funcion saludar(nombre) {
    imprimir "Hola " + nombre
}

saludar("Juan")
```

### Condicionales

```
si x > 5 {
    imprimir "Mayor que 5"
} sino {
    imprimir "Menor o igual a 5"
}
```

### Bucles

**While:**

```
mientras x < 5 {
    imprimir x
    x = x + 1
}
```

**For con rango:**

```
para i = 0 hasta 5 {
    imprimir i
}
```

**For sobre lista:**

```
numeros = [1, 2, 3]
para n en numeros {
    imprimir n
}
```

### Clases y Objetos

```
clase Persona {
    nombre
    edad: numero

    funcion saludar() {
        imprimir "Hola, soy " + self.nombre
    }
}

juan = Persona()
juan.nombre = "Juan"
juan.edad = 30
juan.saludar()
```

### Herencia

```
clase Empleado : Persona {
    sueldo: numero

    funcion mostrar_sueldo() {
        imprimir "Mi sueldo es " + self.sueldo
    }
}
```

### Imprimir

```
imprimir "Hola mundo"
imprimir 42
imprimir verdadero
```

### Comentarios

```
# Esto es un comentario
x = 10  # También funciona aquí
```

## Ejemplo Completo

```
imprimir "Hola desde ÁGUILA"

nombre = "Emerson"
edad = 25

funcion mostrar() {
    imprimir nombre
    imprimir edad
}

si edad >= 18 {
    imprimir "Mayor de edad"
}

para i = 0 hasta 3 {
    imprimir i
}

numeros = [10, 20, 30]
para n en numeros {
    imprimir n
}

clase Persona {
    nombre
    edad: numero

    funcion saludar() {
        imprimir "Hola, soy " + self.nombre
    }
}

juan = Persona()
juan.nombre = "Juan"
juan.edad = 30
juan.saludar()

mostrar()
```

## Estructura del Proyecto

```
aguila/
├── Cargo.toml           # Configuración de Rust
├── src/
│   ├── main.rs          # Punto de entrada y CLI
│   ├── cli.rs           # Módulo CLI
│   ├── lexer.rs         # Tokenizador
│   ├── parser.rs        # Parser sintáctico
│   ├── ast.rs           # Abstract Syntax Tree
│   ├── interpreter.rs   # Intérprete / Máquina Virtual
│   └── types.rs         # Tipos de datos
└── ejemplos/
    └── hola.ag          # Ejemplo funcional
```

## Archivos Generados

- `src/main.rs` - Punto de entrada
- `src/cli.rs` - Interfaz de línea de comandos
- `src/lexer.rs` - Tokenizador (~250 líneas)
- `src/parser.rs` - Parser (~480 líneas)
- `src/ast.rs` - Definiciones AST (~80 líneas)
- `src/interpreter.rs` - Intérprete (~390 líneas)
- `src/types.rs` - Tipos de datos (~75 líneas)
- `ejemplos/hola.ag` - Archivo de ejemplo

## Confirmación de Funcionamiento

El ejemplo `hola.ag` produce la siguiente salida:

```
Hola desde ÁGUILA
Usuario activo
0
1
2
10
20
30
Hola, soy Juan
Hola, soy Ana
Emerson
25
```

## Limitaciones Actuales

- No hay herencia completamente funcional aún
- No hay diccionarios con iteración `para clave, valor en dict`
- No hay constructores con `__init__` personalizados
- No hay control de flujo avanzado (break, continue)
- No hay manejo de excepciones

## Expansión Futura

El proyecto está diseñado para ser expandible a:

- Backend (servidor HTTP, APIs)
- Web (compilación a WebAssembly)
- Móvil (compilación a plataformas móviles)
- Módulos y sistemas de paquetes
- Bibliotecas estándar más completas

## Compilación y Ejecución

```bash
# Compilar en modo debug
cargo build

# Compilar en modo release
cargo build --release

# Ejecutar directamente
cargo run -- ejecutar archivo.ag
cargo run -- repl

# Ejecutar pruebas
cargo test
```

## Autores

- **Lenguaje ÁGUILA** - Implementado como proyecto de demostración

---

¡Disfruta programando en ÁGUILA! 🦅
