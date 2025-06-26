use rusqlite::Connection;

mod input;
mod login;
mod signup;
mod menu;
mod encrypt_aes;
mod encrypt_rsa;

use input::{get_username, get_password};
use login::log_in;
use signup::sign_up;
use menu::main_menu;
use crate::encrypt_aes::{encrypt_aes,decrypt_aes};
use crate::encrypt_rsa::{encrypt_data,decrypt_data};
fn main() {
    // Application entry point. Uncomment the AES demo or the main menu as needed.
     main_menu();


   // main_menu();
}


