use rustyline::error::ReadlineError;
use rustyline::DefaultEditor;
use crate::interpreter::Interprete;
use crate::lexer::Lexer;
use crate::parser::Parser;

pub fn iniciar() {
    println!("🦅 ÁGUILA v0.1.0");
    println!("Escribe 'salir' para terminar.");

    let mut rl = DefaultEditor::new().unwrap();
    if rl.load_history("historial.txt").is_err() {
        // No hay historial previo
    }

    let mut interprete = Interprete::nuevo();

    loop {
        let readline = rl.readline(">> ");
        match readline {
            Ok(line) => {
                let input = line.trim();
                if input == "salir" {
                    break;
                }
                if input.is_empty() {
                    continue;
                }

                let _ = rl.add_history_entry(input);

                let mut lexer = Lexer::nuevo(input);
                let tokens = lexer.tokenizar();
                
                let mut parser = Parser::nuevo(tokens);
                match parser.parsear() {
                    Ok(programa) => {
                        match interprete.ejecutar(programa) {
                            Ok(Some(valor)) => {
                                if valor != crate::types::Value::Nulo {
                                    println!("{}", valor.a_texto());
                                }
                            },
                            Ok(None) => {},
                            Err(e) => eprintln!("Error de ejecución: {}", e),
                        }
                    },
                    Err(e) => eprintln!("Error de sintaxis: {}", e),
                }
            },
            Err(ReadlineError::Interrupted) => {
                println!("CTRL-C");
                break;
            },
            Err(ReadlineError::Eof) => {
                println!("CTRL-D");
                break;
            },
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    let _ = rl.save_history("historial.txt");
}
