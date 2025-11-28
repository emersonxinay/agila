# 🦅 ÁGUILA - Extensión para VS Code

Soporte oficial de VS Code para el lenguaje de programación **ÁGUILA**, un lenguaje moderno en español diseñado para ser intuitivo y educativo.

## ✨ Características

- 🎨 **Resaltado de sintaxis completo** para archivos `.ag`
- 🔧 **Autocompletado** de palabras clave y métodos nativos
- 📁 **Icono personalizado** para archivos ÁGUILA
- 🔄 **Auto-cierre** de paréntesis, llaves y corchetes
- 💬 **Comentarios** con `#`
- 🌈 **Soporte para interpolación de strings** con `a"..."`

## 📦 Instalación

Busca "Aguila" en el Marketplace de VS Code o instala desde la terminal:

```bash
code --install-extension aguila-lang.aguila-vscode
```

## 🚀 Uso Rápido

Crea un archivo con extensión `.ag` y comienza a programar:

```aguila
# Hola Mundo
imprimir "¡Hola, mundo!"

# Interpolación (Nuevo en v0.4.3)
nombre = "Águila"
imprimir a"Hola, {nombre}"

# Operadores aritméticos
potencia = 2 ** 3  # 8
division_entera = 10 // 3  # 3

# Estructuras de datos
numeros = [1, 2, 3, 4, 5]
numeros.agregar(6)
imprimir numeros.longitud()  # 6
```

## 🆕 Novedades en v0.4.3

- 🌈 **Interpolación de Cadenas:** Ahora usa el prefijo `a` y llaves `{}`. Ejemplo: `a"Hola {nombre}"`.
- 🛑 **Snippet 'romper':** Soporte para la nueva palabra clave de control de flujo.
- 🔧 **Correcciones de Snippets:** Arreglada la sintaxis de `segun` y añadidos `interp` e `impmod`.
- 📚 **Sintaxis Actualizada:** Soporte para asignación por índice `lista[0] = x`.

## ⌨️ Snippets y Atajos

### Snippets Disponibles
Escribe el prefijo y presiona Tab para expandir:

- `fun` → Función completa
- `si` → Condicional si
- `sisi` → Si-sino
- `para` → Bucle para-en
- `pararango` → Bucle numérico (0 hasta N)
- `mientras` → Bucle mientras
- `romper` → Salir del bucle
- `clase` → Clase con constructor
- `try` → Intentar-capturar
- `segun` → Switch/match
- `imp` → imprimir
- `interp` → Texto interpolado `a"..."`
- `impmod` → Importar módulo
- `dict` → Diccionario
- `conjunto` → Conjunto

### Atajos de Teclado
- **Ctrl+/** o **Cmd+/** → Comentar/descomentar línea con `#`
- **Ctrl+K Ctrl+C** → Comentar selección
- **Ctrl+K Ctrl+U** → Descomentar selección
- **Enter** en comentario → Auto-continúa con `# `

## 📚 Sintaxis Soportada

### Palabras Clave
- **Control de flujo:** `si`, `sino`, `mientras`, `para`, `en`, `hasta`, `segun`, `caso`, `defecto`, `romper`
- **Funciones:** `funcion`, `retornar`, `asincrono`, `esperar`
- **Clases:** `clase`, `nuevo`, `this`
- **Módulos:** `importar`
- **Errores:** `intentar`, `capturar`
- **Constantes:** `verdadero`, `falso`, `nulo`

### Tipos de Datos
- `Numero`, `Texto`, `Logico`, `Lista`, `Diccionario`, `Conjunto`

### Operadores
- **Aritméticos:** `+`, `-`, `*`, `/`, `//` (división entera), `%` (módulo), `**` (potencia)
- **Comparación:** `==`, `!=`, `>`, `<`, `>=`, `<=`
- **Lógicos:** `y`, `o`, `no`
- **Asignación:** `=`, `+=`, `-=`

### Métodos Nativos

**Listas:**
`.agregar()`, `.eliminar()`, `.insertar()`, `.longitud()`, `.contiene()`, `.ordenar()`, `.invertir()`, `.limpiar()`, `.copiar()`, `.unir()`, `.sublista()`, `.a_texto()`, `.suma()`, `.minimo()`, `.maximo()`, `.promedio()`

**Diccionarios:**
`.claves()`, `.valores()`, `.longitud()`, `.contiene()`, `.obtener()`, `.eliminar()`, `.limpiar()`, `.copiar()`, `.insertar()`

**Conjuntos (Sets):**
`.agregar()`, `.eliminar()`, `.contiene()`, `.longitud()`, `.unir()`, `.intersectar()`, `.diferencia()`, `.a_lista()`

**Texto:**
`.longitud()`, `.mayusculas()`, `.minusculas()`, `.contiene()`, `.reemplazar()`, `.dividir()`, `.recortar()`

**Números:**
`.abs()`, `.redondear()`, `.piso()`, `.techo()`

### Funciones Globales
- `imprimir()` - Imprime en consola
- `leer()` - Lee entrada del usuario (con inferencia de tipos)
- `afirmar()` - Aserciones para testing
- `conjunto()` - Crea un conjunto

## 🔗 Enlaces

- [Repositorio en GitHub](https://github.com/emersonxinay/aguila)
- [Documentación completa](https://github.com/emersonxinay/aguila/blob/main/DOCUMENTACION.md)
- [Reportar un problema](https://github.com/emersonxinay/aguila/issues)

## 📝 Licencia

MIT © 2025 Emerson Espinoza
