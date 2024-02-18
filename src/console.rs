use crossterm::{
    style::{Color, Print, ResetColor, SetForegroundColor},
    ExecutableCommand,
};

pub fn print_log(message: &str) {
    let mut stdout = std::io::stdout();
    stdout
        .execute(SetForegroundColor(Color::Green))
        .expect("Failed to set foreground color");
    stdout
        .execute(Print(message))
        .expect("Failed to print message");
    stdout
        .execute(ResetColor)
        .expect("Failed to reset color");
}

pub fn print_warn(message: &str) {
    let mut stdout = std::io::stdout();
    stdout
        .execute(SetForegroundColor(Color::Yellow))
        .expect("Failed to set foreground color");
    stdout
        .execute(Print(message))
        .expect("Failed to print message");
    stdout
        .execute(ResetColor)
        .expect("Failed to reset color");
}

pub fn print_error(message: &str) {
    let mut stdout = std::io::stdout();
    stdout
        .execute(SetForegroundColor(Color::Red))
        .expect("Failed to set foreground color");
    stdout
        .execute(Print(message))
        .expect("Failed to print message");
    stdout
        .execute(ResetColor)
        .expect("Failed to reset color");
}

pub fn wait_for_key_and_exit(key_code: crossterm::event::KeyCode, code: i32) {
    print_warn(&format!(
        "Press {:?} to exit: ",
        crossterm::event::KeyCode::Esc
    ));
    loop {
        if crossterm::event::poll(std::time::Duration::from_millis(2000)).unwrap() {
            if let crossterm::event::Event::Key(key_event) = crossterm::event::read().unwrap() {
                if key_event.code == key_code {
                    break;
                }
            }
        }
    }
    std::process::exit(code);
}

pub fn wait_input() -> String {
    print_warn("Write folder path and press enter to continue:\n");

    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    input.trim().to_string()
}
