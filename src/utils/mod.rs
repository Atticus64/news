pub mod config;
pub mod constants;
use termimad::crossterm::style::Stylize;

pub fn get_read_error(msg: &str) -> &str {

    if msg.contains("Dns Failed") {
        return "Failed in http request"
    } else {
        return "Error to get news"
    }
}

pub fn manage_error(message: &str, custom_message: Option<&str>) {

    let error = {
        if custom_message.is_none() {
            get_read_error(message)
        } else {
            custom_message.unwrap()
        }
    };

    println!("\n{}: {}", "Error".to_string().red(), error);
    println!("{}: {}", "Description".to_string().yellow(), message);
}