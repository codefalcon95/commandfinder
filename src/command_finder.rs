use std::fs::File;
use std::io::Read;
pub struct CommandFinder {
    command: String,
    command_history_file: String,
}

impl CommandFinder {
    pub fn new(command: &str, command_history_file: &str) -> CommandFinder {
        CommandFinder {
            command: command.to_string(),
            command_history_file: command_history_file.to_string(),
        }
    }

    pub fn find_command_in_file(self) {s 
        let mut file = match File::open(self.command_history_file) {
            Ok(file) => file,
            Err(_) => {
                panic!("Could not open file");
            }
        };

        let mut contents = String::new();
        match file.read_to_string(&mut contents) {
            Ok(_) => (),
            Err(_) => {
                panic!("Could not read file");
            }
        }

        // find command
        let lines: Vec<&str> = contents.lines().collect();

        for line in lines {
            if line.contains(&self.command) {
                println!("{}", line);
            }
        }
    }
}

#[cfg(test)]
mod test {
    use crate::command_finder::CommandFinder;
}
