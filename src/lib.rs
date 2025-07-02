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
pub fn register_credentials(env_path: &str) -> PyResult<()> {
    let re_username = Regex::new(r"_USERNAME$").unwrap();
    let re_password = Regex::new(r"_PASSWORD$").unwrap();
    let env_variables = read_file(env_path).map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
    

    for (key, value) in &env_variables {
        if re_password.is_match(key) {
  
            let service = key.replace("_PASSWORD", "");

            let username_key = format!("{}_USERNAME", service);
            let username = env_variables.iter()
                .find(|(k, _)| **k == username_key)
                .map(|(_, v)| v.clone())
                .unwrap_or_else(|| "".to_string());
            
            if username.is_empty() {
                continue; 
            }
            
           
            let entry = Entry::new(&service, &username).map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
    
            entry.set_password(value).map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
        }

    }
    Ok(())
}

#[pyfunction]
pub fn register_username(env_path:&str,username:&str) -> PyResult<()> {
    let re = Regex::new(r"_PASSWORD$").unwrap();
    let env_variables = read_file(env_path).map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;
    for(key,value) in env_variables{
        if re.is_match(&key){
            continue
        }
        let entry = String::new();
    }
    Ok(())
}

#[pyfunction]
pub fn get_credentials(env_path: &str, service: &str) -> PyResult<(String, String)> {
    let username_key = format!("{}_USERNAME", service);

 
    let env_variables = read_file(env_path)
        .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

 
    let username = env_variables
        .iter()
        .find(|(k, _)| **k == username_key)
        .map(|(_, v)| v.clone())
        .ok_or_else(|| {
            pyo3::exceptions::PyKeyError::new_err(format!(
                "Username key '{}' not found in .env file",
                username_key
            ))
        })?;

    // Retrieve the password from the system keyring
    let entry = Entry::new(service, &username)
        .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

    let password = entry
        .get_password()
        .map_err(|e| pyo3::exceptions::PyRuntimeError::new_err(e.to_string()))?;

    // Return both username and password so callers have everything they need
    Ok((username, password))
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_and_verify_password() {
        let password = "test123";
        let hashed = hash_password(password).unwrap();
        assert!(verify_password(password, &hashed).unwrap());
        assert!(!verify_password("wrongpass", &hashed).unwrap());
    }
}
