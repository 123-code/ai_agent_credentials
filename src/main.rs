use std::io;
use bcrypt::{hash, verify};
use rusqlite::{params,Connection,Result};


fn create_table(conn:&Connection) -> Result<()>{
    conn.execute(
        "CREATE TABLE IF NOT EXISTS users (
            id INTEGER PRIMARY KEY,
            username TEXT NOT NULL UNIQUE,
            password TEXT NOT NULL
        )",
        "CREATE TABLE IF NOT EXISTS sessions(
            id INTEGER PRIMARY KEY,
            username TEXT NOT NULL,
            timestamp TEXT NOT NULL
        )",
        []
    )?;
    Ok(())
}

fn insert_password(conn: &Connection, username: &str, password: &str) -> Result<(), rusqlite::Error> {
    let hashed_password = hash_password(password);
    let mut insert_stmt = conn.prepare(
        "INSERT OR IGNORE INTO users (username, password) VALUES (?1, ?2)"
    )?;
    insert_stmt.execute(params![username, hashed_password])?;
    Ok(())
}

fn retrieve_password(conn: &Connection,username:&str) -> Result<String>{
    let mut stmt = conn.prepare("SELECT password FROM users WHERE username = ?1")?;
    let mut rows = stmt.query(params![username])?;
    let row = rows.next()?;
    let password: String = row.expect("REASON").get(0)?;
    Ok(password)
}


pub fn hash_password(password: &str) -> String {
    let hashed_password = hash(password,12).unwrap();
    hashed_password
}

pub fn verify_password(password: &str, hashed_password: &str) -> bool {
    let is_valid = verify(password, hashed_password).unwrap();
    is_valid
}


fn main(){
    println!("Enter your username: ");
    let mut username = String::new();
    io::stdin()
    .read_line(&mut username)
    .expect("Error al leer la línea");
    let username = username.trim();
    
    println!("Enter your password: ");
    let mut password = String::new();
    io::stdin()
    .read_line(&mut password)
    .expect("Error al leer la línea");
    let password = password.trim();
    
    let hashed_password = hash_password(password);
    let conn = Connection::open("passwords.db").unwrap();
    create_table(&conn).unwrap();
    insert_password(&conn, username, &hashed_password).unwrap();
    let retrieved_password = retrieve_password(&conn, username).unwrap_or_else(|_| {
        println!("User not found, password not retrieved.");
        String::new()
    });
    if !retrieved_password.is_empty() {
        println!("Retrieved password: {}", retrieved_password);
        let is_valid = verify_password(password, &hashed_password);
        println!("Is valid: {}", is_valid);
    }
   
}


