use crate::traits::Printable;

// Enum que representa los diferentes nodos de un lenguaje
pub enum NodoArbol {
    Literal(i32),
    Identificador(String),
    // Usamos Box (Smart Pointer) porque el tamaño del NodoArbol es recursivo
    OperacionBinaria {
        izq: Box<NodoArbol>,
        operador: char,
        der: Box<NodoArbol>,
    },
}

impl Printable for NodoArbol {
    fn print_info(&self) {
        match self {
            NodoArbol::Literal(val) => print!("{}", val),
            NodoArbol::Identificador(id) => print!("{}", id),
            NodoArbol::OperacionBinaria { izq, operador, der } => {
                print!("(");
                izq.print_info();
                print!(" {} ", operador);
                der.print_info();
                print!(")");
            }
        }
    }
}