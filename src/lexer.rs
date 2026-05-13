use crate::data_structures::cola::Cola;

// Agregamos Debug y Clone para poder imprimirlos y copiarlos fácilmente
#[derive(Debug, Clone)]
pub enum Token {
    Let,                  // Palabra reservada 'let'
    Identificador(String),// Nombres de variables, ej: 'x'
    Igual,                // Operador '='
    Numero(i32),          // Números, ej: 10, 5
    Suma,                 // Operador '+'
    PuntoYComa,           // Símbolo ';'
    Desconocido(char),    // Para atrapar errores
}

pub fn tokenizar(codigo_fuente: &str) -> Cola<Token> {
    let mut cola_tokens = Cola::new();
    // Convertimos el texto en un iterador de caracteres que podemos "espiar" (peek)
    let mut caracteres = codigo_fuente.chars().peekable();

    while let Some(&c) = caracteres.peek() {
        match c {
            // Ignoramos los espacios en blanco y saltos de línea
            ' ' | '\n' | '\t' | '\r' => {
                caracteres.next();
            }
            // Símbolos simples
            '=' => { cola_tokens.enqueue(Token::Igual); caracteres.next(); }
            '+' => { cola_tokens.enqueue(Token::Suma); caracteres.next(); }
            ';' => { cola_tokens.enqueue(Token::PuntoYComa); caracteres.next(); }
            
            // Palabras y variables (Letras)
            'a'..='z' | 'A'..='Z' => {
                let mut palabra = String::new();
                while let Some(&ch) = caracteres.peek() {
                    if ch.is_alphanumeric() {
                        palabra.push(ch);
                        caracteres.next();
                    } else {
                        break;
                    }
                }
                // Verificamos si es una palabra reservada o un identificador normal
                if palabra == "let" {
                    cola_tokens.enqueue(Token::Let);
                } else {
                    cola_tokens.enqueue(Token::Identificador(palabra));
                }
            }
            
            // Números
            '0'..='9' => {
                let mut numero_str = String::new();
                while let Some(&ch) = caracteres.peek() {
                    if ch.is_numeric() {
                        numero_str.push(ch);
                        caracteres.next();
                    } else {
                        break;
                    }
                }
                let valor: i32 = numero_str.parse().unwrap();
                cola_tokens.enqueue(Token::Numero(valor));
            }
            
            // Cualquier otra cosa es un error/desconocido
            _ => {
                cola_tokens.enqueue(Token::Desconocido(c));
                caracteres.next();
            }
        }
    }

    cola_tokens
}