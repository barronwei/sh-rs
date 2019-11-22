use std::io::{stdin, stdout, Write};
use std::process::Command;

fn main() {
  loop {
    print!("> ");
    stdout().flush().unwrap();

    let mut input = String::new();
    stdin().read_line(&mut input).unwrap();

    let mut parts = input.trim().split_whitespace();

    let cmd = parts.next().unwrap();
    let arg = parts;

    let mut res = Command::new(cmd).args(arg).spawn().unwrap();
    res.wait().unwrap();
  }
}
