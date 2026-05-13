use std::fs;
use std::io;

pub fn leer_archivo(ruta: &str) -> io::Result<String> {
    fs::read_to_string(ruta)
}

pub fn escribir_archivo(ruta: &str, contenido: &str) -> io::Result<()> {
    fs::write(ruta, contenido)
}