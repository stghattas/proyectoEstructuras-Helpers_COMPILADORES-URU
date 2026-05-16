use crate::helpers::file_io::añadir_linea;
use chrono::Local;

pub enum LogLevel { Info, Warning, Error }

pub fn log(nivel: LogLevel, mensaje: &str) {
    let prefijo = match nivel {
        LogLevel::Info => "[INFO]",
        LogLevel::Warning => "[WARNING]",
        LogLevel::Error => "[ERROR]",
    };

    let fecha_hora_legible = Local::now().format("%Y-%m-%d %H:%M:%S").to_string();

    let mensaje_formateado = format!("[{}] {} {}", fecha_hora_legible, prefijo, mensaje);

    println!("{}", mensaje_formateado);

    let _ = añadir_linea("src/logs/registro_logs.txt", &mensaje_formateado);
}