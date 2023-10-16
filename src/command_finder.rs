
  pub struct CommandFinder{
    pub command: String,
  }

 
 impl CommandFinder {
  pub fn new( command: &str) -> CommandFinder {
    CommandFinder {
      command: command.to_string(),
    }
  }
}


