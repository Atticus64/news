use inquire::Select;

pub fn get_answer<'a>(message: &'a str, options: Vec<&'a str>, err_message: &str) -> &'a str {
    let answer = Select::new(message, options).prompt();

    match answer {
        Ok(choice) => choice,
        Err(_) => {
            println!("Operation cancelled: {err_message}");
            std::process::exit(1);
        }
    }
}

pub fn get_answer_str(message: &str, options: Vec<String>, err_message: &str) -> String {
    let answer = Select::new(message, options).prompt();

    match answer {
        Ok(choice) => choice,
        Err(_) => {
            println!("Operation cancelled: {err_message}");
            std::process::exit(1);
        }
    }
}

pub fn manage_exit(err: &str) {
    println!("Operation cancelled: {err}");
    std::process::exit(1);
}
