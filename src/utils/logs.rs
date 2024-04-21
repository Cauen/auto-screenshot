use std::fs::OpenOptions;
use std::io::Write;
use std::path::Path;

fn write_log_message(message: &str) {
    let path = Path::new("logs.txt");
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open(path)
        .unwrap();

    let date_and_message = format!("[{}]: {}", chrono::Local::now().format("%Y-%m-%d %H:%M:%S"), message);
    if let Err(e) = writeln!(file, "{}", date_and_message) {
        eprintln!("Error writing to file: {}", e);
    }
}

pub fn log_message(message: &str) {
    println!("Logging message: {}", message);
    write_log_message(message);
}
