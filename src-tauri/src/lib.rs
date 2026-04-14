use rusqlite::Connection;
use serde::{Serialize, Deserialize};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command]
async fn saludar(nombre: String) ->String{
    format!("hola como estas, {}!", nombre)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    init_db();
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet, saludar, crear_producto, obtener_productos, actualizar_producto, eliminar_producto])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");

    
}


#[derive(Serialize, Deserialize)]
struct Producto {
    id: i32,
    nombre: String,
    precio: f64,
    stock: i32,
}

fn init_db() {
    let conn = Connection::open("farmacia.db").unwrap();

    conn.execute(
        "CREATE TABLE IF NOT EXISTS productos (
            id INTEGER PRIMARY KEY AUTOINCREMENT,
            nombre TEXT NOT NULL,
            precio REAL NOT NULL,
            stock INTEGER NOT NULL
        )",
        [],
    ).unwrap();
}

#[tauri::command]
fn crear_producto(nombre: String, precio: f64, stock: i32) {
    println!("Creando productos: {} {}",nombre,precio);
    let conn = Connection::open("farmacia.db").unwrap();

    conn.execute(
        "INSERT INTO productos (nombre, precio, stock) VALUES (?1, ?2, ?3)",
        (&nombre, precio, stock),
    ).unwrap();
}

#[tauri::command]
fn obtener_productos() -> Vec<Producto> {
    let conn = Connection::open("farmacia.db").unwrap();

    let mut stmt = conn.prepare("SELECT id, nombre, precio, stock FROM productos").unwrap();

    let productos_iter = stmt.query_map([], |row| {
        Ok(Producto {
            id: row.get(0)?,
            nombre: row.get(1)?,
            precio: row.get(2)?,
            stock: row.get(3)?,
        })
    }).unwrap();

    productos_iter.map(|p| p.unwrap()).collect()
}

#[tauri::command]
fn actualizar_producto(id: i32, nombre: String, precio: f64, stock: i32) {
    let conn = Connection::open("farmacia.db").unwrap();

    conn.execute(
        "UPDATE productos SET nombre=?1, precio=?2, stock=?3 WHERE id=?4",
        (&nombre, precio, stock, id),
    ).unwrap();
}

#[tauri::command]
fn eliminar_producto(id: i32) {
    let conn = Connection::open("farmacia.db").unwrap();

    conn.execute(
        "DELETE FROM productos WHERE id=?1",
        [id],
    ).unwrap();
}