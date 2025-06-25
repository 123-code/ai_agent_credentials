use agent_credentials;
use rusqlite::Connection;

mod input;
mod login;
mod signup;
mod menu;

use input::{get_username, get_password};
use login::log_in;
use signup::sign_up;
use menu::main_menu;

fn main() {
    main_menu();
}


