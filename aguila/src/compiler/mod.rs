pub mod codegen;

use crate::ast::Programa;
use crate::compiler::codegen::GeneradorJS;

pub fn compilar(programa: Programa) -> String {
    let mut generador = GeneradorJS::nuevo();
    generador.generar(programa)
}
