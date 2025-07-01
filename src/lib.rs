use pyo3::prelude::*;
use bcrypt::{hash, verify, DEFAULT_COST};
use regex::Regex;
use keyring::Entry;
use env_file_reader::read_file;

#[pyfunction]
pub fn hash_password(password: &str) -> PyResult<String> {
    let hashed = hash(password, DEFAULT_COST).map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
    Ok(hashed)
}

#[pyfunction]
pub fn verify_password(password: &str, hashed_password: &str) -> PyResult<bool> {
    let is_valid = verify(password, hashed_password).map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
    Ok(is_valid)
}

#[pyfunction]
pub fn register_credentials(env_path:&str) -> PyResult<()> {
    let re = Regex::new(r"_USERNAME$").unwrap();
    let env_variables = read_file(env_path).map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
    for (key, value) in env_variables {
        if re.is_match(&key) {
            continue;
        }
        let entry = Entry::new(&key, &value).map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        entry.set_password(&value).map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
    }
    Ok(())
}

#[pyfunction]
pub fn get_credentials(env_path: &str, username: &str) -> PyResult<String> {
    let re_username = Regex::new(r"_USERNAME$").unwrap();
    let env_variables = read_file(env_path).map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
    let mut target_service = String::new();
    
    for (key, value) in &env_variables {
        if re_username.is_match(key) && value == username {
            target_service = key.replace("_USERNAME", "");
            break;
        }
    }
    
    if target_service.is_empty() {
        return Err(pyo3::exceptions::PyKeyError::new_err(format!("Username '{}' not found in credentials", username)));
    }
    
    let password_key = format!("{}_PASSWORD", target_service);
    for (key, value) in &env_variables {
        if key == &password_key {
            let entry = Entry::new(key, value).map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            let password = entry.get_password().map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
            return Ok(password);
        }
    }
    
    Err(pyo3::exceptions::PyKeyError::new_err(format!("Password for username '{}' not found", username)))
}

#[pyfunction]
pub fn set_master_password(master_password: &str) -> PyResult<()> {
    let entry = Entry::new("AI_CREDENTIALS_MASTER", "master").map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
    let hashed = hash(master_password, DEFAULT_COST).map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
    entry.set_password(&hashed).map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
    Ok(())
}

#[pyfunction]
pub fn verify_master_password(master_password: &str) -> PyResult<bool> {
    let entry = Entry::new("AI_CREDENTIALS_MASTER", "master").map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
    let hashed = entry.get_password().map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
    let is_valid = verify(master_password, &hashed).map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
    Ok(is_valid)
}

#[pymodule]
fn ai_credentials(_py: Python, m: &PyModule) -> PyResult<()> {
    m.add_function(wrap_pyfunction!(hash_password, m)?)?;
    m.add_function(wrap_pyfunction!(verify_password, m)?)?;
    m.add_function(wrap_pyfunction!(register_credentials, m)?)?;
    m.add_function(wrap_pyfunction!(get_credentials, m)?)?;
    m.add_function(wrap_pyfunction!(set_master_password, m)?)?;
    m.add_function(wrap_pyfunction!(verify_master_password, m)?)?;
    Ok(())
}
