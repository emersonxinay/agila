use std::fs;
use std::io::{self, Write};
use crate::lexer::Lexer;
use crate::parser::Parser;
use crate::interpreter::Interprete;

pub fn cli_ejecutar(archivo: &str) -> Result<(), String> {
    let contenido = fs::read_to_string(archivo)
        .map_err(|e| format!("Error al leer archivo '{}': {}", archivo, e))?;

    ejecutar_codigo(&contenido)
}

pub fn cli_repl() {
    println!("ÁGUILA v0.1.0");
    println!("Escribe 'salir' para terminar");

    let mut interprete = Interprete::nuevo();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut linea = String::new();
        match io::stdin().read_line(&mut linea) {
            Ok(_) => {
                let linea = linea.trim();
                if linea == "salir" {
                    println!("¡Hasta luego!");
                    break;
                }

                if linea.is_empty() {
                    continue;
                }

                match ejecutar_en_repl(&mut interprete, linea) {
                    Ok(_) => {}
                    Err(e) => println!("Error: {}", e),
                }
            }
            Err(e) => {
                println!("Error: {}", e);
                break;
            }
        }
    }
}

pub fn ejecutar_codigo(codigo: &str) -> Result<(), String> {
    let mut lexer = Lexer::nuevo(codigo);
    let tokens = lexer.tokenizar();

    let mut parser = Parser::nuevo(tokens);
    let programa = parser.parsear()?;

    let mut interprete = Interprete::nuevo();
    interprete.ejecutar(programa)?;

    Ok(())
}

fn ejecutar_en_repl(interprete: &mut Interprete, linea: &str) -> Result<(), String> {
    let mut lexer = Lexer::nuevo(linea);
    let tokens = lexer.tokenizar();

    let mut parser = Parser::nuevo(tokens);
    let programa = parser.parsear()?;

    interprete.ejecutar(programa)?;

    Ok(())
}
