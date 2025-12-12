# Guía Fullstack con Nido y Águila

Esta guía te llevará paso a paso para crear una aplicación completa: desde la base de datos hasta el frontend, utilizando el generador automático de Águila (Scaffolding).

## 1. Prerrequisitos
- **Águila** instalado (v2.7+).
- **PostgreSQL** corriendo y una base de datos creada (ej: `mi_app_db`).

## 2. Inicializar Proyecto
Crea una carpeta y un proyecto vacío:
```bash
aguila init mi_tienda
cd mi_tienda
```
Instala el framework Nido:
```bash
aguila instalar nido
```

## 3. Generar un Recurso (Scaffold)
Usaremos el comando `crear api` para generar Modelo, Controlador y Migración automáticamente.

Sintaxis:
`aguila crear api <Modelo> <campo>:<tipo>:<opciones> ...`

Ejemplo (Crear Producto):
```bash
aguila crear api Producto nombre:Texto:requerido:unico precio:Decimal:requerido stock:Entero en_oferta:Logico
```

Esto generará:
- `modelos/Producto.ag`: Modelo ORM con reglas de validación.
- `controladores/ProductoControlador.ag`: API REST (listar, crear, actualizar, borrar).
- `migraciones/202..._crear_producto.ag`: Script SQL para crear la tabla.

## 4. Configurar Base de Datos
Águila busca la variable de entorno `DB_URL`. 
En Linux/Mac:
```bash
export DB_URL="postgresql://usuario:password@localhost:5432/mi_app_db"
```
En Windows (Powershell):
```powershell
$env:DB_URL="postgresql://usuario:password@localhost:5432/mi_app_db"
```

## 5. Ejecutar Migraciones
Corre el comando para aplicar los cambios a la DB:
```bash
aguila migrar
```
Verás mensajes como `✅ Migrado: ..._crear_producto`.

## 6. Integrar en Main.ag
Ahora debes conectar todo en tu `main.ag`. Abre el archivo y configúralo:

```aguila
importar nido
importar db

# Importar Controladores Generados
importar controladores/ProductoControlador

# Conexión DB
conn_str = sistema.variable_entorno("DB_URL")
si conn_str == nulo { conn_str = "postgresql://postgres:postgres@localhost:5432/aguila" }

imprimir("Conectando a DB...")
conexion = db.conectar(conn_str)
nido.configurar_db_global(conexion) # Importante para que los Modelos funcionen

# Iniciar App
app = nido.App(conexion)

# Registrar Rutas del Controlador
prod = ProductoControlador()

app.get("/api/productos", prod.listar, nulo)
app.post("/api/productos", prod.crear, {"body": Producto}) # 'Producto' es la clase del modelo
app.put("/api/productos", prod.actualizar, {"body": Producto})
app.delete("/api/productos", prod.borrar_ultimo, nulo)

# Habilitar Docs
app.activar_docs()

app.escuchar(8080, app.rutas)
```
*> Nota: Asegúrate de importar el modelo 'Producto' si lo usas en la config de validación.*

## 7. Probar y Frontend
Ejecuta tu servidor:
```bash
aguila main.ag
```
Visita `http://localhost:8080/docs` para ver tu API documentada.

### Ejemplo Frontend (HTML + JS)
Crea un archivo `index.html`:

```html
<!DOCTYPE html>
<html>
<body>
    <h1>Productos</h1>
    <ul id="lista"></ul>
    
    <script>
        async function cargar() {
            const res = await fetch('http://localhost:8080/api/productos');
            const prods = await res.json();
            
            const ul = document.getElementById('lista');
            prods.forEach(p => {
                const li = document.createElement('li');
                li.innerText = `${p.nombre} - $${p.precio}`;
                ul.appendChild(li);
            });
        }
        cargar();
    </script>
</body>
</html>
```
Gracias a los headers CORS automáticos de Nido, este frontend funcionará sin problemas.
