use keyring::{Entry,Result};
use dotenv::dotenv;
use env_file_reader::read_file;
use std::env;
use regex::Regex;
mod HashBcrypt;
use HashBcrypt::hash_password;

fn main() -> Result<()>{
    dotenv().ok();
    let env_variables = read_file(".env");
    let re = Regex::new(r"_USERNAME$").unwrap();
    match env_variables {
        Ok(vars) => {
            for (key, value) in vars {
                if re.is_match(&key){
                    println!("skipping addition of {} to keyring", key);
                    continue;
                }
                println!("adding {} to keyring", key);
                let entry = Entry::new(&key,&value)?;
                entry.set_password(&value)?;
                let password = entry.get_password()?;
                println!("password: {}",password);
                let hashed_password = hash_password(&password);
                println!("hashed password: {}",hashed_password);
            }
        }
        Err(e) => {
            eprintln!("Error reading .env file: {}", e);
        }
    }
    
    Ok(())
    /*
    let entry = Entry::new("Google","naranjojose256@gmail.com")?;
    entry.set_password("passbword")?;
    let password = entry.get_password()?;
    println!("password: {}",password);
    Ok(())
    */
}