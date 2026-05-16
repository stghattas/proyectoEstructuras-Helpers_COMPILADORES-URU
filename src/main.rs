#![allow(dead_code)]

mod traits;
mod data_structures;
mod helpers;

use traits::Printable;
use data_structures::arbol::NodoArbol;
use data_structures::cola::Cola;
use data_structures::pila::Pila;
use data_structures::mapas::Mapa;
use helpers::logger::{log, LogLevel};

fn main() {
    log(LogLevel::Info, "--- INICIANDO PRUEBAS ---");

    // 1. ÁRBOL N-ARIO (Jerarquia de Empresa)
    log(LogLevel::Info, "Probando Árbol Jerárquico Genérico...");
    let mut ceo = NodoArbol::new("CEO - Director General");
    
    let mut gerente_it = NodoArbol::new("Gerente de IT");
    gerente_it.agregar_hijo(NodoArbol::new("Programador Frontend"));
    gerente_it.agregar_hijo(NodoArbol::new("Programador Backend"));

    let gerente_rh = NodoArbol::new("Gerente de Recursos Humanos");

    ceo.agregar_hijo(gerente_it);
    ceo.agregar_hijo(gerente_rh);
    
    ceo.print_info();
    println!();

    // 2. PILA (Ver Tope)
    log(LogLevel::Info, "Probando Pila...");
    let mut pila = Pila::new();
    pila.push(10);
    pila.push(20);
    if let Some(tope) = pila.ver_tope() {
        println!("  -> El elemento en el TOPE es: {}", tope);
    }
    println!();

    // 3. COLA (Ver Frente y Final)
    log(LogLevel::Info, "Probando Cola...");
    let mut cola = Cola::new();
    cola.enqueue("Cliente A");
    cola.enqueue("Cliente B");
    cola.enqueue("Cliente C");
    if let Some(frente) = cola.ver_frente() {
        println!("  -> El primero en la fila (FRENTE) es: {}", frente);
    }
    if let Some(final_cola) = cola.ver_final() {
        println!("  -> El último en la fila (FINAL) es: {}", final_cola);
    }
    println!();

    // 4. MAPA
    log(LogLevel::Info, "Probando Mapa...");
    let mut mapa = Mapa::new();
    mapa.insertar("Usuario1", "Contraseña123");
    mapa.insertar("Usuario2", "Admin456");
    mapa.print_info();

    log(LogLevel::Info, "--- FIN DE LAS PRUEBAS ---");
    println!("Revisa el archivo 'registro_logs.txt' en tu carpeta para ver los logs guardados");
}