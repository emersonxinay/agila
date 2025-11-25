# ÁGUILA - Inicio Rápido

## ⚡ Pasos Rápidos (5 minutos)

### 1. Navegar al directorio
```bash
cd /Users/emersonespinoza/Documents/proyectos/proyecto_nuevo_lenguaje/aguila
```

### 2. Compilar el proyecto
```bash
/Users/emersonespinoza/.cargo/bin/cargo build
```

### 3. Crear un alias (opcional)
Para facilitar el uso, crea un alias en tu shell:

```bash
# En bash o zsh, agrega esto a ~/.bashrc o ~/.zshrc
alias aguila="/Users/emersonespinoza/Documents/proyectos/proyecto_nuevo_lenguaje/aguila/target/debug/aguila"

# Luego recarga:
source ~/.zshrc  # o ~/.bashrc
```

### 4. Ejecutar ejemplo
```bash
aguila ejecutar ejemplos/hola.ag
```

O sin alias:
```bash
./target/debug/aguila ejecutar ejemplos/hola.ag
```

### 5. Usar el REPL
```bash
aguila repl
```

---

## 📚 Ejemplos Disponibles

### `ejemplos/hola.ag` - Ejemplo Completo
Demuestra variables, booleanos, listas, condicionales, bucles, funciones y clases.

```bash
aguila ejecutar ejemplos/hola.ag
```

### `ejemplos/basico.ag` - Operaciones Básicas
Ejemplos simples de operaciones, condicionales y bucles.

```bash
aguila ejecutar ejemplos/basico.ag
```

### `ejemplos/funciones.ag` - Funciones
Ejemplos de definición y uso de funciones.

```bash
aguila ejecutar ejemplos/funciones.ag
```

### `ejemplos/poo.ag` - Programación Orientada a Objetos
Ejemplo de clases, instancias, atributos y métodos.

```bash
aguila ejecutar ejemplos/poo.ag
```

---

## 🔧 Comandos CLI

### Ver versión
```bash
aguila --version
```

### Ver ayuda
```bash
aguila --help
```

### Ejecutar archivo
```bash
aguila ejecutar <ruta/archivo.ag>
```

### REPL interactivo
```bash
aguila repl
```

---

## 📝 Crear tu Primer Programa

Crea un archivo llamado `mi_programa.ag`:

```aguila
nombre = "Emerson"
edad = 25

imprimir "Hola, " + nombre
imprimir "Tienes " + edad + " años"

si edad >= 18 {
    imprimir "Eres mayor de edad"
}
```

Luego ejecuta:

```bash
aguila ejecutar mi_programa.ag
```

---

## 🎯 Características Rápidas

### Variables
```
x = 10
nombre = "Juan"
activo = verdadero
```

### Funciones
```
funcion saludar(nombre) {
    imprimir "Hola " + nombre
}

saludar("Emerson")
```

### Clases
```
clase Persona {
    nombre
    edad: numero
}

juan = Persona()
juan.nombre = "Juan"
juan.edad = 30
```

### Bucles
```
para i = 0 hasta 5 {
    imprimir i
}

lista = [1, 2, 3]
para num en lista {
    imprimir num
}
```

### Condicionales
```
si x > 10 {
    imprimir "Mayor que 10"
} sino {
    imprimir "Menor o igual a 10"
}
```

---

## 🐛 Solución de Problemas

### Error: "command not found: aguila"
- Sin alias: usa `./target/debug/aguila`
- Con alias: asegúrate de que la ruta sea correcta

### Error: "cargo: command not found"
- Instala Rust: `curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh`
- Recarga el shell: `source ~/.zshrc`

### El archivo no se encuentra
- Asegúrate de usar la ruta completa o relativa correcta
- Ejemplo: `aguila ejecutar ./ejemplos/hola.ag`

---

## 📖 Documentación Completa

Para la documentación completa, consulta:
- `INSTRUCCIONES.md` - Guía de instalación y uso completa
- `RESUMEN.md` - Resumen del proyecto y características implementadas

---

**¡Disfruta programando en ÁGUILA!** 🦅
