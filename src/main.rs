mod command_finder;

use crate::command_finder::CommandFinder;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        println!("Not enough arguments");
        return;
    }

    let command = &args[1];
    let command_history_file = &args[2];

    let command_finder: CommandFinder = CommandFinder::new(command, command_history_file);

    command_finder.find_command_in_file();
}
