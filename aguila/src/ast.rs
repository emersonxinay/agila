#[derive(Debug, Clone, PartialEq)]
pub enum Token {
    // Literales
    Numero(f64),
    Texto(String),
    Identificador(String),

    // Palabras clave
    Funcion,
    Si,
    Sino,
    Mientras,
    Para,
    En,
    Hasta,
    Clase,
    Imprimir,
    Verdadero,
    Falso,
    Nulo,

    // Operadores
    Mas,
    Menos,
    Por,
    Div,
    Mayor,
    Menor,
    MayorIgual,
    MenorIgual,
    Igual,
    NoIgual,
    Asignacion,
    Punto,
    DosPuntos,
    Coma,
    Dos,

    // Delimitadores
    ParAbre,
    ParCierra,
    LlaveAbre,
    LlaveCierra,
    CorcheteAbre,
    CorcheteCierra,

    // Control
    EOF,
}

#[derive(Debug, Clone)]
pub enum Sentencia {
    Asignacion {
        nombre: String,
        tipo: Option<String>,
        valor: Expresion,
    },
    Expresion(Expresion),
    Si {
        condicion: Expresion,
        si_bloque: Vec<Sentencia>,
        sino_bloque: Option<Vec<Sentencia>>,
    },
    Mientras {
        condicion: Expresion,
        bloque: Vec<Sentencia>,
    },
    Para {
        variable: String,
        iterador: Expresion,
        bloque: Vec<Sentencia>,
    },
    ParaRango {
        variable: String,
        inicio: i64,
        fin: i64,
        bloque: Vec<Sentencia>,
    },
    Funcion {
        nombre: String,
        parametros: Vec<(String, Option<String>)>,
        bloque: Vec<Sentencia>,
    },
    Clase {
        nombre: String,
        padre: Option<String>,
        atributos: Vec<(String, Option<String>)>,
        metodos: Vec<(String, Vec<(String, Option<String>)>, Vec<Sentencia>)>,
    },
    Retorno(Option<Expresion>),
    Imprimir(Expresion),
}

#[derive(Debug, Clone)]
pub enum Expresion {
    Numero(f64),
    Texto(String),
    Logico(bool),
    Nulo,
    Identificador(String),
    Lista(Vec<Expresion>),
    Diccionario(Vec<(String, Expresion)>),
    BinOp {
        izq: Box<Expresion>,
        op: String,
        der: Box<Expresion>,
    },
    Llamada {
        nombre: String,
        args: Vec<Expresion>,
    },
    MetodoLlamada {
        objeto: Box<Expresion>,
        metodo: String,
        args: Vec<Expresion>,
    },
    AccesoAtributo {
        objeto: Box<Expresion>,
        atributo: String,
    },
    AsignacionAtributo {
        objeto: Box<Expresion>,
        atributo: String,
        valor: Box<Expresion>,
    },
    Instancia {
        clase: String,
        args: Vec<Expresion>,
    },
}

#[derive(Debug, Clone)]
pub struct Programa {
    pub sentencias: Vec<Sentencia>,
}
