use std::fs::OpenOptions;
use std::io::{self, BufRead, BufReader, Write};
use std::fs::File;

// Lee el archivo línea por línea y devuelve un vector con las líneas
pub fn leer_lineas(ruta: &str) -> io::Result<Vec<String>> {
    let archivo = File::open(ruta)?;
    let lector = BufReader::new(archivo);
    let mut lineas = Vec::new();

    for linea in lector.lines() {
        lineas.push(linea?);
    }
    Ok(lineas)
}

// Escribe (añade) una sola linea al final del archivo sin borrar lo anterior
pub fn añadir_linea(ruta: &str, contenido: &str) -> io::Result<()> {
    let mut archivo = OpenOptions::new()
        .create(true) // Crea el archivo si no existe
        .append(true) // Añade al final
        .open(ruta)?;

    writeln!(archivo, "{}", contenido)
}