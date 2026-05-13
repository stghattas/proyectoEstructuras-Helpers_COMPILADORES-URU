pub enum LogLevel {
    Info,
    Warning,
    Error,
}

pub fn log(nivel: LogLevel, mensaje: &str) {
    let prefijo = match nivel {
        LogLevel::Info => "[INFO]",
        LogLevel::Warning => "[WARNING]",
        LogLevel::Error => "[ERROR]",
    };
    println!("{} {}", prefijo, mensaje);
}