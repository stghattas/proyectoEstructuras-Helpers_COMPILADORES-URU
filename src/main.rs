#![allow(dead_code)]

mod traits;
mod data_structures;
mod helpers;

use traits::Printable;
use data_structures::ast::NodoArbol;
use data_structures::cola::Cola;
use data_structures::pila::Pila;
use data_structures::mapas::TablaSimbolos;
use helpers::logger::{log, LogLevel};

fn main() {
    log(LogLevel::Info, "=== INICIANDO ===");

    // 1. Probando el ÁRBOL (Usa Enums y Smart Pointers 'Box')
    log(LogLevel::Info, "1. Probando Árbol (AST)...");
    let ast = NodoArbol::OperacionBinaria {
        izq: Box::new(NodoArbol::Identificador("variable_x".to_string())),
        operador: '=',
        der: Box::new(NodoArbol::Literal(42)),
    };
    print!("   Resultado Printable: ");
    ast.print_info(); 
    println!("\n");

    // 2. Probando la COLA
    log(LogLevel::Info, "2. Probando Cola...");
    let mut cola_tokens = Cola::new();
    cola_tokens.enqueue("Token(LET)");
    cola_tokens.enqueue("Token(IDENTIFICADOR)");
    cola_tokens.enqueue("Token(IGUAL)");
    print!("   Resultado Printable: ");
    cola_tokens.print_info();
    println!();

    // 3. Probando la PILA
    log(LogLevel::Info, "3. Probando Pila...");
    let mut pila_operandos = Pila::new();
    pila_operandos.push(10);
    pila_operandos.push(20);
    pila_operandos.push(30);
    print!("   Resultado Printable: ");
    pila_operandos.print_info();
    println!();

    // 4. Probando MAPAS
    log(LogLevel::Info, "4. Probando Mapas (Tabla de Símbolos)...");
    let mut tabla = TablaSimbolos::new();
    tabla.insertar("variable_x".to_string(), "entero".to_string());
    tabla.insertar("mensaje".to_string(), "string".to_string());
    print!("   Resultado Printable: \n");
    tabla.print_info();
    println!();

    log(LogLevel::Info, "=== TERMINANDO PRUEBA ===");
}