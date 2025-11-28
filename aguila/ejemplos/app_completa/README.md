# Gestor de Tareas (Ejemplo End-to-End)

Esta aplicación demuestra cómo construir un programa completo en Águila.

## Características
- **Persistencia de Datos:** Guarda y carga tareas en `tareas.json` usando los módulos nativos `fs` y `json`.
- **Programación Orientada a Objetos:** Usa clases `Tarea` y `Gestor` para organizar la lógica.
- **Interacción:** Menú interactivo por consola.

## Ejecución

Desde la raíz del proyecto:

```bash
# Ejecutar con el binario local
./aguila/target/release/aguila aguila/ejemplos/app_completa/gestor_tareas.ag

# O si tienes aguila instalado globalmente
aguila aguila/ejemplos/app_completa/gestor_tareas.ag
```

## Estructura del Código
- `clase Tarea`: Modelo de datos.
- `clase Gestor`: Lógica de negocio (CRUD).
- `bucle principal`: Interfaz de usuario (CLI).
