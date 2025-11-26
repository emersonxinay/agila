use crate::ast::{Token, Sentencia, Expresion, Programa};

pub struct Parser {
    tokens: Vec<Token>,
    posicion: usize,
}

impl Parser {
    pub fn nuevo(tokens: Vec<Token>) -> Self {
        Parser {
            tokens,
            posicion: 0,
        }
    }

    fn token_actual(&self) -> &Token {
        self.tokens.get(self.posicion).unwrap_or(&Token::EOF)
    }

    fn token_siguiente(&self) -> &Token {
        self.tokens.get(self.posicion + 1).unwrap_or(&Token::EOF)
    }

    fn avanzar(&mut self) -> Token {
        let token = self.token_actual().clone();
        if self.posicion < self.tokens.len() {
            self.posicion += 1;
        }
        token
    }

    fn esperar(&mut self, esperado: Token) -> Result<(), String> {
        if self.token_actual() == &esperado {
            self.avanzar();
            Ok(())
        } else {
            Err(format!(
                "Error de sintaxis: se esperaba {:?}, se encontró {:?}",
                esperado, self.token_actual()
            ))
        }
    }

    pub fn parsear(&mut self) -> Result<Programa, String> {
        let mut sentencias = Vec::new();
        while self.token_actual() != &Token::EOF {
            sentencias.push(self.parsear_sentencia()?);
        }
        Ok(Programa { sentencias })
    }

    fn parsear_sentencia(&mut self) -> Result<Sentencia, String> {
        match self.token_actual() {
            Token::Imprimir => self.parsear_imprimir(),
            Token::Si => self.parsear_si(),
            Token::Mientras => self.parsear_mientras(),
            Token::Para => self.parsear_para(),
            Token::Retornar => self.parsear_retornar(),
            Token::Funcion => self.parsear_funcion(false),
            Token::Asincrono => {
                self.avanzar(); // Consumir 'asincrono'
                if self.token_actual() == &Token::Funcion {
                    self.parsear_funcion(true)
                } else {
                    Err("Se esperaba 'funcion' después de 'asincrono'".to_string())
                }
            }
            Token::Clase => self.parsear_clase(),
            Token::Clase => self.parsear_clase(),
            Token::Importar => self.parsear_importar(),
            Token::Intentar => self.parsear_intentar(),
            Token::Identificador(nombre) => {
                let nombre = nombre.clone();
                self.avanzar();

                match self.token_actual() {
                    Token::Asignacion => {
                        self.avanzar();
                        let valor = self.parsear_expresion()?;
                        Ok(Sentencia::Asignacion {
                            nombre,
                            tipo: None,
                            valor,
                        })
                    }
                    Token::DosPuntos => {
                        self.avanzar();
                        let tipo = if let Token::Identificador(t) = self.avanzar() {
                            t
                        } else {
                            return Err("Se esperaba tipo después de ':'".to_string());
                        };
                        self.esperar(Token::Asignacion)?;
                        let valor = self.parsear_expresion()?;
                        Ok(Sentencia::Asignacion {
                            nombre,
                            tipo: Some(tipo),
                            valor,
                        })
                    }
                    Token::Punto => {
                        self.posicion -= 1;
                        let expr = self.parsear_expresion()?;
                        Ok(Sentencia::Expresion(expr))
                    }
                    _ => {
                        self.posicion -= 1;
                        let expr = self.parsear_expresion()?;
                        Ok(Sentencia::Expresion(expr))
                    }
                }
            }
            _ => {
                let expr = self.parsear_expresion()?;
                Ok(Sentencia::Expresion(expr))
            }
        }
    }

    fn parsear_imprimir(&mut self) -> Result<Sentencia, String> {
        self.esperar(Token::Imprimir)?;
        let expr = self.parsear_expresion()?;
        Ok(Sentencia::Imprimir(expr))
    }

    fn parsear_importar(&mut self) -> Result<Sentencia, String> {
        self.esperar(Token::Importar)?;
        let ruta = if let Token::Texto(s) = self.avanzar() {
            s
        } else {
            return Err("Se esperaba ruta de archivo en 'importar'".to_string());
        };

        let alias = if let Token::Identificador(s) = self.token_actual() {
            if s == "como" || s == "as" {
                self.avanzar();
                if let Token::Identificador(nombre) = self.avanzar() {
                    Some(nombre)
                } else {
                    return Err("Se esperaba nombre después de 'como'".to_string());
                }
            } else {
                None
            }
        } else {
            None
        };

        Ok(Sentencia::Importar { ruta, alias })
    }

    fn parsear_intentar(&mut self) -> Result<Sentencia, String> {
        self.esperar(Token::Intentar)?;
        self.esperar(Token::LlaveAbre)?;
        let bloque_intentar = self.parsear_bloque()?;
        self.esperar(Token::LlaveCierra)?;

        self.esperar(Token::Capturar)?;
        let variable_error = if let Token::Identificador(nombre) = self.avanzar() {
            nombre
        } else {
            return Err("Se esperaba nombre de variable de error después de 'capturar'".to_string());
        };

        self.esperar(Token::LlaveAbre)?;
        let bloque_capturar = self.parsear_bloque()?;
        self.esperar(Token::LlaveCierra)?;

        Ok(Sentencia::Intentar {
            bloque_intentar,
            variable_error,
            bloque_capturar,
        })
    }

    fn parsear_retornar(&mut self) -> Result<Sentencia, String> {
        self.esperar(Token::Retornar)?;
        if self.token_actual() == &Token::LlaveCierra || self.token_actual() == &Token::EOF {
            Ok(Sentencia::Retorno(None))
        } else {
            let expr = self.parsear_expresion()?;
            Ok(Sentencia::Retorno(Some(expr)))
        }
    }

    fn parsear_si(&mut self) -> Result<Sentencia, String> {
        self.esperar(Token::Si)?;
        let condicion = self.parsear_expresion()?;
        self.esperar(Token::LlaveAbre)?;
        let si_bloque = self.parsear_bloque()?;
        self.esperar(Token::LlaveCierra)?;

        let sino_bloque = if self.token_actual() == &Token::Sino {
            self.avanzar();
            self.esperar(Token::LlaveAbre)?;
            let bloque = self.parsear_bloque()?;
            self.esperar(Token::LlaveCierra)?;
            Some(bloque)
        } else {
            None
        };

        Ok(Sentencia::Si {
            condicion,
            si_bloque,
            sino_bloque,
        })
    }

    fn parsear_mientras(&mut self) -> Result<Sentencia, String> {
        self.esperar(Token::Mientras)?;
        let condicion = self.parsear_expresion()?;
        self.esperar(Token::LlaveAbre)?;
        let bloque = self.parsear_bloque()?;
        self.esperar(Token::LlaveCierra)?;
        Ok(Sentencia::Mientras { condicion, bloque })
    }

    fn parsear_para(&mut self) -> Result<Sentencia, String> {
        self.esperar(Token::Para)?;

        let variable = if let Token::Identificador(nombre) = self.avanzar() {
            nombre
        } else {
            return Err("Se esperaba nombre de variable en 'para'".to_string());
        };

        if self.token_actual() == &Token::Asignacion {
            // para i = 0 hasta 5
            self.avanzar();
            let inicio = if let Token::Numero(n) = self.avanzar() {
                n as i64
            } else {
                return Err("Se esperaba número".to_string());
            };

            self.esperar(Token::Hasta)?;

            let fin = if let Token::Numero(n) = self.avanzar() {
                n as i64
            } else {
                return Err("Se esperaba número".to_string());
            };

            self.esperar(Token::LlaveAbre)?;
            let bloque = self.parsear_bloque()?;
            self.esperar(Token::LlaveCierra)?;

            Ok(Sentencia::ParaRango {
                variable,
                inicio,
                fin,
                bloque,
            })
        } else if self.token_actual() == &Token::En {
            // para n en lista
            self.avanzar();
            let iterador = self.parsear_expresion()?;
            self.esperar(Token::LlaveAbre)?;
            let bloque = self.parsear_bloque()?;
            self.esperar(Token::LlaveCierra)?;

            Ok(Sentencia::Para {
                variable,
                iterador,
                bloque,
            })
        } else {
            Err("Se esperaba '=' o 'en' en sentencia 'para'".to_string())
        }
    }

    fn parsear_funcion(&mut self, es_asincrona: bool) -> Result<Sentencia, String> {
        self.esperar(Token::Funcion)?;

        let nombre = if let Token::Identificador(n) = self.avanzar() {
            n
        } else {
            return Err("Se esperaba nombre de función".to_string());
        };

        self.esperar(Token::ParAbre)?;
        let mut parametros = Vec::new();

        while self.token_actual() != &Token::ParCierra {
            if let Token::Identificador(param) = self.avanzar() {
                let tipo = if self.token_actual() == &Token::DosPuntos {
                    self.avanzar();
                    if let Token::Identificador(t) = self.avanzar() {
                        Some(t)
                    } else {
                        None
                    }
                } else {
                    None
                };
                parametros.push((param, tipo));

                if self.token_actual() == &Token::Coma {
                    self.avanzar();
                }
            }
        }
        self.esperar(Token::ParCierra)?;

        let retorno_tipo = if self.token_actual() == &Token::Flecha {
            self.avanzar();
            if let Token::Identificador(t) = self.avanzar() {
                Some(t)
            } else {
                return Err("Se esperaba tipo de retorno después de '->'".to_string());
            }
        } else {
            None
        };

        self.esperar(Token::LlaveAbre)?;
        let bloque = self.parsear_bloque()?;
        self.esperar(Token::LlaveCierra)?;

        Ok(Sentencia::Funcion {
            nombre,
            parametros,
            retorno_tipo,
            bloque,
            es_asincrona,
        })
    }

    fn parsear_clase(&mut self) -> Result<Sentencia, String> {
        self.esperar(Token::Clase)?;

        let nombre = if let Token::Identificador(n) = self.avanzar() {
            n
        } else {
            return Err("Se esperaba nombre de clase".to_string());
        };

        let padre = if self.token_actual() == &Token::DosPuntos {
            self.avanzar();
            if let Token::Identificador(p) = self.avanzar() {
                Some(p)
            } else {
                None
            }
        } else {
            None
        };

        self.esperar(Token::LlaveAbre)?;

        let mut atributos = Vec::new();
        let mut metodos = Vec::new();

        while self.token_actual() != &Token::LlaveCierra {
            if self.token_actual() == &Token::Funcion {
                // ... (código existente para 'funcion nombre(...)')
                self.avanzar();
                let metodo_nombre = if let Token::Identificador(n) = self.avanzar() {
                    n
                } else {
                    return Err("Se esperaba nombre de método".to_string());
                };
                // ... (parsear parámetros y bloque)
                self.esperar(Token::ParAbre)?;
                let mut params = Vec::new();
                while self.token_actual() != &Token::ParCierra {
                    if let Token::Identificador(param) = self.avanzar() {
                        let tipo = if self.token_actual() == &Token::DosPuntos {
                            self.avanzar();
                            if let Token::Identificador(t) = self.avanzar() { Some(t) } else { None }
                        } else { None };
                        params.push((param, tipo));
                        if self.token_actual() == &Token::Coma { self.avanzar(); }
                    }
                }
                self.esperar(Token::ParCierra)?;
                self.esperar(Token::LlaveAbre)?;
                let bloque = self.parsear_bloque()?;
                self.esperar(Token::LlaveCierra)?;
                metodos.push((metodo_nombre, params, bloque));

            } else if let Token::Identificador(nombre) = self.token_actual().clone() {
                // Verificar si es método o atributo mirando el siguiente token
                if self.token_siguiente() == &Token::ParAbre {
                    // Es un método: nombre(...) { ... }
                    self.avanzar(); // Consumir nombre
                    self.esperar(Token::ParAbre)?;
                    let mut params = Vec::new();
                    while self.token_actual() != &Token::ParCierra {
                        if let Token::Identificador(param) = self.avanzar() {
                            let tipo = if self.token_actual() == &Token::DosPuntos {
                                self.avanzar();
                                if let Token::Identificador(t) = self.avanzar() { Some(t) } else { None }
                            } else { None };
                            params.push((param, tipo));
                            if self.token_actual() == &Token::Coma { self.avanzar(); }
                        }
                    }
                    self.esperar(Token::ParCierra)?;
                    self.esperar(Token::LlaveAbre)?;
                    let bloque = self.parsear_bloque()?;
                    self.esperar(Token::LlaveCierra)?;
                    metodos.push((nombre, params, bloque));
                } else {
                    // Es un atributo: nombre: Tipo
                    self.avanzar(); // Consumir nombre
                    let tipo = if self.token_actual() == &Token::DosPuntos {
                        self.avanzar();
                        if let Token::Identificador(t) = self.avanzar() { Some(t) } else { None }
                    } else { None };
                    atributos.push((nombre, tipo));
                }
            } else {
                // Token inesperado dentro de clase, avanzar para evitar bucle infinito
                self.avanzar();
            }
        }
        self.esperar(Token::LlaveCierra)?;

        Ok(Sentencia::Clase {
            nombre,
            padre,
            atributos,
            metodos,
        })
    }

    fn parsear_bloque(&mut self) -> Result<Vec<Sentencia>, String> {
        let mut sentencias = Vec::new();
        while self.token_actual() != &Token::LlaveCierra && self.token_actual() != &Token::EOF {
            sentencias.push(self.parsear_sentencia()?);
        }
        Ok(sentencias)
    }

    fn parsear_expresion(&mut self) -> Result<Expresion, String> {
        self.parsear_comparacion()
    }

    fn parsear_comparacion(&mut self) -> Result<Expresion, String> {
        let mut izq = self.parsear_suma()?;

        while matches!(
            self.token_actual(),
            Token::Igual | Token::NoIgual | Token::Mayor | Token::Menor | Token::MayorIgual | Token::MenorIgual
        ) {
            let op = match self.avanzar() {
                Token::Igual => "==",
                Token::NoIgual => "!=",
                Token::Mayor => ">",
                Token::Menor => "<",
                Token::MayorIgual => ">=",
                Token::MenorIgual => "<=",
                _ => unreachable!(),
            };
            let der = self.parsear_suma()?;
            izq = Expresion::BinOp {
                izq: Box::new(izq),
                op: op.to_string(),
                der: Box::new(der),
            };
        }

        Ok(izq)
    }

    fn parsear_suma(&mut self) -> Result<Expresion, String> {
        let mut izq = self.parsear_multiplicacion()?;

        while matches!(self.token_actual(), Token::Mas | Token::Menos) {
            let op = match self.avanzar() {
                Token::Mas => "+",
                Token::Menos => "-",
                _ => unreachable!(),
            };
            let der = self.parsear_multiplicacion()?;
            izq = Expresion::BinOp {
                izq: Box::new(izq),
                op: op.to_string(),
                der: Box::new(der),
            };
        }

        Ok(izq)
    }

    fn parsear_multiplicacion(&mut self) -> Result<Expresion, String> {
        let mut izq = self.parsear_unaria()?;

        while matches!(self.token_actual(), Token::Por | Token::Div) {
            let op = match self.avanzar() {
                Token::Por => "*",
                Token::Div => "/",
                _ => unreachable!(),
            };
            let der = self.parsear_unaria()?;
            izq = Expresion::BinOp {
                izq: Box::new(izq),
                op: op.to_string(),
                der: Box::new(der),
            };
        }

        Ok(izq)
    }

    fn parsear_unaria(&mut self) -> Result<Expresion, String> {
        if self.token_actual() == &Token::Esperar {
            self.avanzar();
            let expr = self.parsear_unaria()?;
            Ok(Expresion::Esperar(Box::new(expr)))
        } else {
            self.parsear_primaria()
        }
    }

    fn parsear_primaria(&mut self) -> Result<Expresion, String> {
        let expr = match self.token_actual().clone() {
            Token::Numero(n) => {
                self.avanzar();
                Expresion::Numero(n)
            }
            Token::Texto(s) => {
                self.avanzar();
                Expresion::Texto(s)
            }
            Token::TextoInterpolado(s) => {
                self.avanzar();
                let mut partes = Vec::new();
                let mut buffer = String::new();
                let mut chars = s.chars().peekable();
                
                while let Some(c) = chars.next() {
                    if c == '{' {
                        if !buffer.is_empty() {
                            partes.push(Expresion::Texto(buffer.clone()));
                            buffer.clear();
                        }
                        
                        // Extraer contenido entre {}
                        let mut expr_str = String::new();
                        let mut balance = 1;
                        while let Some(inner_c) = chars.next() {
                            if inner_c == '{' {
                                balance += 1;
                                expr_str.push(inner_c);
                            } else if inner_c == '}' {
                                balance -= 1;
                                if balance == 0 {
                                    break;
                                }
                                expr_str.push(inner_c);
                            } else {
                                expr_str.push(inner_c);
                            }
                        }
                        
                        // Parsear expresión interna
                        let mut lexer = crate::lexer::Lexer::nuevo(&expr_str);
                        let tokens = lexer.tokenizar();
                        let mut parser = Parser::nuevo(tokens);
                        match parser.parsear_expresion() {
                            Ok(expr) => partes.push(expr),
                            Err(e) => return Err(format!("Error en interpolación: {}", e)),
                        }
                    } else {
                        buffer.push(c);
                    }
                }
                
                if !buffer.is_empty() {
                    partes.push(Expresion::Texto(buffer));
                }
                
                Expresion::Interpolacion(partes)
            }
            Token::Verdadero => {
                self.avanzar();
                Expresion::Logico(true)
            }
            Token::Falso => {
                self.avanzar();
                Expresion::Logico(false)
            }
            Token::Nulo => {
                self.avanzar();
                Expresion::Nulo
            }
            Token::ParAbre => {
                self.avanzar();
                let expr = self.parsear_expresion()?;
                self.esperar(Token::ParCierra)?;
                expr
            }
            Token::CorcheteAbre => {
                self.avanzar();
                let mut elementos = Vec::new();
                while self.token_actual() != &Token::CorcheteCierra {
                    elementos.push(self.parsear_expresion()?);
                    if self.token_actual() == &Token::Coma {
                        self.avanzar();
                    }
                }
                self.esperar(Token::CorcheteCierra)?;
                Expresion::Lista(elementos)
            }
            Token::LlaveAbre => {
                self.avanzar();
                let mut pares = Vec::new();
                while self.token_actual() != &Token::LlaveCierra {
                    let clave = if let Token::Texto(s) = self.avanzar() {
                        s
                    } else if let Token::Identificador(s) = self.token_actual().clone() {
                        self.avanzar();
                        s
                    } else {
                        return Err("Se esperaba clave en diccionario".to_string());
                    };
                    self.esperar(Token::DosPuntos)?;
                    let valor = self.parsear_expresion()?;
                    pares.push((clave, valor));
                    if self.token_actual() == &Token::Coma {
                        self.avanzar();
                    }
                }
                self.esperar(Token::LlaveCierra)?;
                Expresion::Diccionario(pares)
            }
            Token::Identificador(nombre) => {
                self.avanzar();
                if self.token_actual() == &Token::ParAbre {
                    self.avanzar();
                    let mut args = Vec::new();
                    while self.token_actual() != &Token::ParCierra {
                        args.push(self.parsear_expresion()?);
                        if self.token_actual() == &Token::Coma {
                            self.avanzar();
                        }
                    }
                    self.esperar(Token::ParCierra)?;
                    Expresion::Llamada { nombre, args }
                } else {
                    Expresion::Identificador(nombre)
                }
            }
            Token::Nuevo => {
                self.avanzar();
                if let Token::Identificador(clase) = self.avanzar() {
                    self.esperar(Token::ParAbre)?;
                    let mut args = Vec::new();
                    while self.token_actual() != &Token::ParCierra {
                        args.push(self.parsear_expresion()?);
                        if self.token_actual() == &Token::Coma {
                            self.avanzar();
                        }
                    }
                    self.esperar(Token::ParCierra)?;
                    Expresion::Instancia { clase, args }
                } else {
                    return Err("Se esperaba nombre de clase después de 'nuevo'".to_string());
                }
            }
            Token::Funcion => {
                self.avanzar();
                self.esperar(Token::ParAbre)?;
                let mut parametros = Vec::new();
                while self.token_actual() != &Token::ParCierra {
                    if let Token::Identificador(param) = self.avanzar() {
                        parametros.push(param);
                    } else {
                        return Err("Se esperaba nombre de parámetro".to_string());
                    }
                    if self.token_actual() == &Token::Coma {
                        self.avanzar();
                    }
                }
                self.esperar(Token::ParCierra)?;
                self.esperar(Token::LlaveAbre)?;
                let bloque = self.parsear_bloque()?;
                self.esperar(Token::LlaveCierra)?;
                Expresion::FuncionAnonima { parametros, bloque, es_asincrona: false }
            }
            Token::Asincrono => {
                self.avanzar(); // Consumir 'asincrono'
                self.esperar(Token::Funcion)?;
                self.esperar(Token::ParAbre)?;
                let mut parametros = Vec::new();
                while self.token_actual() != &Token::ParCierra {
                    if let Token::Identificador(param) = self.avanzar() {
                        parametros.push(param);
                    } else {
                        return Err("Se esperaba nombre de parámetro".to_string());
                    }
                    if self.token_actual() == &Token::Coma {
                        self.avanzar();
                    }
                }
                self.esperar(Token::ParCierra)?;
                self.esperar(Token::LlaveAbre)?;
                let bloque = self.parsear_bloque()?;
                self.esperar(Token::LlaveCierra)?;
                Expresion::FuncionAnonima { parametros, bloque, es_asincrona: true }
            }
            _ => return Err(format!("Token inesperado en expresión: {:?}", self.token_actual())),
        };

        self.parsear_postfija(expr)
    }

    fn parsear_postfija(&mut self, mut expr: Expresion) -> Result<Expresion, String> {
        loop {
            match self.token_actual() {
                Token::Punto => {
                    self.avanzar();
                    if let Token::Identificador(nombre) = self.avanzar() {
                        if self.token_actual() == &Token::ParAbre {
                            self.avanzar();
                            let mut args = Vec::new();
                            while self.token_actual() != &Token::ParCierra {
                                args.push(self.parsear_expresion()?);
                                if self.token_actual() == &Token::Coma {
                                    self.avanzar();
                                }
                            }
                            self.esperar(Token::ParCierra)?;
                            expr = Expresion::MetodoLlamada {
                                objeto: Box::new(expr),
                                metodo: nombre,
                                args,
                            };
                        } else if self.token_actual() == &Token::Asignacion {
                            self.avanzar();
                            let valor = self.parsear_expresion()?;
                            expr = Expresion::AsignacionAtributo {
                                objeto: Box::new(expr),
                                atributo: nombre,
                                valor: Box::new(valor),
                            };
                            break;
                        } else {
                            expr = Expresion::AccesoAtributo {
                                objeto: Box::new(expr),
                                atributo: nombre,
                            };
                        }
                    }
                }
                Token::CorcheteAbre => {
                    self.avanzar();
                    let indice = self.parsear_expresion()?;
                    self.esperar(Token::CorcheteCierra)?;
                    expr = Expresion::AccesoIndice {
                        objeto: Box::new(expr),
                        indice: Box::new(indice),
                    };
                }
                _ => break,
            }
        }
        Ok(expr)
    }
}
