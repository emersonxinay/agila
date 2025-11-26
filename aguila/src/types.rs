use std::collections::HashMap;
use std::rc::Rc;
use std::cell::RefCell;
use std::fmt;
use postgres;

#[derive(Clone)]
pub struct NativeFn(pub Rc<dyn Fn(&[Value]) -> Result<Value, String>>);

impl fmt::Debug for NativeFn {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<función nativa>")
    }
}

impl PartialEq for NativeFn {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

#[derive(Clone)]
pub struct DbClient(pub Rc<RefCell<postgres::Client>>);

impl fmt::Debug for DbClient {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "<conexión db>")
    }
}

impl PartialEq for DbClient {
    fn eq(&self, _other: &Self) -> bool {
        false
    }
}

#[derive(Debug, Clone)]
pub enum Value {
    Numero(f64),
    Texto(String),
    Logico(bool),
    Lista(Rc<RefCell<Vec<Value>>>),
    Diccionario(Rc<RefCell<HashMap<String, Value>>>),
    Nulo,
    Funcion(Vec<String>, Vec<crate::ast::Sentencia>, Rc<RefCell<HashMap<String, Value>>>, bool),
    FuncionNativa(NativeFn),
    Clase(String, Rc<RefCell<HashMap<String, Value>>>), // Nombre, Scope de clase (métodos)
    Instancia {
        clase: String,
        atributos: Rc<RefCell<HashMap<String, Value>>>,
    },
    BaseDeDatos(DbClient),
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
            Value::Funcion(..) => "<función>".to_string(),
            Value::FuncionNativa(_) => "<función nativa>".to_string(),
            Value::Clase(nombre, _) => format!("<clase {}>", nombre),
            Value::Instancia { clase, .. } => format!("<instancia de {}>", clase),
            Value::BaseDeDatos(_) => "<conexión db>".to_string(),
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
            (Value::Instancia { clase: c1, .. }, Value::Instancia { clase: c2, .. }) => c1 == c2,
            (Value::BaseDeDatos(_), Value::BaseDeDatos(_)) => false, // No comparamos conexiones
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
