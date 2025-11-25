use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;

#[derive(Debug, Clone)]
pub enum Value {
    Numero(f64),
    Texto(String),
    Logico(bool),
    Lista(Rc<RefCell<Vec<Value>>>),
    Diccionario(Rc<RefCell<HashMap<String, Value>>>),
    Nulo,
    Funcion(Vec<String>, Vec<crate::ast::Sentencia>, Rc<RefCell<HashMap<String, Value>>>),
    Objeto(String, Rc<RefCell<HashMap<String, Value>>>),
    Clase(String, Rc<RefCell<HashMap<String, Value>>>, Option<Box<Value>>),
}

impl Value {
    pub fn a_texto(&self) -> String {
        match self {
            Value::Numero(n) => {
                if n.fract() == 0.0 {
                    format!("{}", *n as i64)
                } else {
                    format!("{}", n)
                }
            }
            Value::Texto(s) => s.clone(),
            Value::Logico(b) => if *b { "verdadero" } else { "falso" }.to_string(),
            Value::Nulo => "nulo".to_string(),
            Value::Lista(items) => {
                let items = items.borrow();
                let strs: Vec<String> = items.iter().map(|v| v.a_texto()).collect();
                format!("[{}]", strs.join(", "))
            }
            Value::Diccionario(map) => {
                let map = map.borrow();
                let items: Vec<String> = map
                    .iter()
                    .map(|(k, v)| format!("\"{}\": {}", k, v.a_texto()))
                    .collect();
                format!("{{{}}}", items.join(", "))
            }
            Value::Funcion(_, _, _) => "<función>".to_string(),
            Value::Objeto(clase, _) => format!("<instancia de {}>", clase),
            Value::Clase(nombre, _, _) => format!("<clase {}>", nombre),
        }
    }

    pub fn a_booleano(&self) -> bool {
        match self {
            Value::Logico(b) => *b,
            Value::Nulo => false,
            Value::Numero(n) => *n != 0.0,
            Value::Texto(s) => !s.is_empty(),
            Value::Lista(items) => !items.borrow().is_empty(),
            Value::Diccionario(map) => !map.borrow().is_empty(),
            _ => true,
        }
    }
}

impl PartialEq for Value {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Value::Numero(a), Value::Numero(b)) => a == b,
            (Value::Texto(a), Value::Texto(b)) => a == b,
            (Value::Logico(a), Value::Logico(b)) => a == b,
            (Value::Nulo, Value::Nulo) => true,
            _ => false,
        }
    }
}

impl PartialOrd for Value {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Value::Numero(a), Value::Numero(b)) => a.partial_cmp(b),
            (Value::Texto(a), Value::Texto(b)) => a.partial_cmp(b),
            _ => None,
        }
    }
}
