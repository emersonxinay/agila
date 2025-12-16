# Ciencia de Datos en Águila

Águila incluye una potente suite de herramientas nativas para análisis de datos, estadística y visualización científica. Este stack está diseñado para ser tan fácil como Python pero con el rendimiento de Rust.

## Módulos Principales

*   `azar`: Generación de números aleatorios y estadística.
*   `datos`: Manipulación de datasets (DataFrames) de alto rendimiento.
*   `graficas`: Visualización de datos de calidad de publicación (PDF, SVG, PNG).

---

## 1. Módulo `azar`
Control de aleatoriedad y distribuciones estadísticas. Vital para simulaciones (Monte Carlo) y muestreo.

### Funciones Principales
| Función | Descripción | Ejemplo |
| :--- | :--- | :--- |
| `elegir(lista)` | Elemento al azar de una lista. | `azar.elegir(["A", "B"])` |
| `entero(min, max)` | Entero aleatorio entre min y max. | `azar.entero(1, 100)` |
| `uniforme(a, b)` | Float aleatorio en rango [a, b). | `azar.uniforme(0.0, 1.0)` |
| `normal(mu, sigma)` | Distribución Normal (Gaussiana). | `azar.normal(0.0, 1.0)` |
| `semilla(n)` | Fija la semilla para reproducibilidad. | `azar.semilla(42)` |
| `ponderado(items, pesos)` | Selección basada en probabilidad. | `azar.ponderado(["Raro", "Común"], [0.1, 0.9])` |

---

## 2. Módulo `datos`
El motor de DataFrames de Águila. Permite cargar, limpiar y transformar millones de filas en segundos gracias a su backend optimizado (comparable a Pandas/Polars).

### Carga y Vista Previa
```aguila
importar datos

df = datos.leer_csv("ventas_2024.csv")
imprimir(datos.dimensiones(df)) # [filas, columnas]
datos.ver(df, 5) # Mostrar las primeras 5 filas
```

### Limpieza de Datos
```aguila
# Eliminar filas con valores nulos (NaN)
datos.eliminar_nulos(df)

# O rellenar nulos con un valor por defecto
datos.llenar_nulos(df, 0)
```

### Consultas y Transformación
```aguila
# 1. Seleccionar columnas de interés
df_sub = datos.seleccionar(df, ["Producto", "Precio", "Cantidad"])

# 2. Filtrar filas (SQL WHERE)
# Operadores: ">", "<", "==", "!="
df_caros = datos.filtrar(df, "Precio", ">", 100.0)

# 3. Ordenar
datos.ordenar(df, "Precio", falso) # Descendente
```

### Agregación (Group By)
```aguila
# Sumar "Cantidad" agrupar por "Categoria"
resumen = datos.agrupar(df, "Categoria", "Cantidad")
```

---

## 3. Módulo `graficas`
Visualización profesional. Soporta salidas en pantalla (interactivo), imágenes (PNG/SVG) y reportes PDF.

### Configuración
```aguila
importar graficas

graficas.calidad(2.0) # 1.0 = Normal, 2.0 = Retina (Alta Resolución)
graficas.figura(800, 600, "Título del Gráfico")
```

### Tipos de Gráficos
| Función | Uso |
| :--- | :--- |
| `linea(x, y, nombre, color)` | Series de tiempo, tendencias. |
| `dispersion(x, y, nombre, color)` | Correlación entre variables. |
| `barras(lbls, vals, titulo, color)` | Comparación entre categorías. |
| `histograma(datos, bins, color)` | Distribución de probabilidad. |

### Exportación y Reportes
```aguila
# Guardar imagen para web
graficas.guardar("analisis.png")

# Guardar vector para impresión profesional
graficas.guardar("analisis.svg")

# Generar Informe PDF completo
texto_informe = "El Q4 mostró un crecimiento del 15%.\nSe recomienda invertir en Marketing."
graficas.reporte("Informe Ejecutivo 2024", texto_informe, "reporte_anual.pdf")
```

---

## Casos de Uso Reales (Ejemplos Complejos)

### Caso 1: Simulación de Monte Carlo (Finanzas)
Proyectar el valor futuro de una acción usando volatilidad aleatoria (`azar`).

```aguila
importar azar
importar graficas

# Parámetros
precio_inicial = 100.0
dias = 365
volatilidad = 0.02 # 2% diario

azar.semilla(1234) # Reproducible

precios = []
dias_lista = []
precio_actual = precio_inicial

# Simulación
i = 0
mientras i < dias {
    cambio = azar.normal(0.0, volatilidad)
    precio_actual = precio_actual * (1.0 + cambio)
    
    precios.agregar(precio_actual)
    dias_lista.agregar(i)
    i = i + 1
}

# Visualización
graficas.calidad(2.0)
graficas.figura(1000, 500, "Proyección de Activos (Monte Carlo)")
graficas.linea(dias_lista, precios, "Escenario Base", "verde")

graficas.guardar("monte_carlo.png")
imprimir("Simulación guardada en monte_carlo.png")
```

### Caso 2: Pipeline de Análisis de Ventas (ETL + Reporting)
Carga datos, limpia, agrega por región y genera un PDF para la gerencia.

```aguila
importar datos
importar graficas

# 1. Ingesta (Extract)
raw_df = datos.leer_csv("transacciones_globales.csv")

# 2. Limpieza (Clean)
datos.eliminar_nulos(raw_df)

# 3. Análisis (Transform)
# Queremos ver ventas totales por Región
# Asumimos que el CSV tiene columnas "Region" y "Venta"
ventas_region = datos.agrupar(raw_df, "Region", "Venta")
datos.ordenar(ventas_region, "Venta", falso) # Top regions primero

# Extraer datos de las columnas para graficar
regiones = datos.columna(ventas_region, "Region")
valores = datos.columna(ventas_region, "Venta")

# 4. Visualización y Reporte (Load)
graficas.calidad(2.0)
graficas.figura(800, 600, "Ventas Globales por Región")
graficas.barras(regiones, valores, "Ingresos (USD)", "azul")

cuerpo_reporte = "Norteamérica domina el mercado con un 45%.\nEuropa muestra estancamiento."
graficas.reporte(
    "Reporte de Desempeño Comercial",
    cuerpo_reporte,
    "dashboard_ventas.pdf"
)

imprimir("Reporte generado exitosamente.")
```
