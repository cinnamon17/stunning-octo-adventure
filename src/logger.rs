use std::fs::OpenOptions;
use std::io::Write;

fn log_debug(mensaje: &str) {
    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("debug.log")
        .unwrap();
    writeln!(file, "{}", mensaje).unwrap();
}
