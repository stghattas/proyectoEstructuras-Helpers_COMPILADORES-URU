use std::time::{SystemTime, UNIX_EPOCH};
use crate::helpers::file_io::añadir_linea;

pub enum LogLevel { Info, Warning, Error }

pub fn log(nivel: LogLevel, mensaje: &str) {
    let prefijo = match nivel {
        LogLevel::Info => "[INFO]",
        LogLevel::Warning => "[WARNING]",
        LogLevel::Error => "[ERROR]",
    };

    // Obtenemos los segundos actuales para generar un Timestamp
    let tiempo_actual = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_secs();

    let mensaje_formateado = format!("(Time: {}) {} {}", tiempo_actual, prefijo, mensaje);

    println!("{}", mensaje_formateado);

    let _ = añadir_linea("registro_logs.txt", &mensaje_formateado);
}