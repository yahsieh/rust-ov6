use std::io::{self, BufRead, Write};
use std::process::Command;

const PROMPT: &str = "> ";

pub struct Cmd {
    prog: String,
    args: Vec<String>,
}

impl From<Vec<String>> for Cmd {
    fn from(value: Vec<String>) -> Self {
        if value.len() == 1 {
            return Cmd {
                prog: value[0].clone(),
                args: vec![],
            };
        } else {
            return Cmd {
                prog: value[0].clone(),
                args: value.into_iter().skip(1).collect(),
            };
        }
    }
}

fn parse_line(line: String) -> Result<Vec<String>, ()> {
    let ret: Vec<String> = line
        .split_ascii_whitespace()
        .map(|s| s.to_string())
        .collect();
    if ret.len() > 0 {
        Ok(ret)
    } else {
        Err(())
    }
}

fn exec(cmd: Cmd) {
    match Command::new(&cmd.prog).args(&cmd.args).output() {
        Ok(o) => {
            io::stdout().write_all(&o.stdout).unwrap();
            io::stderr().write_all(&o.stderr).unwrap();
        }
        Err(msg) => {
            println!("{:?}", msg)
        }
    }
}

fn main() -> io::Result<()> {
    loop {
        print!("{}", PROMPT);
        io::stdout().flush().unwrap();
        let mut buffer = String::new();
        let stdin = io::stdin();
        let mut handle = stdin.lock();
        handle.read_line(&mut buffer)?;
        if let Ok(cmd) = parse_line(buffer) {
            exec(cmd.into());
        }
    }
}
