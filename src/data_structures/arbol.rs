use crate::traits::Printable;

// Nodo generico
pub struct NodoArbol<T> {
    pub dato: T,
    pub hijos: Vec<NodoArbol<T>>,
}

impl<T> NodoArbol<T> {
    pub fn new(dato: T) -> Self {
        Self {
            dato,
            hijos: Vec::new(), // nodo nace sin hijos
        }
    }

    pub fn agregar_hijo(&mut self, hijo: NodoArbol<T>) {
        self.hijos.push(hijo);
    }
}

impl<T: std::fmt::Debug> NodoArbol<T> {
    pub fn imprimir_jerarquia(&self, nivel: usize) {
        let sangria = "  ".repeat(nivel);
        println!("{}- {:?}", sangria, self.dato);
        for hijo in &self.hijos {
            hijo.imprimir_jerarquia(nivel + 1);
        }
    }
}

impl<T: std::fmt::Debug> Printable for NodoArbol<T> {
    fn print_info(&self) {
        println!("Jerarquía del Árbol:");
        self.imprimir_jerarquia(0);
    }
}