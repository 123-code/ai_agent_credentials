use rusqlite::{params, Connection, Result};
use chrono::{DateTime, Utc, Duration};
use bcrypt::{hash, verify};
#[cfg(feature = "extension-module")]
use pyo3::prelude::*;
use zerocopy::AsBytes;
use crate::encrypt_aes::encrypt_aes;

pub mod encrypt_aes;

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

        conn.execute("CREATE TABLE IF NOT EXISTS keys(
            id INTEGER PRIMARY KEY,
            master_password TEXT NOT NULL,
            encrypted_private_key TEXT NOT NULL
        )",
        [],
        )?;
        
        migrate_database(conn)?;
        
        Ok(())
    }

    pub fn insert_master_password(conn: &Connection, master_password: &Vec<u8>) -> Result<(), rusqlite::Error> {
        let mut insert_stmt = conn.prepare("INSERT INTO keys (master_password, encrypted_private_key) VALUES (?1, ?2)")?;
        insert_stmt.execute(params![master_password, ""])?;
        Ok(())
    }

    pub fn insert_password(conn: &Connection, username: &str, password: &Vec<u8>) -> Result<(), rusqlite::Error> {
   
        let encrypted_password = encrypt_aes("password", password);
        let mut insert_stmt = conn.prepare(
            "INSERT OR IGNORE INTO passwords (provider, username, password) VALUES (?1, ?2, ?3)"
        )?;
        insert_stmt.execute(params!["Manual", username, encrypted_password])?;
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

    pub fn store_private_key(conn: &Connection, username: &str, private_key: &Vec<u8>) -> Result<()> {
        let mut stmt = conn.prepare("UPDATE sessions SET private_key = ?1 WHERE username = ?2")?;
        stmt.execute(params![private_key, username])?;
        Ok(())
    }

    pub fn retrieve_session(conn: &Connection) -> Result<String> {
        let mut stmt = conn.prepare("SELECT username FROM sessions LIMIT 1")?;
        let mut rows = stmt.query(params![])?;
        let row = rows.next()?.expect("No session found");
        let username: String = row.get(0)?;
        Ok(username)
    }

    pub fn retrieve_master_password(conn: &Connection) -> Result<Vec<u8>> {
        let mut stmt = conn.prepare("SELECT master_password FROM keys LIMIT 1")?;
        let mut rows = stmt.query(params![])?;
        let row = rows.next()?.expect("No master password found");
        let master_password: Vec<u8> = row.get(0)?;
        Ok(master_password)
    }
}

#[cfg_attr(feature = "extension-module", pyfunction)]
pub fn hash_password(password: &str) -> String {
    hash(password, 12).unwrap()
}

#[cfg_attr(feature = "extension-module", pyfunction)]
pub fn verify_password(password: &str, hashed_password: &str) -> bool {
    verify(password, hashed_password).unwrap()
}

#[cfg(feature = "extension-module")]
#[pymodule]
fn agent_credentials(py: Python, m: &Bound<PyModule>) -> PyResult<()> {
    use pyo3::wrap_pyfunction;
    m.add_function(wrap_pyfunction!(hash_password, m)?)?;
    m.add_function(wrap_pyfunction!(verify_password, m)?)?;
    Ok(())
}

#[no_mangle]
pub extern "C" fn init_library() {
    println!("Library initialized");
}