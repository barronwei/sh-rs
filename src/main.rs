use std::io::{stdin, stdout, Write};
use std::process::Command;

fn main() {
  loop {
    print!("[sh-rs] ");
    stdout().flush().unwrap();

    let mut line = String::new();
    stdin().read_line(&mut line).unwrap();

    let data = line.trim();

    if data.is_empty() {
      continue;
    }

    let mut list = data.split_whitespace();
    let call = list.next().unwrap();
    let args = list;

    match call {
      "cd" => {}
      _ => {
        let cmd = Command::new(call).args(args).spawn();
        match cmd {
          Ok(mut c) => {
            c.wait().unwrap();
          }
          Err(e) => eprint!("{}\n", e),
        }
      }
    }
  }
}
