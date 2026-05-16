use crate::traits::Printable;

pub struct Mapa<K, V> {
    entradas: Vec<(K, V)>,
}

impl<K: PartialEq, V: Clone> Mapa<K, V> {
    pub fn new() -> Self {
        Self { entradas: Vec::new() }
    }

    pub fn insertar(&mut self, clave: K, valor: V) {
        // Si la clave ya existe, actualiza su valor
        for entrada in &mut self.entradas {
            if entrada.0 == clave {
                entrada.1 = valor;
                return;
            }
        }
        // Si no existe, la agrega
        self.entradas.push((clave, valor));
    }

    pub fn buscar(&self, clave: &K) -> Option<&V> {
        for entrada in &self.entradas {
            if &entrada.0 == clave {
                return Some(&entrada.1);
            }
        }
        None
    }
}

impl<K: std::fmt::Debug, V: std::fmt::Debug> Printable for Mapa<K, V> {
    fn print_info(&self) {
        println!("Mapa:");
        for (clave, valor) in &self.entradas {
            println!("  [{:?}] -> {:?}", clave, valor);
        }
    }
}