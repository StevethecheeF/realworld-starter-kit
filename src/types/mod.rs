pub mod data_beans;

use dotenv_codegen::dotenv;

pub const SESSION_TOKEN:&str = "session.token";
pub const API_ENDPOINT: &str = dotenv!("API_ENDPOINT");
