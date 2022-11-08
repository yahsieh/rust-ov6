use std::env;
use std::fs::File;
use std::io::{self, Read, Write};

fn cat(path: &str) -> io::Result<()> {
    let file = File::open(path)?;
    let mut buf_reader = io::BufReader::new(file);
    let mut buffer = String::new();
    buf_reader.read_to_string(&mut buffer)?;
    io::stdout().write(buffer.as_bytes())?;
    Ok(())
}

fn main() -> io::Result<()> {
    let args: Vec<String> = env::args().skip(1).collect();
    for path in &args {
        cat(path)?;
    }
    Ok(())
}
