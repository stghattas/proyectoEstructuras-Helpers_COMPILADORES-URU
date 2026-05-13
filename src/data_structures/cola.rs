use std::collections::VecDeque;
use crate::traits::Printable;

pub struct Cola<T> {
    elementos: VecDeque<T>,
}

impl<T> Cola<T> {
    pub fn new() -> Self { Self { elementos: VecDeque::new() } }
    pub fn enqueue(&mut self, item: T) { self.elementos.push_back(item); }
    pub fn dequeue(&mut self) -> Option<T> { self.elementos.pop_front() }
}

// Implementacion del trait Printable para la Cola
impl<T: std::fmt::Debug> Printable for Cola<T> {
    fn print_info(&self) {
        println!("Cola actual: {:?}", self.elementos);
    }
}