mod ast;
mod lexer;
mod parser;
mod types;
mod interpreter;
mod cli;

use std::env;

const VERSION: &str = "0.1.0";

fn main() {
    let args: Vec<String> = env::args().collect();

    if args.len() < 2 {
        mostrar_ayuda();
        return;
    }

    match args[1].as_str() {
        "ejecutar" => {
            if args.len() < 3 {
                eprintln!("Uso: aguila ejecutar <archivo.ag>");
                return;
            }
            if let Err(e) = cli::cli_ejecutar(&args[2]) {
                eprintln!("Error: {}", e);
                std::process::exit(1);
            }
        }
        "repl" => {
            cli::cli_repl();
        }
        "--version" => {
            println!("aguila v{}", VERSION);
        }
        "--help" | "-h" => {
            mostrar_ayuda();
        }
        _ => {
            eprintln!("Comando desconocido: {}", args[1]);
            mostrar_ayuda();
        }
    }
}

fn mostrar_ayuda() {
    println!("ÁGUILA v{}", VERSION);
    println!();
    println!("Uso: aguila <comando> [opciones]");
    println!();
    println!("Comandos:");
    println!("  ejecutar <archivo.ag>    Ejecuta un archivo ÁGUILA");
    println!("  repl                     Inicia el intérprete interactivo");
    println!("  --version                Muestra la versión");
    println!("  --help, -h               Muestra esta ayuda");
}
