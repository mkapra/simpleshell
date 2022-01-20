# Crate SimpleShell
A crate that provides a simple interface for executing commands from the user.

```rust
use simple_shell::{Shell, Command, CommandError};
use ansi_term::{Color, Style};

fn version(_: &[String], _: &[Command]) -> Result<(), CommandError> {
    println!("v0.1.0");
    Ok(())
}

fn help(_: &[String], commands: &[Command]) -> Result<(), CommandError> {
    println!("{}", Color::Blue.paint("HELP"));
    commands.iter().for_each(|c| println!("{}: {}", Style::new().bold().paint(&c.name), c.description));
    Ok(())
}

let commands = vec![
    Command {
        name: "version".to_owned(),
        description: "Returns the version of the software".to_owned(),
        exec: Box::new(version),
    },
    Command {
        name: "help".to_owned(),
        description: "Prints out this help".to_owned(),
        exec: Box::new(help),
    },
];

let shell = Shell::new(None, commands);
loop {
    if let Err(e) = shell.process(){
        eprintln!("{}", e);
    }
}
```

Results in:
```
$ shell> version
v0.1.0
```
