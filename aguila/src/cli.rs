use std::fs;
use std::io::{self, Write};
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::interpreter::Interprete;
use crate::compiler;
use crate::analyzer::Analizador;
use std::path::Path;
use notify::{Watcher, RecursiveMode, Result as NotifyResult};
use std::sync::mpsc::channel;

pub fn cli_ejecutar(archivo: &str) -> Result<(), String> {
    let contenido = fs::read_to_string(archivo)
        .map_err(|e| format!("Error al leer archivo '{}': {}", archivo, e))?;

    ejecutar_codigo(&contenido)
}

pub fn cli_chequear(archivo: &str) {
    let contenido = match fs::read_to_string(archivo) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error al leer el archivo: {}", e);
            return;
        }
    };

    let mut lexer = Lexer::nuevo(&contenido);
    let tokens = lexer.tokenizar();
    let mut parser = Parser::nuevo(tokens);

    match parser.parsear() {
        Ok(programa) => {
            let mut analizador = Analizador::nuevo();
            let errores = analizador.analizar(&programa);

            if errores.is_empty() {
                println!("✅ Análisis estático completado sin errores.");
            } else {
                println!("❌ Se encontraron {} errores:", errores.len());
                for error in errores {
                    println!("  - {}", error);
                }
            }
        }
        Err(e) => eprintln!("Error de parseo: {}", e),
    }
}

pub fn cli_compilar(archivo: &str) -> Result<(), String> {
    let contenido = fs::read_to_string(archivo)
        .map_err(|e| format!("Error al leer archivo '{}': {}", archivo, e))?;

    let mut lexer = Lexer::nuevo(&contenido);
    let tokens = lexer.tokenizar();

    let mut parser = Parser::nuevo(tokens);
    let programa = parser.parsear()?;

    let js_codigo = compiler::compilar(programa);

    let archivo_salida = archivo.replace(".ag", ".js");
    fs::write(&archivo_salida, js_codigo)
        .map_err(|e| format!("Error al escribir archivo '{}': {}", archivo_salida, e))?;

    println!("Compilado exitosamente a: {}", archivo_salida);
    Ok(())
}

pub fn ejecutar_codigo(codigo: &str) -> Result<(), String> {
    let mut lexer = Lexer::nuevo(codigo);
    let tokens = lexer.tokenizar();

    let mut parser = Parser::nuevo(tokens);
    let programa = parser.parsear()?;

    let mut interprete = Interprete::nuevo();
    let _ = interprete.ejecutar(programa)?;

    Ok(())
}
pub fn cli_dev(archivo: &str) {
    println!("Iniciando modo desarrollo para '{}'...", archivo);
    
    let path_buf = Path::new(archivo);
    let parent_dir = path_buf.parent().unwrap_or(Path::new(".")).to_string_lossy().to_string();
    let parent_dir_server = parent_dir.clone();

    // Iniciar servidor en hilo separado
    std::thread::spawn(move || {
        let mut puerto = 3000;
        let listener = loop {
            match std::net::TcpListener::bind(format!("0.0.0.0:{}", puerto)) {
                Ok(l) => break l,
                Err(_) => {
                    puerto += 1;
                    if puerto > 3010 {
                        eprintln!("Error: No se pudo encontrar un puerto libre entre 3000 y 3010.");
                        return;
                    }
                }
            }
        };
        
        println!("🌐 Servidor de desarrollo corriendo en http://localhost:{}", puerto);
        
        for stream in listener.incoming() {
            if let Ok(mut stream) = stream {
                let mut buffer = [0; 1024];
                use std::io::Read;
                let _ = stream.read(&mut buffer);
                let peticion = String::from_utf8_lossy(&buffer);
                
                // Parseo muy básico
                let linea = peticion.lines().next().unwrap_or("");
                let partes: Vec<&str> = linea.split_whitespace().collect();
                
                if partes.len() >= 2 {
                    let mut ruta = partes[1].to_string();
                    if ruta == "/" {
                        ruta = "/ejemplos/web/index.html".to_string(); // Default hardcoded para el ejemplo
                    }
                    
                    // Ignorar favicon.ico para limpiar consola
                    if ruta == "/favicon.ico" {
                        let response = "HTTP/1.1 204 No Content\r\n\r\n";
                        let _ = stream.write(response.as_bytes());
                        continue;
                    }

                    // Quitar el / inicial para path relativo
                    let ruta_archivo = if ruta.starts_with("/") { &ruta[1..] } else { &ruta };
                    
                    // Seguridad básica: no permitir ..
                    if ruta_archivo.contains("..") {
                         let response = "HTTP/1.1 403 Forbidden\r\n\r\nAcceso denegado";
                         let _ = stream.write(response.as_bytes());
                         continue;
                    }

                    // Intentar leer archivo (primero en raíz, luego en directorio del script)
                    let contenido_opt = fs::read(ruta_archivo).ok().or_else(|| {
                        let path_con_parent = Path::new(&parent_dir_server).join(ruta_archivo);
                        fs::read(path_con_parent).ok()
                    });

                    if let Some(contenido) = contenido_opt {
                        let header = if ruta_archivo.ends_with(".html") {
                            "HTTP/1.1 200 OK\r\nContent-Type: text/html\r\n\r\n"
                        } else if ruta_archivo.ends_with(".js") {
                            "HTTP/1.1 200 OK\r\nContent-Type: application/javascript\r\n\r\n"
                        } else {
                            "HTTP/1.1 200 OK\r\n\r\n"
                        };
                        
                        let _ = stream.write(header.as_bytes());
                        let _ = stream.write(&contenido);
                    } else {
                        let response = "HTTP/1.1 404 Not Found\r\n\r\nArchivo no encontrado";
                        let _ = stream.write(response.as_bytes());
                    }
                }
            }
        }
    });

    // Compilación inicial
    if let Err(e) = cli_compilar(archivo) {
        eprintln!("Error de compilación inicial: {}", e);
    } else {
        println!("Compilación inicial exitosa.");
    }

    // Configurar watcher
    let (tx, rx) = channel();
    let mut watcher = notify::recommended_watcher(tx).unwrap();
    
    if let Err(e) = watcher.watch(Path::new(archivo), RecursiveMode::NonRecursive) {
         eprintln!("Error al iniciar watcher: {:?}", e);
         return;
    }

    println!("👀 Observando cambios... (Ctrl+C para detener)");

    for res in rx {
        match res {
            Ok(_) => {
                // Pequeño debounce para evitar dobles compilaciones rápidas
                std::thread::sleep(std::time::Duration::from_millis(100));
                
                println!("Detectado cambio en '{}', recompilando...", archivo);
                match cli_compilar(archivo) {
                    Ok(_) => println!("✔ Recompilado exitosamente."),
                    Err(e) => eprintln!("✖ Error de compilación: {}", e),
                }
            },
            Err(e) => eprintln!("Error de vigilancia: {:?}", e),
        }
    }
}
