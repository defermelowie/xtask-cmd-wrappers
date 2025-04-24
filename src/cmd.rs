//! Ergonomic rapper around [`std::process::Command`]

use std::ffi::OsStr;

pub trait Command {
    /// Add an argument to pass to the program
    fn arg<S: AsRef<OsStr>>(&mut self, arg: S) -> &mut Self;

    /// Add an argument to pass to the program only if the predicate is true
    fn arg_if<S: AsRef<OsStr>>(&mut self, predicate: bool, arg: S) -> &mut Self;

    /// Convert a command with its arguments to a [String]
    fn stringify(&self) -> String;
}

impl Command for std::process::Command {
    fn arg<S: AsRef<OsStr>>(&mut self, arg: S) -> &mut Self {
        self.arg(arg)
    }

    fn arg_if<S: AsRef<OsStr>>(&mut self, predicate: bool, arg: S) -> &mut Self {
        match predicate {
            true => Command::arg(self, arg),
            false => self,
        }
    }

    fn stringify(&self) -> String {
        let program = self.get_program().to_string_lossy();
        let args = self
            .get_args()
            .map(|a| a.to_string_lossy())
            .fold(String::new(), |s, a| s + " " + &a);
        format!("{}{}", program, args)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn add_arg_single() {
        let mut cmd = std::process::Command::new("echo");
        Command::arg(&mut cmd, "hello world");

        assert_eq!(Command::stringify(&cmd), "echo hello world")
    }

    #[test]
    fn add_arg_multiple() {
        let mut cmd = std::process::Command::new("echo");
        Command::arg(&mut cmd, "-n");
        Command::arg(&mut cmd, "-e");
        Command::arg(&mut cmd, "hello\tworld");

        assert_eq!(Command::stringify(&cmd), "echo -n -e hello\tworld")
    }
}
