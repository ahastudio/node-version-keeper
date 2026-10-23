use std::path::PathBuf;
use std::slice::Iter;

use crate::error::KeeperError;
use crate::verdict::ApplyMode;

#[derive(Debug, Default, PartialEq, Eq)]
pub struct Options {
    pub folder: PathBuf,
    pub write: bool,
    pub upgrade_major: bool,
    pub config_path: Option<PathBuf>,
    pub exclude: Vec<String>,
}

#[derive(Debug, PartialEq, Eq)]
pub enum Command {
    Help,
    Run(Options),
}

impl Options {
    pub fn apply_mode(&self) -> ApplyMode {
        ApplyMode { write: self.write, upgrade_major: self.upgrade_major }
    }
}

pub fn parse_arguments(arguments: &[String]) -> Result<Command, KeeperError> {
    let mut parsed = ParsedArguments::default();
    parsed.accept_all(arguments.iter())?;
    parsed.into_command()
}

// The folder may not have been received yet, so it is held separately; other options are collected straight into Options
#[derive(Default)]
struct ParsedArguments {
    folder: Option<PathBuf>,
    help: bool,
    options: Options,
}

impl ParsedArguments {
    fn accept_all(&mut self, mut remaining: Iter<'_, String>) -> Result<(), KeeperError> {
        while let Some(argument) = remaining.next() {
            self.accept(argument, &mut remaining)?;
        }
        Ok(())
    }

    fn accept(&mut self, argument: &str, remaining: &mut Iter<'_, String>) -> Result<(), KeeperError> {
        if argument.starts_with("--") {
            return self.accept_option(argument, remaining);
        }
        self.accept_folder(argument)
    }

    fn accept_option(&mut self, name: &str, remaining: &mut Iter<'_, String>) -> Result<(), KeeperError> {
        match name {
            "--help" | "--write" | "--upgrade-major" => self.accept_switch(name),
            "--config" | "--exclude" => self.accept_valued(name, remaining),
            _ => Err(KeeperError::new(format!("unknown option: {name}"))),
        }
    }

    fn accept_switch(&mut self, name: &str) -> Result<(), KeeperError> {
        self.help |= name == "--help";
        self.options.write |= name == "--write";
        self.options.upgrade_major |= name == "--upgrade-major";
        Ok(())
    }

    fn accept_valued(&mut self, name: &str, remaining: &mut Iter<'_, String>) -> Result<(), KeeperError> {
        let value = next_value(name, remaining)?;
        self.store_value(name, value);
        Ok(())
    }

    fn store_value(&mut self, name: &str, value: String) {
        match name {
            "--config" => self.options.config_path = Some(PathBuf::from(value)),
            _ => self.options.exclude.push(value),
        }
    }

    fn accept_folder(&mut self, argument: &str) -> Result<(), KeeperError> {
        if self.folder.is_some() {
            return Err(KeeperError::new(format!("only one folder can be given: {argument}")));
        }
        self.folder = Some(PathBuf::from(argument));
        Ok(())
    }

    fn into_command(self) -> Result<Command, KeeperError> {
        if self.help {
            return Ok(Command::Help);
        }
        self.into_run_command()
    }

    fn into_run_command(self) -> Result<Command, KeeperError> {
        self.ensure_write_with_upgrade()?;
        let folder = self.folder.ok_or_else(|| KeeperError::new("a folder must be specified"))?;
        Ok(Command::Run(Options { folder, ..self.options }))
    }

    fn ensure_write_with_upgrade(&self) -> Result<(), KeeperError> {
        if self.options.upgrade_major && !self.options.write {
            return Err(KeeperError::new("--upgrade-major must be used with --write"));
        }
        Ok(())
    }
}

fn next_value(name: &str, remaining: &mut Iter<'_, String>) -> Result<String, KeeperError> {
    remaining.next().cloned().ok_or_else(|| KeeperError::new(format!("{name} requires a value")))
}
