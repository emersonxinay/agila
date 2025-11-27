use std::io::{self, Write};
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::interpreter::Interprete;

pub fn iniciar() {
    println!("ÁGUILA v{}", env!("CARGO_PKG_VERSION"));
    println!("Escribe 'salir' para terminar, o 'ayuda' para ver comandos.");

    let mut interprete = Interprete::nuevo();

    loop {
        print!("> ");
        if let Err(e) = io::stdout().flush() {
            eprintln!("Error al flushear stdout: {}", e);
            break;
        }

        let mut linea = String::new();
        match io::stdin().read_line(&mut linea) {
            Ok(_) => {
                let linea = linea.trim();
                if linea.is_empty() {
                    continue;
                }

                match linea {
                    "salir" => {
                        println!("¡Hasta luego!");
                        break;
                    }
                    "ayuda" => {
                        println!("Comandos disponibles:");
                        println!("  salir    - Termina la sesión");
                        println!("  ayuda    - Muestra este mensaje");
                        continue;
                    }
                    _ => {}
                }

                ejecutar_linea(&mut interprete, linea);
            }
            Err(e) => {
                eprintln!("Error al leer entrada: {}", e);
                break;
            }
        }
    }
}

fn ejecutar_linea(interprete: &mut Interprete, codigo: &str) {
    let mut lexer = Lexer::nuevo(codigo);
    let tokens = lexer.tokenizar();

    let mut parser = Parser::nuevo(tokens);
    match parser.parsear() {
        Ok(programa) => {
            match interprete.ejecutar(programa) {
                Ok(val_opt) => {
                    if let Some(val) = val_opt {
                        println!("=> {}", val.a_texto());
                    }
                }
                Err(e) => eprintln!("Error de ejecución: {}", e),
            }
        }
        Err(e) => eprintln!("Error de sintaxis: {}", e),
    }
}
