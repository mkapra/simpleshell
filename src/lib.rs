//! A minimalistic shell for commands
//!
//! # Example
//! ```rust
//! fn version(_: &[String], _: &[Command]) -> Result<(), CommandError> {
//!     println!("v0.1.0");
//!     Ok(())
//! }
//!
//! fn help(_: &[String], commands: &[Command]) -> Result<(), CommandError> {
//!     println!("{}", Color::Blue.paint("HELP"));
//!     commands.iter().for_each(|c| println!("{}: {}", Style::new().bold().paint(&c.name), c.description));
//!     Ok(())
//! }
//!
//! let commands = vec![
//!     Command {
//!         name: "version".to_owned(),
//!         description: "Returns the version of the software".to_owned(),
//!         exec: Box::new(version),
//!     },
//!     Command {
//!         name: "help".to_owned(),
//!         description: "Prints out this help".to_owned(),
//!         exec: Box::new(help),
//!     },
//! ];
//!
//! let shell = Shell::new(None, commands);
//! loop {
//!     if let Err(e) = shell.process(){
//!         eprintln!("{}", e);
//!     }
//! }
//!
//! // User Input:
//! // shell> version
//! // Output:
//! // v0.1.0
//! ```
use std::{
    fmt::Debug,
    fmt::Display,
    io::{self, Write},
};

/// Errors that may occur while processing a command. An error occurs if it was
/// not found or an error occured while executing the command
#[derive(Debug, PartialEq, Eq)]
pub enum CommandError {
    Empty,
    NotFound,
    ExecutionError,
}

impl std::error::Error for CommandError {}

impl Display for CommandError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::Empty => write!(f, "No command given"),
            Self::NotFound => write!(f, "Command not found"),
            Self::ExecutionError => write!(f, "Error while executing command"),
        }
    }
}

/// Represents an executable command
pub struct Command {
    /// This field represents the name of the command that the user will call
    pub name: String,
    /// A short description what this command does
    pub description: String,
    /// The function that will be executed if the user called the command
    pub exec: Box<dyn Fn(&[String], &[Command]) -> Result<(), CommandError>>,
}

impl Command {
    /// Invokes the command
    fn invoke(&self, arguments: &[String], commands: &[Command]) -> Result<(), CommandError> {
        (self.exec)(arguments, commands)
    }
}

impl Debug for Command {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Command")
            .field("cmd_name", &self.name)
            .finish()
    }
}

/// Represents the `Shell` that parses the user input into a command and executes it
pub struct Shell {
    prefix: Option<String>,
    available_commands: Vec<Command>,
}

impl Shell {
    /// Creates a new `Shell`
    ///
    /// # Arguments
    /// * `prefix` - The prefix that should be printed before the user inputs a
    ///              command
    /// * `available_commands` - A list of commands that are executable
    pub fn new(prefix: Option<&str>, available_commands: Vec<Command>) -> Self {
        Shell {
            prefix: prefix.map(|s| s.to_string()),
            available_commands,
        }
    }

    /// Processes a whole command
    ///
    /// This includes:
    /// * Reading the command with arguments from `STDIN`
    /// * Execute the command
    /// * Return the result
    ///
    /// # Returns
    /// This function returns `Ok(())` if everything went fine. Otherwise it
    /// will return a [`CommandError`] which represents the error hat occured
    pub fn process(&self) -> Result<(), CommandError> {
        let mut user_input = self.get_user_input();
        match user_input.pop() {
            Some(requested_cmd) => {
                let selected_command = self
                    .available_commands
                    .iter()
                    .filter(|c| c.name == requested_cmd)
                    .collect::<Vec<&Command>>()
                    .pop();

                match selected_command {
                    Some(cmd) => cmd.invoke(&user_input, &self.available_commands),
                    None => Err(CommandError::NotFound),
                }
            }
            None => Err(CommandError::Empty),
        }
    }

    /// Reads the user input from `STDIN` and splits it at the whitespaces
    fn get_user_input(&self) -> Vec<String> {
        match self.prefix.as_ref() {
            Some(p) => print!("{}", p),
            None => print!("cmdshell> "),
        }
        io::stdout()
            .flush()
            .expect("Could not flush prefix of input");

        let mut user_input = String::new();
        io::stdin()
            .read_line(&mut user_input)
            .expect("Failed to read user input");

        user_input
            .trim()
            .split(' ')
            .map(|s| s.to_string())
            .collect()
    }
}
