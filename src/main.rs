mod command;

use std::io::{stdin, stdout, Write};

fn main() {
    loop {
        print!("> ");
        stdout().flush().expect("flush error!");

        let mut input: String = String::new();
        stdin().read_line(&mut input).unwrap();

        let cmd = command::new(&input);
        cmd.enter();
    }
}
