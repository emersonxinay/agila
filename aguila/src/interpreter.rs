use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use crate::ast::{Sentencia, Expresion, Programa};
use crate::types::Value;

pub struct Interprete {
    variables: Vec<Rc<RefCell<HashMap<String, Value>>>>,
    funciones: HashMap<String, (Vec<(String, Option<String>)>, Vec<Sentencia>)>,
    clases: HashMap<String, (Option<String>, Vec<(String, Option<String>)>, Vec<(String, Vec<(String, Option<String>)>, Vec<Sentencia>)>)>,
}

impl Interprete {
    pub fn nuevo() -> Self {
        let mut interprete = Interprete {
            variables: vec![Rc::new(RefCell::new(HashMap::new()))],
            funciones: HashMap::new(),
            clases: HashMap::new(),
        };
        interprete.registrar_funciones_nativas();
        interprete
    }

    fn registrar_funciones_nativas(&mut self) {}

    pub fn ejecutar(&mut self, programa: Programa) -> Result<(), String> {
        for sentencia in programa.sentencias {
            self.ejecutar_sentencia(&sentencia)?;
        }
        Ok(())
    }

    fn ejecutar_sentencia(&mut self, sentencia: &Sentencia) -> Result<(), String> {
        match sentencia {
            Sentencia::Asignacion {
                nombre,
                tipo: _,
                valor,
            } => {
                let val = self.evaluar_expresion(valor)?;
                if let Some(scope) = self.variables.last_mut() {
                    scope.borrow_mut().insert(nombre.clone(), val);
                }
                Ok(())
            }
            Sentencia::Expresion(expr) => {
                self.evaluar_expresion(expr)?;
                Ok(())
            }
            Sentencia::Imprimir(expr) => {
                let val = self.evaluar_expresion(expr)?;
                println!("{}", val.a_texto());
                Ok(())
            }
            Sentencia::Si {
                condicion,
                si_bloque,
                sino_bloque,
            } => {
                let cond = self.evaluar_expresion(condicion)?;
                if cond.a_booleano() {
                    for sent in si_bloque {
                        self.ejecutar_sentencia(sent)?;
                    }
                } else if let Some(bloque) = sino_bloque {
                    for sent in bloque {
                        self.ejecutar_sentencia(sent)?;
                    }
                }
                Ok(())
            }
            Sentencia::Mientras {
                condicion,
                bloque,
            } => {
                loop {
                    let cond = self.evaluar_expresion(condicion)?;
                    if !cond.a_booleano() {
                        break;
                    }
                    for sent in bloque {
                        self.ejecutar_sentencia(sent)?;
                    }
                }
                Ok(())
            }
            Sentencia::Para {
                variable,
                iterador,
                bloque,
            } => {
                let iter_val = self.evaluar_expresion(iterador)?;
                match iter_val {
                    Value::Lista(items) => {
                        for item in items.borrow().iter() {
                            if let Some(scope) = self.variables.last_mut() {
                                scope.borrow_mut().insert(variable.clone(), item.clone());
                            }
                            for sent in bloque {
                                self.ejecutar_sentencia(sent)?;
                            }
                        }
                    }
                    Value::Diccionario(map) => {
                        for (key, _) in map.borrow().iter() {
                            if let Some(scope) = self.variables.last_mut() {
                                scope.borrow_mut().insert(variable.clone(), Value::Texto(key.clone()));
                            }
                            for sent in bloque {
                                self.ejecutar_sentencia(sent)?;
                            }
                        }
                    }
                    _ => return Err("No se puede iterar sobre este tipo".to_string()),
                }
                Ok(())
            }
            Sentencia::ParaRango {
                variable,
                inicio,
                fin,
                bloque,
            } => {
                for i in *inicio..*fin {
                    if let Some(scope) = self.variables.last_mut() {
                        scope.borrow_mut().insert(variable.clone(), Value::Numero(i as f64));
                    }
                    for sent in bloque {
                        self.ejecutar_sentencia(sent)?;
                    }
                }
                Ok(())
            }
            Sentencia::Funcion {
                nombre,
                parametros,
                bloque,
            } => {
                self.funciones.insert(
                    nombre.clone(),
                    (parametros.clone(), bloque.clone()),
                );
                Ok(())
            }
            Sentencia::Clase {
                nombre,
                padre,
                atributos,
                metodos,
            } => {
                self.clases.insert(
                    nombre.clone(),
                    (padre.clone(), atributos.clone(), metodos.clone()),
                );
                Ok(())
            }
            Sentencia::Retorno(_) => Ok(()),
        }
    }

    fn evaluar_expresion(&mut self, expresion: &Expresion) -> Result<Value, String> {
        match expresion {
            Expresion::Numero(n) => Ok(Value::Numero(*n)),
            Expresion::Texto(s) => Ok(Value::Texto(s.clone())),
            Expresion::Logico(b) => Ok(Value::Logico(*b)),
            Expresion::Nulo => Ok(Value::Nulo),
            Expresion::Identificador(nombre) => self.obtener_variable(nombre),
            Expresion::Lista(elementos) => {
                let mut lista = Vec::new();
                for elem in elementos {
                    lista.push(self.evaluar_expresion(elem)?);
                }
                Ok(Value::Lista(Rc::new(RefCell::new(lista))))
            }
            Expresion::Diccionario(pares) => {
                let mut dict = HashMap::new();
                for (clave, expr) in pares {
                    let valor = self.evaluar_expresion(expr)?;
                    dict.insert(clave.clone(), valor);
                }
                Ok(Value::Diccionario(Rc::new(RefCell::new(dict))))
            }
            Expresion::BinOp { izq, op, der } => {
                self.evaluar_binop(izq, op, der)
            }
            Expresion::Llamada { nombre, args } => {
                self.ejecutar_llamada(nombre, args)
            }
            Expresion::MetodoLlamada {
                objeto,
                metodo,
                args,
            } => {
                let obj = self.evaluar_expresion(objeto)?;
                self.ejecutar_metodo(&obj, metodo, args)
            }
            Expresion::AccesoAtributo { objeto, atributo } => {
                let obj = self.evaluar_expresion(objeto)?;
                self.acceder_atributo(&obj, atributo)
            }
            Expresion::AsignacionAtributo {
                objeto,
                atributo,
                valor,
            } => {
                let obj = self.evaluar_expresion(objeto)?;
                let val = self.evaluar_expresion(valor)?;
                self.asignar_atributo(&obj, atributo, val)?;
                Ok(Value::Nulo)
            }
            Expresion::Instancia { clase, args } => {
                self.crear_instancia(clase, args)
            }
        }
    }

    fn evaluar_binop(&mut self, izq: &Expresion, op: &str, der: &Expresion) -> Result<Value, String> {
        let izq_val = self.evaluar_expresion(izq)?;
        let der_val = self.evaluar_expresion(der)?;

        match op {
            "+" => match (&izq_val, &der_val) {
                (Value::Numero(a), Value::Numero(b)) => Ok(Value::Numero(a + b)),
                (Value::Texto(a), Value::Texto(b)) => Ok(Value::Texto(format!("{}{}", a, b))),
                (Value::Texto(a), b) => Ok(Value::Texto(format!("{}{}", a, b.a_texto()))),
                (a, Value::Texto(b)) => Ok(Value::Texto(format!("{}{}", a.a_texto(), b))),
                _ => Err("Operación + no válida".to_string()),
            },
            "-" => match (&izq_val, &der_val) {
                (Value::Numero(a), Value::Numero(b)) => Ok(Value::Numero(a - b)),
                _ => Err("Operación - no válida".to_string()),
            },
            "*" => match (&izq_val, &der_val) {
                (Value::Numero(a), Value::Numero(b)) => Ok(Value::Numero(a * b)),
                _ => Err("Operación * no válida".to_string()),
            },
            "/" => match (&izq_val, &der_val) {
                (Value::Numero(a), Value::Numero(b)) => {
                    if *b == 0.0 {
                        Err("Error: División por cero".to_string())
                    } else {
                        Ok(Value::Numero(a / b))
                    }
                }
                _ => Err("Operación / no válida".to_string()),
            },
            "==" => Ok(Value::Logico(izq_val == der_val)),
            "!=" => Ok(Value::Logico(izq_val != der_val)),
            ">" => match (&izq_val, &der_val) {
                (Value::Numero(a), Value::Numero(b)) => Ok(Value::Logico(a > b)),
                (Value::Texto(a), Value::Texto(b)) => Ok(Value::Logico(a > b)),
                _ => Err("Operación > no válida".to_string()),
            },
            "<" => match (&izq_val, &der_val) {
                (Value::Numero(a), Value::Numero(b)) => Ok(Value::Logico(a < b)),
                (Value::Texto(a), Value::Texto(b)) => Ok(Value::Logico(a < b)),
                _ => Err("Operación < no válida".to_string()),
            },
            ">=" => match (&izq_val, &der_val) {
                (Value::Numero(a), Value::Numero(b)) => Ok(Value::Logico(a >= b)),
                (Value::Texto(a), Value::Texto(b)) => Ok(Value::Logico(a >= b)),
                _ => Err("Operación >= no válida".to_string()),
            },
            "<=" => match (&izq_val, &der_val) {
                (Value::Numero(a), Value::Numero(b)) => Ok(Value::Logico(a <= b)),
                (Value::Texto(a), Value::Texto(b)) => Ok(Value::Logico(a <= b)),
                _ => Err("Operación <= no válida".to_string()),
            },
            _ => Err(format!("Operador {} no reconocido", op)),
        }
    }

    fn obtener_variable(&self, nombre: &str) -> Result<Value, String> {
        for scope in self.variables.iter().rev() {
            if let Some(val) = scope.borrow().get(nombre) {
                return Ok(val.clone());
            }
        }
        Err(format!("Error: Variable '{}' no definida", nombre))
    }

    fn ejecutar_llamada(&mut self, nombre: &str, args: &[Expresion]) -> Result<Value, String> {
        // Primero, intenta como clase (instancia)
        if self.clases.contains_key(nombre) {
            return self.crear_instancia(nombre, args);
        }

        // Luego, intenta como función
        if let Some((parametros, bloque)) = self.funciones.get(nombre).cloned() {
            let mut nuevo_scope = HashMap::new();

            for (i, (param_name, _)) in parametros.iter().enumerate() {
                let arg_val = if i < args.len() {
                    self.evaluar_expresion(&args[i])?
                } else {
                    Value::Nulo
                };
                nuevo_scope.insert(param_name.clone(), arg_val);
            }

            self.variables.push(Rc::new(RefCell::new(nuevo_scope)));

            for sent in &bloque {
                self.ejecutar_sentencia(sent)?;
            }

            self.variables.pop();
            Ok(Value::Nulo)
        } else {
            Err(format!("Error: Función '{}' no definida", nombre))
        }
    }

    fn crear_instancia(&mut self, clase_nombre: &str, args: &[Expresion]) -> Result<Value, String> {
        if let Some((_padre, atributos, metodos_all)) = self.clases.get(clase_nombre).cloned() {
            let mut objeto = HashMap::new();

            // Inicializar atributos
            for (attr_name, _) in &atributos {
                objeto.insert(attr_name.clone(), Value::Nulo);
            }

            let mut instancia = Value::Objeto(clase_nombre.to_string(), Rc::new(RefCell::new(objeto)));

            // Buscar y ejecutar constructor (__init__)
            for (metodo_nombre, parametros, bloque) in metodos_all {
                if metodo_nombre == "__init__" {
                    let mut nuevo_scope = HashMap::new();
                    nuevo_scope.insert("self".to_string(), instancia.clone());

                    for (i, (param_name, _)) in parametros.iter().enumerate() {
                        if param_name != "self" {
                            let arg_val = if i - 1 < args.len() {
                                self.evaluar_expresion(&args[i - 1])?
                            } else {
                                Value::Nulo
                            };
                            nuevo_scope.insert(param_name.clone(), arg_val);
                        }
                    }

                    self.variables.push(Rc::new(RefCell::new(nuevo_scope)));

                    for sent in &bloque {
                        self.ejecutar_sentencia(sent)?;
                    }

                    if let Some(scope) = self.variables.pop() {
                        if let Some(self_val) = scope.borrow().get("self") {
                            instancia = self_val.clone();
                        }
                    }
                }
            }

            Ok(instancia)
        } else {
            Err(format!("Error: Clase '{}' no definida", clase_nombre))
        }
    }

    fn ejecutar_metodo(&mut self, objeto: &Value, metodo: &str, args: &[Expresion]) -> Result<Value, String> {
        if let Value::Objeto(clase_nombre, _) = objeto {
            if let Some((_, _, metodos)) = self.clases.get(clase_nombre).cloned() {
                for (metodo_nombre, parametros, bloque) in metodos {
                    if metodo_nombre == metodo {
                        let mut nuevo_scope = HashMap::new();
                        nuevo_scope.insert("self".to_string(), objeto.clone());

                        for (i, (param_name, _)) in parametros.iter().enumerate() {
                            if param_name != "self" {
                                let arg_val = if i - 1 < args.len() {
                                    self.evaluar_expresion(&args[i - 1])?
                                } else {
                                    Value::Nulo
                                };
                                nuevo_scope.insert(param_name.clone(), arg_val);
                            }
                        }

                        self.variables.push(Rc::new(RefCell::new(nuevo_scope)));

                        for sent in &bloque {
                            self.ejecutar_sentencia(sent)?;
                        }

                        self.variables.pop();
                        return Ok(Value::Nulo);
                    }
                }
            }
            Err(format!("Error: Método '{}' no encontrado", metodo))
        } else {
            Err("Error: No es un objeto".to_string())
        }
    }

    fn acceder_atributo(&self, objeto: &Value, atributo: &str) -> Result<Value, String> {
        if let Value::Objeto(_, attrs) = objeto {
            if let Some(val) = attrs.borrow().get(atributo) {
                Ok(val.clone())
            } else {
                Err(format!("Error: Atributo '{}' no existe", atributo))
            }
        } else {
            Err("Error: No es un objeto".to_string())
        }
    }

    fn asignar_atributo(&self, objeto: &Value, atributo: &str, valor: Value) -> Result<(), String> {
        if let Value::Objeto(_, attrs) = objeto {
            attrs.borrow_mut().insert(atributo.to_string(), valor);
            Ok(())
        } else {
            Err("Error: No es un objeto".to_string())
        }
    }
}
