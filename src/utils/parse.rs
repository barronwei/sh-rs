struct Exec {
  value: String,
}

struct Pipe {
  child: Vec<Exec>,
}

struct Cond {
  child: Vec<Pipe>,
}

struct List {
  child: Vec<Cond>,
}

pub enum Token {
  Exec(Exec),
  Pipe(Pipe),
  Cond(Cond),
  List(List),
}

pub fn init_(t: &str) -> Token {
  let mut pipe = Pipe { child: vec![] };
  let mut cond = Cond { child: vec![] };
  let mut list = List { child: vec![] };

  cond.child.push(pipe);
  list.child.push(cond);

  return Token::List(list);
}

pub fn parse(t: &str) -> Token {
  match t {
    _ => Token::Exec(Exec {
      value: String::from(t),
    }),
  }
}

#[cfg(test)]
mod tests {
  use super::*;
}
