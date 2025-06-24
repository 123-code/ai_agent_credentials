use dialoguer::{theme::ColorfulTheme, Select};
use crate::login::log_in;
use crate::signup::sign_up;

pub fn main_menu() {
    loop {
        let options = vec![
            "Login",
            "Sign Up", 
            "Exit"
        ];

        let selection = Select::with_theme(&ColorfulTheme::default())
            .with_prompt("Welcome! What would you like to do?")
            .items(&options)
            .default(0)
            .interact()
            .unwrap();

        match selection {
            0 => {
                println!("\n--- Login ---");
                log_in();
                break;
            },
            1 => {
                println!("\n--- Sign Up ---");
                sign_up();
                break;
            },
            2 => {
                println!("Goodbye! 👋");
                break;
            },
            _ => unreachable!(),
        }
    }
} 