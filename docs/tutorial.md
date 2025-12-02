# 🦅 Tutorial Completo de Águila: De Cero a Experto

> **Proyecto Real:** Sistema de Biblioteca - Aprende Águila construyendo una aplicación completa de gestión de libros y préstamos.

---

## 📚 Índice

1. [Introducción y Configuración](#1-introducción-y-configuración)
2. [Variables y Tipos de Datos](#2-variables-y-tipos-de-datos)
3. [Operadores y Expresiones](#3-operadores-y-expresiones)
4. [Estructuras de Control](#4-estructuras-de-control)
5. [Funciones](#5-funciones)
6. [Listas y Diccionarios](#6-listas-y-diccionarios)
7. [Programación Orientada a Objetos](#7-programación-orientada-a-objetos)
8. [Persistencia de Datos (JSON + FS)](#8-persistencia-de-datos)
9. [Proyecto Final: Sistema Completo](#9-proyecto-final-sistema-completo)

---

## 1. Introducción y Configuración

### ¿Qué es Águila?

Águila es un lenguaje de programación moderno, expresivo y completamente en español. Diseñado para ser intuitivo y potente, combina la simplicidad de Python con características avanzadas como tipado gradual y programación orientada a objetos.

### Instalación

```bash
# Opción 1: NPM (Recomendado)
npm install -g aguila-lang
```

### Tu Primer Programa

Crea un archivo `hola.ag`:

```aguila
imprimir("¡Bienvenido a Águila! 🦅")
```

Ejecútalo:

```bash
aguila hola.ag
```

### El REPL Interactivo

Para experimentar rápidamente:

```bash
aguila
```

---

## 2. Variables y Tipos de Datos

### Variables Dinámicas

En Águila, no necesitas declarar el tipo de una variable:

```aguila
# Variables básicas
nombre = "Ana García"
edad = 28
es_estudiante = verdadero
saldo = 1500.50

imprimir(nombre)
imprimir(edad)
```

### Tipado Opcional

Para mayor claridad y seguridad, puedes especificar tipos:

```aguila
titulo: Texto = "El Quijote"
paginas: Numero = 863
disponible: Logico = verdadero

imprimir(titulo + " tiene " + paginas + " páginas")
```

### Tipos de Datos

| Tipo | Descripción | Ejemplo |
|------|-------------|---------|
| `Numero` | Enteros y decimales | `42`, `3.14` |
| `Texto` | Cadenas de caracteres | `"Hola"`, `a"Hola {nombre}"` |
| `Logico` | Booleanos | `verdadero`, `falso` |
| `Lista` | Colecciones ordenadas | `[1, 2, 3]` |
| `Diccionario` | Pares clave-valor | `{"nombre": "Ana"}` |
| `Nulo` | Ausencia de valor | `nulo` |

### 🎯 Ejercicio 1: Variables de Biblioteca

Crea un archivo `ejercicio1.ag`:

```aguila
# Información de un libro
titulo = "Cien Años de Soledad"
autor = "Gabriel García Márquez"
isbn = "978-0307474728"
paginas = 417
disponible = verdadero

imprimir("=== INFORMACIÓN DEL LIBRO ===")
imprimir("Título: " + titulo)
imprimir("Autor: " + autor)
imprimir("ISBN: " + isbn)
imprimir("Páginas: " + paginas)
imprimir("Disponible: " + disponible)
```

---

## 3. Operadores y Expresiones

### Operadores Aritméticos

```aguila
# Calculadora de multas de biblioteca
dias_retraso = 5
multa_por_dia = 2.50

multa_total = dias_retraso * multa_por_dia
imprimir("Multa total: $" + multa_total)  # $12.50

# Operadores avanzados
precio_libro = 100
descuento = precio_libro // 10  # División entera: 10
precio_final = precio_libro - descuento
imprimir("Precio con descuento: $" + precio_final)  # $90
```

### Operadores de Comparación

```aguila
edad_usuario = 16
edad_minima = 18

puede_registrarse = edad_usuario >= edad_minima
imprimir("¿Puede registrarse? " + puede_registrarse)  # falso

# Comparaciones múltiples
stock = 5
imprimir(stock > 0)  # verdadero
imprimir(stock == 0)  # falso
```

### Operadores Lógicos

```aguila
tiene_credencial = verdadero
debe_multas = falso

puede_prestar = tiene_credencial y no debe_multas
imprimir("¿Puede pedir prestado? " + puede_prestar)  # verdadero
```

### Interpolación de Cadenas

```aguila
nombre = "Carlos"
libros_prestados = 3

mensaje = a"Hola {nombre}, tienes {libros_prestados} libros prestados"
imprimir(mensaje)
# Salida: Hola Carlos, tienes 3 libros prestados
```

### 🎯 Ejercicio 2: Calculadora de Multas

```aguila
# ejercicio2.ag
nombre_usuario = "María López"
dias_retraso = 7
tarifa_diaria = 3.0

# Cálculo de multa
multa_base = dias_retraso * tarifa_diaria
recargo = multa_base * 0.1  # 10% de recargo
multa_total = multa_base + recargo

# Mostrar resultado
imprimir(a"Usuario: {nombre_usuario}")
imprimir(a"Días de retraso: {dias_retraso}")
imprimir(a"Multa base: ${multa_base}")
imprimir(a"Recargo (10%): ${recargo}")
imprimir(a"Total a pagar: ${multa_total}")
```

---

## 4. Estructuras de Control

### Condicionales: `si` / `sino`

```aguila
# Verificar disponibilidad de libro
libros_disponibles = 3

si libros_disponibles > 0 {
    imprimir("✅ Libro disponible para préstamo")
    libros_disponibles = libros_disponibles - 1
} sino {
    imprimir("❌ No hay copias disponibles")
}

imprimir(a"Quedan {libros_disponibles} copias")
```

### Condicionales Anidados

```aguila
edad = 15
tiene_permiso_padres = verdadero

si edad >= 18 {
    imprimir("Acceso completo a la biblioteca")
} sino si edad >= 13 y tiene_permiso_padres {
    imprimir("Acceso con permiso de padres")
} sino {
    imprimir("Acceso solo a sección infantil")
}
```

### Selección Múltiple: `segun`

```aguila
categoria = 2

segun categoria {
    caso 1 {
        imprimir("📚 Ficción")
    }
    caso 2 {
        imprimir("📖 No Ficción")
    }
    caso 3 {
        imprimir("🔬 Ciencia")
    }
    defecto {
        imprimir("❓ Categoría desconocida")
    }
}
```

### Bucle `mientras`

```aguila
# Procesar lista de espera
personas_en_espera = 5

mientras personas_en_espera > 0 {
    imprimir(a"Atendiendo... Quedan {personas_en_espera}")
    personas_en_espera = personas_en_espera - 1
}

imprimir("✅ Lista de espera procesada")
```

### Bucle `para` con Rango

```aguila
# Generar códigos de estantería
imprimir("Códigos de estantería:")

para i = 1 hasta 6 {
    codigo = a"EST-{i}"
    imprimir(codigo)
}
# Salida: EST-1, EST-2, EST-3, EST-4, EST-5
```

### 🎯 Ejercicio 3: Sistema de Categorías

```aguila
# ejercicio3.ag
edad_usuario = leer("¿Cuál es tu edad? ")
categoria_libro = leer("Categoría (1=Infantil, 2=Juvenil, 3=Adulto): ")

# Validación de acceso
puede_acceder = falso

si categoria_libro == 1 {
    puede_acceder = verdadero
} sino si categoria_libro == 2 {
    puede_acceder = edad_usuario >= 13
} sino si categoria_libro == 3 {
    puede_acceder = edad_usuario >= 18
}

si puede_acceder {
    imprimir("✅ Acceso permitido")
} sino {
    imprimir("❌ No tienes edad suficiente para esta categoría")
}
```

---

## 5. Funciones

### Funciones Básicas

```aguila
funcion saludar_usuario(nombre) {
    imprimir(a"¡Bienvenido a la biblioteca, {nombre}!")
}

saludar_usuario("Pedro")
```

### Funciones con Retorno

```aguila
funcion calcular_multa(dias) {
    tarifa = 2.5
    retornar dias * tarifa
}

multa = calcular_multa(10)
imprimir(a"Multa: ${multa}")  # Multa: $25.0
```

### Funciones con Tipos

```aguila
funcion calcular_descuento(precio: Numero, porcentaje: Numero) -> Numero {
    descuento = precio * (porcentaje / 100)
    retornar precio - descuento
}

precio_final = calcular_descuento(100, 15)
imprimir(a"Precio con descuento: ${precio_final}")  # $85.0
```

### Funciones con Múltiples Parámetros

```aguila
funcion registrar_prestamo(usuario, libro, dias) {
    imprimir("=== REGISTRO DE PRÉSTAMO ===")
    imprimir(a"Usuario: {usuario}")
    imprimir(a"Libro: {libro}")
    imprimir(a"Días permitidos: {dias}")
}

registrar_prestamo("Ana", "El Principito", 14)
```

### 🎯 Ejercicio 4: Biblioteca de Funciones

```aguila
# ejercicio4.ag

funcion validar_isbn(isbn) {
    longitud = isbn.longitud()
    retornar longitud == 13 o longitud == 10
}

funcion calcular_dias_retraso(fecha_devolucion, fecha_actual) {
    # Simplificado: asumimos que son números de días
    retraso = fecha_actual - fecha_devolucion
    
    si retraso > 0 {
        retornar retraso
    } sino {
        retornar 0
    }
}

funcion generar_reporte(nombre, libros_prestados, multa) {
    imprimir("╔════════════════════════════╗")
    imprimir("║   REPORTE DE USUARIO       ║")
    imprimir("╚════════════════════════════╝")
    imprimir(a"Nombre: {nombre}")
    imprimir(a"Libros activos: {libros_prestados}")
    imprimir(a"Multa pendiente: ${multa}")
}

# Uso
isbn_valido = validar_isbn("9780307474728")
imprimir(a"ISBN válido: {isbn_valido}")

dias_retraso = calcular_dias_retraso(15, 20)
imprimir(a"Días de retraso: {dias_retraso}")

generar_reporte("Carlos Ruiz", 2, 15.50)
```

---

## 6. Listas y Diccionarios

### Listas

```aguila
# Lista de libros disponibles
libros = ["El Quijote", "Cien Años de Soledad", "1984"]

# Acceso por índice
imprimir(libros[0])  # El Quijote

# Agregar elementos
libros.agregar("Rayuela")
imprimir(libros.longitud())  # 4

# Iterar sobre lista
para libro en libros {
    imprimir(a"📚 {libro}")
}
```

### Métodos de Listas

```aguila
numeros = [5, 2, 8, 1, 9]

# Ordenar
numeros.ordenar()
imprimir(numeros)  # [1, 2, 5, 8, 9]

# Invertir
numeros.invertir()
imprimir(numeros)  # [9, 8, 5, 2, 1]

# Verificar contenido
tiene_cinco = numeros.contiene(5)
imprimir(tiene_cinco)  # verdadero

# Sublista
primeros_tres = numeros.sublista(0, 3)
imprimir(primeros_tres)  # [9, 8, 5]
```

### Diccionarios

```aguila
# Información de un libro
libro = {
    "titulo": "El Principito",
    "autor": "Antoine de Saint-Exupéry",
    "año": 1943,
    "disponible": verdadero
}

# Acceso a valores
imprimir(libro["titulo"])  # El Principito

# Modificar valores
libro["disponible"] = falso

# Agregar nuevos campos
libro.insertar("prestado_a", "María")
```

### Métodos de Diccionarios

```aguila
libro = {"titulo": "1984", "autor": "Orwell", "paginas": 328}

# Obtener claves
claves = libro.claves()
imprimir(claves)  # ["titulo", "autor", "paginas"]

# Obtener valores
valores = libro.valores()
imprimir(valores)  # ["1984", "Orwell", 328]

# Verificar existencia
tiene_isbn = libro.contiene("isbn")
imprimir(tiene_isbn)  # falso
```

### 🎯 Ejercicio 5: Catálogo de Libros

```aguila
# ejercicio5.ag

# Crear catálogo
catalogo = []

# Función para agregar libro
funcion agregar_libro(titulo, autor, isbn) {
    libro = {
        "titulo": titulo,
        "autor": autor,
        "isbn": isbn,
        "disponible": verdadero,
        "prestamos": 0
    }
    catalogo.agregar(libro)
    imprimir(a"✅ Libro '{titulo}' agregado al catálogo")
}

# Función para buscar libro
funcion buscar_libro(titulo_buscar) {
    para libro en catalogo {
        si libro["titulo"] == titulo_buscar {
            retornar libro
        }
    }
    retornar nulo
}

# Función para listar todos
funcion listar_catalogo() {
    imprimir("\n=== CATÁLOGO DE BIBLIOTECA ===")
    contador = 1
    
    para libro en catalogo {
        estado = "✅ Disponible"
        si no libro["disponible"] {
            estado = "❌ Prestado"
        }
        
        imprimir(a"{contador}. {libro['titulo']} - {libro['autor']} {estado}")
        contador = contador + 1
    }
}

# Uso del sistema
agregar_libro("El Quijote", "Cervantes", "978-1234567890")
agregar_libro("Cien Años de Soledad", "García Márquez", "978-0987654321")
agregar_libro("1984", "George Orwell", "978-1111111111")

listar_catalogo()

# Buscar un libro
libro_encontrado = buscar_libro("1984")
si libro_encontrado != nulo {
    imprimir(a"\n📖 Encontrado: {libro_encontrado['titulo']} por {libro_encontrado['autor']}")
}
```

---

## 7. Programación Orientada a Objetos

### Clases Básicas

```aguila
clase Libro {
    titulo: Texto
    autor: Texto
    isbn: Texto
    disponible: Logico

    funcion iniciar(titulo, autor, isbn) {
        este.titulo = titulo
        este.autor = autor
        este.isbn = isbn
        este.disponible = verdadero
    }

    mostrar_info() {
        imprimir(a"📚 {este.titulo}")
        imprimir(a"   Autor: {este.autor}")
        imprimir(a"   ISBN: {este.isbn}")
        
        estado = "Disponible"
        si no este.disponible {
            estado = "Prestado"
        }
        imprimir(a"   Estado: {estado}")
    }

    prestar() {
        si este.disponible {
            este.disponible = falso
            imprimir("✅ Libro prestado exitosamente")
        } sino {
            imprimir("❌ El libro no está disponible")
        }
    }

    devolver() {
        este.disponible = verdadero
        imprimir("✅ Libro devuelto")
    }
}

# Uso
mi_libro = Libro("El Principito", "Saint-Exupéry", "978-0156012195")
mi_libro.mostrar_info()
mi_libro.prestar()
mi_libro.devolver()
```

### Herencia

```aguila
clase Usuario {
    nombre: Texto
    id: Numero
    activo: Logico

    funcion iniciar(nombre, id) {
        este.nombre = nombre
        este.id = id
        este.activo = verdadero
    }

    saludar() {
        imprimir(a"Hola, soy {este.nombre}")
    }
}

clase Estudiante : Usuario {
    carrera: Texto
    semestre: Numero

    funcion iniciar(nombre, id, carrera, semestre) {
        este.nombre = nombre
        este.id = id
        este.activo = verdadero
        este.carrera = carrera
        este.semestre = semestre
    }

    mostrar_perfil() {
        imprimir("=== PERFIL DE ESTUDIANTE ===")
        imprimir(a"Nombre: {este.nombre}")
        imprimir(a"ID: {este.id}")
        imprimir(a"Carrera: {este.carrera}")
        imprimir(a"Semestre: {este.semestre}")
    }
}

# Uso
estudiante = Estudiante("Ana García", 12345, "Ingeniería", 5)
estudiante.saludar()
estudiante.mostrar_perfil()
```

### 🎯 Ejercicio 6: Sistema POO Completo

```aguila
# ejercicio6.ag

clase Libro {
    titulo: Texto
    autor: Texto
    isbn: Texto
    disponible: Logico
    veces_prestado: Numero

    funcion iniciar(titulo, autor, isbn) {
        este.titulo = titulo
        este.autor = autor
        este.isbn = isbn
        este.disponible = verdadero
        este.veces_prestado = 0
    }

    a_diccionario() {
        retornar {
            "titulo": este.titulo,
            "autor": este.autor,
            "isbn": este.isbn,
            "disponible": este.disponible,
            "veces_prestado": este.veces_prestado
        }
    }
}

clase Usuario {
    nombre: Texto
    id: Numero
    libros_prestados: Lista

    funcion iniciar(nombre, id) {
        este.nombre = nombre
        este.id = id
        este.libros_prestados = []
    }

    prestar_libro(libro) {
        si libro.disponible {
            libro.disponible = falso
            libro.veces_prestado = libro.veces_prestado + 1
            este.libros_prestados.agregar(libro)
            imprimir(a"✅ '{libro.titulo}' prestado a {este.nombre}")
        } sino {
            imprimir(a"❌ '{libro.titulo}' no está disponible")
        }
    }

    devolver_libro(libro) {
        libro.disponible = verdadero
        imprimir(a"✅ '{libro.titulo}' devuelto por {este.nombre}")
    }

    mostrar_prestamos() {
        imprimir(a"\n📚 Libros de {este.nombre}:")
        si este.libros_prestados.longitud() == 0 {
            imprimir("   (No tiene libros prestados)")
        } sino {
            para libro en este.libros_prestados {
                imprimir(a"   - {libro.titulo}")
            }
        }
    }
}

# Crear biblioteca
libro1 = Libro("El Quijote", "Cervantes", "978-1234567890")
libro2 = Libro("1984", "Orwell", "978-0987654321")

usuario1 = Usuario("Carlos Ruiz", 1001)

# Simular préstamos
usuario1.prestar_libro(libro1)
usuario1.prestar_libro(libro2)
usuario1.mostrar_prestamos()

# Devolver un libro
usuario1.devolver_libro(libro1)
```

---

## 8. Persistencia de Datos

### Módulo FS (File System)

```aguila
# Escribir archivo
contenido = "Lista de libros:\n- El Quijote\n- 1984"
fs.escribir("libros.txt", contenido)

# Leer archivo
texto = fs.leer("libros.txt")
imprimir(texto)
```

### Módulo JSON

```aguila
# Convertir a JSON
libro = {
    "titulo": "El Principito",
    "autor": "Saint-Exupéry",
    "paginas": 96
}

texto_json = json.stringificar(libro)
imprimir(texto_json)
# {"titulo":"El Principito","autor":"Saint-Exupéry","paginas":96}

# Parsear JSON
libro_recuperado = json.parsear(texto_json)
imprimir(libro_recuperado["titulo"])  # El Principito
```

### Guardar y Cargar Datos

```aguila
clase BibliotecaDB {
    archivo: Texto
    libros: Lista

    funcion iniciar(archivo) {
        este.archivo = archivo
        este.libros = []
        este.cargar()
    }

    cargar() {
        intentar {
            contenido = fs.leer(este.archivo)
            este.libros = json.parsear(contenido)
            imprimir(a"✅ Cargados {este.libros.longitud()} libros")
        } capturar error {
            imprimir("ℹ️ No hay datos previos, iniciando nueva base")
            este.libros = []
        }
    }

    guardar() {
        texto = json.stringificar(este.libros)
        fs.escribir(este.archivo, texto)
        imprimir("💾 Datos guardados")
    }

    agregar_libro(libro_dict) {
        este.libros.agregar(libro_dict)
        este.guardar()
    }
}
```
