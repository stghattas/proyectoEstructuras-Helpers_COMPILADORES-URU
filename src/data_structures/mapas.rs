use std::collections::HashMap;
use crate::traits::Printable;

pub struct TablaSimbolos {
    tabla: HashMap<String, String>,
}

impl TablaSimbolos {
    pub fn new() -> Self {
        Self {
            tabla: HashMap::new(),
        }
    }

    pub fn insertar(&mut self, id: String, tipo: String) {
        self.tabla.insert(id, tipo);
    }

    pub fn buscar(&self, id: &str) -> Option<&String> {
        self.tabla.get(id)
    }
}

impl Printable for TablaSimbolos {
    fn print_info(&self) {
        println!("--- Tabla de Símbolos ---");
        for (clave, valor) in &self.tabla {
            println!("  Variable: {}, Tipo: {}", clave, valor);
        }
        println!("-------------------------");
    }
}