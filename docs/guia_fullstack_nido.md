# 🦅 Guía Maestra: Tu Primera App Fullstack (Sin Errores)

Esta es la guía definitiva para construir una aplicación real con Águila. Está diseñada para que te salga **perfecta a la primera**, incluso si empiezas desde cero.

---

## ✅ Lista de Verificación (Antes de empezar)

Asegúrate de tener esto instalado en tu computadora:
1.  **Águila**: (Obviamente). Verifica escribiendo `aguila` en tu terminal.
2.  **PostgreSQL**: Tu base de datos.
3.  **Node.js**: Para el frontend.

---

## 🏗️ Paso 1: Crear el Backend

Abre tu terminal y ejecuta estos comandos uno por uno:

> **Nota para Windows**: Usa PowerShell.
> **Nota para Mac/Linux**: Usa tu terminal normal.

```bash
# 1. Crear la carpeta del proyecto
aguila init mitienda

# 2. Entrar en la carpeta
cd mitienda

# 3. Instalar el framework web 'Nido'
# (Esto crea el archivo nido.ag en tu carpeta)
aguila instalar nido
```

---

## 💾 Paso 2: La Base de Datos

Necesitamos una base de datos vacía para guardar los productos.

1.  Abre tu herramienta de base de datos favorita (PgAdmin, TablePlus, DBeaver) o usa la terminal.
2.  Ejecuta este comando SQL:
    ```sql
    CREATE DATABASE mitienda_db;
    ```

### Configurar la Conexión
Águila necesita saber dónde está tu base de datos. Configura esta variable de entorno en tu terminal (en la misma donde estás trabajando):

**Opción A: Mac / Linux**
```bash
export DB_URL="postgresql://postgres:postgres@localhost:5432/mitienda_db"
```

**Opción B: Windows (PowerShell)**
```powershell
$env:DB_URL = "postgresql://postgres:postgres@localhost:5432/mitienda_db"
```
*(Ojo: Si tu usuario/contraseña de Postgres no son 'postgres', cámbialos en la URL)*

---

## ⚡ Paso 3: Generar Código Automáticamente

No escribas todo a mano. Deja que Águila genere el código base para los **Productos**.

En tu terminal (dentro de `mitienda/`), ejecuta:

```bash
aguila crear api Producto nombre:Texto:requerido precio:Decimal:requerido stock:Entero activo:Logico
```

Esto ha creado mágicamente:
*   `modelos/Producto.ag` (Lógica)
*   `controladores/ProductoControlador.ag` (API)
*   Una "Migración" (Script para crear la tabla)

### Aplicar la Migración
Ahora dile a Águila que cree la tabla en la base de datos:

```bash
aguila migrar
```
*Si ves un mensaje verde "✅ Migrado", ¡vamos bien!*

---

## 🔒 Paso 4: Seguridad (Autenticación)

Vamos a crear un sistema de Login.
1.  Crea un archivo nuevo llamado `controladores/AuthControlador.ag`.
2.  Pega este código exacto:

```aguila
importar nido

clase AuthControlador {
    fn login(req, res) {
        # En una app real, aquí validarías usuario/password contra la DB
        secreto = "super_secreto_ninja"
        payload = {"id": 1, "rol": "admin", "nombre": "Admin Supremo"}
        
        # Generamos el Token de acceso
        jwt = nido.JWT()
        token = jwt.generar(payload, secreto)
        
        res.json(200, {"token": token})
    }
}
```

---

## 🚀 Paso 5: El Cerebro (Main.ag)

Este es el archivo principal. Vamos a conectarlo todo.
Abre `main.ag` y REEMPLAZA todo su contenido por esto:

```aguila
importar nido
importar db
importar os

# Importamos nuestros módulos generados
desde modelos/Producto importar Producto 
desde controladores/ProductoControlador importar ProductoControlador
desde controladores/AuthControlador importar AuthControlador

# ==========================================
# 1. CONEXIÓN A BASE DE DATOS
# ==========================================
url = os.variable_entorno("DB_URL")
# Respaldo por si olvidaste configurar la variable
si url == nulo { url = "postgresql://postgres:postgres@localhost:5432/mitienda_db" }

conexion_activa = db.conectar(url)

# ==========================================
# 2. INICIAR NIDO
# ==========================================
# IMPORTANTE: Pasamos la conexión y las Clases base de Request/Response
app = nido.App(conexion_activa, nido.Request, nido.Response)
app.activar_docs() # ¡Documentación automática en /docs!

# ==========================================
# 3. SEGURIDAD
# ==========================================
secreto = "super_secreto_ninja"
auth_mw = nido.AuthMiddleware(secreto, nido.JWT)

# ==========================================
# 4. RUTAS
# ==========================================
prod = ProductoControlador()
auth = AuthControlador()

# --- Rutas Públicas ---
# USAMOS CLASES para los handlers para evitar conflictos de nombres
clase InicioControlador {
    fn inicio(req, res) {
        res.json(200, {
            "mensaje": "Bienvenido a mi API",
            "docs": "/docs"
        })
    }
}
ctrl_inicio = InicioControlador()

app.get("/", ctrl_inicio.inicio, nulo)

app.post("/api/login", auth.login, nulo)

# -- Productos (Público: Ver lista) --
app.get("/api/productos", prod.listar, nulo)

# -- Productos (PROTEGIDO: Crear) --
# Solo si tienes Token válido puedes crear
app.post("/api/productos", prod.crear, {
    "body": Producto,
    "middleware": [auth_mw.ejecutar]
})

# ==========================================
# 5. DESPEGAR
# ==========================================
imprimir("🦅 Servidor volando en http://localhost:8080")
imprimir("📜 Documentación API: http://localhost:8081/docs")

app.escuchar(8080, app.rutas)
```

### ¡Pruébalo!
Ejecuta el servidor:
```bash
aguila main.ag
```
Déjalo corriendo y no cierres esa terminal.

---

## ⚛️ Paso 6: El Frontend (React)

Vamos a crear la página web que consumirá tu API.

1.  Abre una **NUEVA TERMINAL** (deja la de Águila corriendo).
2.  Ejecuta estos comandos:

```bash
# Crear proyecto React con Vite
npm create vite@latest frontend -- --template react

# Entrar
cd frontend

# Instalar dependencias
npm install
```

3.  Abre el archivo `frontend/src/App.jsx` y borra todo. Pega esto:

```jsx
import { useEffect, useState } from 'react'
import './App.css'

function App() {
  const [token, setToken] = useState(localStorage.getItem('token'))
  const [productos, setProductos] = useState([])
  const [nombre, setNombre] = useState('')
  const [precio, setPrecio] = useState('')

  const API = "http://localhost:8080/api"

  // -- LOGICA --

  const login = async () => {
      // Simulamos login
      const res = await fetch(`${API}/login`, { method: 'POST' })
      const data = await res.json()
      if (data.token) {
        setToken(data.token)
        localStorage.setItem('token', data.token)
      }
  }

  const cargarProductos = async () => {
    try {
      const res = await fetch(`${API}/productos`)
      const data = await res.json()
      setProductos(data)
    } catch (error) { console.error("Error cargando productos", error) }
  }

  // Cargar al inicio
  useEffect(() => { cargarProductos() }, [])

  const crearProducto = async (e) => {
    e.preventDefault()
    if (!token) return alert("¡Alto ahí! Necesitas loguearte.")

    const nuevo = { 
        nombre, 
        precio: parseFloat(precio), 
        stock: 10, 
        activo: true 
    }
    
    // Petición con Autenticación (Bearer Token)
    const res = await fetch(`${API}/productos`, {
      method: 'POST',
      headers: { 
          'Content-Type': 'application/json',
          'Authorization': `Bearer ${token}`
      },
      body: JSON.stringify(nuevo)
    })
    
    if (res.ok) {
        setNombre('')
        setPrecio('')
        cargarProductos() // Refrescar lista
        alert("¡Producto Creado con Éxito!")
    } else {
        alert("Error: Token inválido o expirado.")
    }
  }

  // -- VISTA --
  
  return (
    <div className="container">
      <h1>🦅 Tienda Águila</h1>
      
      <div style={{marginBottom: '20px', padding: '10px', background: '#f0f0f0', borderRadius: '8px'}}>
          {!token ? (
              <button onClick={login}>🔑 Login Admin</button>
          ) : (
              <span style={{color: 'green', fontWeight: 'bold'}}>✅ Sesión Iniciada</span>
          )}
      </div>

      <form onSubmit={crearProducto} style={{display: 'flex', gap: '10px', marginBottom: '30px'}}>
        <input placeholder="Producto" value={nombre} onChange={e => setNombre(e.target.value)} required />
        <input placeholder="Precio" type="number" value={precio} onChange={e => setPrecio(e.target.value)} required />
        <button type="submit" disabled={!token}>
            {token ? "Guardar" : "🔒 Login Requerido"}
        </button>
      </form>

      <div style={{display: 'grid', gridTemplateColumns: 'repeat(auto-fill, minmax(200px, 1fr))', gap: '20px'}}>
        {productos.map(p => (
          <div key={p.id} style={{border: '1px solid #ddd', padding: '15px', borderRadius: '8px'}}>
            <h3>{p.nombre}</h3>
            <p style={{fontSize: '1.2em', color: '#2c3e50'}}>${p.precio}</p>
          </div>
        ))}
      </div>
    </div>
  )
}

export default App
```

4.  Arranca el frontend:
```bash
npm run dev
```

---

## 🎉 ¡Misión Cumplida!

Abre el link que te muestra la terminal (usualmente `http://localhost:5173`).

1.  Verás la lista vacía.
2.  Intenta crear un producto -> **No te dejará**.
3.  Dale click a **🔑 Login**.
4.  Intenta crear un producto -> **¡Funciona!**

Has creado una aplicación Fullstack con **Águila** (Backend), **Nido** (Framework), **Postgres** (DB) y **React** (Frontend).
