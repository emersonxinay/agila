# ÁGUILA - Referencia Rápida del Lenguaje

## Tipos de Datos

```
numero    → 42, 3.14, -10
texto     → "Hola", 'mundo', concatenación con +
logico    → verdadero, falso
lista     → [1, 2, 3], acceso con índice
diccionario → {"clave": valor}
nulo      → nulo
```

## Variables

```
x = 10                    # Variable sin tipo
nombre: texto = "Juan"    # Variable con tipo
edad: numero = 25
activo: logico = verdadero
```

## Operadores

```
# Aritméticos
x + y      # Suma
x - y      # Resta
x * y      # Multiplicación
x / y      # División

# Comparación
x == y     # Igual
x != y     # No igual
x > y      # Mayor que
x < y      # Menor que
x >= y     # Mayor o igual
x <= y     # Menor o igual

# Lógicos
verdadero
falso
```

## Control de Flujo

```
# Condicional
si condicion {
    # bloque si verdadero
} sino {
    # bloque si falso
}

# While
mientras condicion {
    # código
}

# For con rango
para i = 0 hasta 10 {
    # código (i = 0, 1, 2, ..., 9)
}

# For sobre lista
para elemento en lista {
    # código
}

# For sobre diccionario
para clave en diccionario {
    # código
}
```

## Funciones

```
funcion nombre() {
    imprimir "sin parámetros"
}

funcion sumar(a, b) {
    imprimir a + b
}

funcion tipos(x: numero, y: texto) {
    # con tipos
}

nombre()          # Llamada sin argumentos
sumar(5, 10)      # Llamada con argumentos
```

## Clases

```
clase Persona {
    nombre          # atributo
    edad: numero    # atributo con tipo

    funcion saludar() {
        imprimir "Hola, " + self.nombre
    }
}

# Instancia
juan = Persona()
juan.nombre = "Juan"
juan.edad = 30
juan.saludar()
```

## Herencia

```
clase Empleado : Persona {
    sueldo: numero

    funcion mostrar_sueldo() {
        imprimir "Sueldo: " + self.sueldo
    }
}

emp = Empleado()
emp.nombre = "Carlos"
emp.edad = 35
emp.sueldo = 2000
emp.saludar()
emp.mostrar_sueldo()
```

## Entrada/Salida

```
imprimir "Texto"
imprimir 42
imprimir verdadero
imprimir [1, 2, 3]
```

## Comentarios

```
# Comentario de línea
x = 10  # También aquí

# Variables locales
nombre = "Emerson"  # variable local en scope actual
```

## Ejemplos Rápidos

### Ejemplo 1: Bucle simple
```
para i = 0 hasta 5 {
    imprimir i
}
```

### Ejemplo 2: Lista
```
numeros = [1, 2, 3, 4, 5]
para n en numeros {
    imprimir n
}
```

### Ejemplo 3: Función
```
funcion cuadrado(x) {
    imprimir x * x
}

cuadrado(5)  # Imprime: 25
```

### Ejemplo 4: Clase
```
clase Coche {
    marca
    modelo

    funcion info() {
        imprimir self.marca + " " + self.modelo
    }
}

mi_coche = Coche()
mi_coche.marca = "Toyota"
mi_coche.modelo = "Corolla"
mi_coche.info()  # Imprime: Toyota Corolla
```

### Ejemplo 5: Condicional
```
edad = 25

si edad >= 18 {
    imprimir "Mayor de edad"
} sino {
    imprimir "Menor de edad"
}
```

## CLI

```bash
# Ver versión
aguila --version

# Ver ayuda
aguila --help

# Ejecutar archivo
aguila ejecutar archivo.ag

# REPL interactivo
aguila repl
```

## Conversiones

Todas las conversiones son automáticas:

```
numero + texto = texto
texto + numero = texto
numero == numero = logico
lista + lista = ERROR
```

## Ámbitos (Scopes)

- Variables globales: fuera de funciones/clases
- Variables locales: dentro de funciones/clases/bloques
- `self`: dentro de métodos para acceder a atributos

## Límites Conocidos

- No hay constructores personalizados (`__init__`)
- La herencia es básica
- No hay diccionario con iteración de clave-valor
- No hay control de excepciones
- No hay async/await

## Mejores Prácticas

```
# ✓ Bueno
nombre = "Juan"
para i = 0 hasta 10 { imprimir i }

# ✓ También bueno
mi_variable: texto = "Emerson"

# Declaraciones claras
funcion procesar(datos) {
    imprimir datos
}
```

---

**ÁGUILA v0.1.0** | Referencia Rápida
