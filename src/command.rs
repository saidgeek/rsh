use std::process::{Child, Command as cmd};

pub struct Command {
    keyword: String,
    args: Vec<String>,
}

impl Command {
    pub fn enter(self) {
        let mut child: Child = match self.keyword {
            _ => cmd::new(self.keyword).args(self.args).spawn().unwrap(),
        };

        child.wait().expect("Wait error!");
    }
}

pub fn new(input: &String) -> Command {
    let mut split_input = input.trim().split_whitespace();

    let command = Command {
        keyword: split_input.next().unwrap().to_string(),
        args: split_input.map(|i| i.to_string()).collect(),
    };

    command
}
