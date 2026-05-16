use crate::traits::Printable;

pub struct Pila<T> {
    elementos: Vec<T>,
}

impl<T> Pila<T> {
    pub fn new() -> Self { Self { elementos: Vec::new() } }
    pub fn push(&mut self, item: T) { self.elementos.push(item); }
    pub fn pop(&mut self) -> Option<T> { self.elementos.pop() }

    pub fn ver_tope(&self) -> Option<&T> {
        self.elementos.last()
    }
}

impl<T: std::fmt::Debug> Printable for Pila<T> {
    fn print_info(&self) { println!("Pila: {:?}", self.elementos); }
}