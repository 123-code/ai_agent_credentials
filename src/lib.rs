use rusqlite::{params, Connection, Result};
use chrono::{DateTime, Utc, Duration};
use bcrypt::{hash, verify};
use pyo3::prelude::*;

pub mod database {
    use super::*;

    pub fn check_timestamp_validity(rfc3339_timestamp: &str) -> bool { 
        let parsed = DateTime::parse_from_rfc3339(rfc3339_timestamp)
            .expect("Invalid RFC3339 timestamp");
        let parsed_utc = parsed.with_timezone(&Utc);
        let now = Utc::now();
        let duration = now.signed_duration_since(parsed_utc);
        duration < Duration::hours(1)
    }

    pub fn migrate_database(conn: &Connection) -> Result<()> {
        let mut stmt = conn.prepare("PRAGMA table_info(passwords)")?; 
        let rows = stmt.query_map([], |row| {
            let column_name: String = row.get(1)?;
            Ok(column_name)
        })?;
        
        let mut has_provider = false;
        for row in rows {
            if let Ok(column_name) = row {
                if column_name == "provider" {
                    has_provider = true;
                    break;
                }
            }
        }
        
        if !has_provider {
            conn.execute("ALTER TABLE passwords ADD COLUMN provider TEXT DEFAULT 'Manual'", [])?;
        }
        
        Ok(())
    }

    pub fn create_table(conn: &Connection) -> Result<()> {
        conn.execute(
            "CREATE TABLE IF NOT EXISTS passwords (
                id INTEGER PRIMARY KEY,
                username TEXT NOT NULL UNIQUE,
                password TEXT NOT NULL
            )",
            [],
        )?;
        
        conn.execute(
            "CREATE TABLE IF NOT EXISTS sessions (
                id INTEGER PRIMARY KEY,
                username TEXT NOT NULL,
                timestamp TEXT NOT NULL
            )",
            [],
        )?;
        
        migrate_database(conn)?;
        
        Ok(())
    }

    pub fn insert_password(conn: &Connection, username: &str, password: &str) -> Result<(), rusqlite::Error> {
        let hashed_password = super::hash_password(password.to_string());
        let mut insert_stmt = conn.prepare(
            "INSERT OR IGNORE INTO passwords (provider, username, password) VALUES (?1, ?2, ?3)"
        )?;
        insert_stmt.execute(params!["Manual", username, hashed_password])?;
        Ok(())
    }

    pub fn retrieve_password(conn: &Connection, username: &str) -> Result<String> {
        let mut stmt = conn.prepare("SELECT password FROM passwords WHERE username = ?1")?;
        let mut rows = stmt.query(params![username])?;
        let row = rows.next()?;
        let password: String = row.expect("REASON").get(0)?;
        Ok(password)
    }

    pub fn create_session(conn: &Connection, username: &str) -> Result<()> {
        let timestamp = Utc::now().to_rfc3339();
        let mut stmt = conn.prepare("INSERT INTO sessions (username, timestamp) VALUES (?1, ?2)")?;
        stmt.execute(params![username, timestamp])?;
        Ok(())
    }
}

#[pyfunction]
pub fn hash_password(password: String) -> String {
    let hashed_password = hash(&password, 12).unwrap();
    hashed_password
}

#[pyfunction]
pub fn verify_password(password: String, hashed_password: String) -> bool {
    let is_valid = verify(&password, &hashed_password).unwrap();
    is_valid
}

#[pymodule]
fn agent_credentials(py: Python, m: Bound<PyModule>) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hash_password, &m)?)?;
    m.add_function(wrap_pyfunction!(verify_password, &m)?)?;
    Ok(())
}

#[no_mangle]
pub extern "C" fn init_library() {
    println!("Library initialized");
}