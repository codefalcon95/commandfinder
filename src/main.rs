mod command_finder;

use crate::command_finder::CommandFinder;
use std::env;

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() < 2 {
        println!("Not enough arguments");
        return;
    }

    println!("Command: {}", args[1]);
    let command_finder: CommandFinder = CommandFinder::new(&args[1]);
    println!("{}", command_finder.command);
}

// take a command from the user
// open the command line session history
// find every line that has the command
// return that command
