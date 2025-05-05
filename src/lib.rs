use std::{ffi::OsStr, iter::Iterator};

pub use xtask_cmdwrap_macro::cmd as command;

// TODO: guard behind feature flags
#[cfg(feature = "make")]
pub mod make;

pub trait Command: Into<std::process::Command> {
    /// Add an argument
    fn arg<S: AsRef<OsStr>>(&mut self, arg: S) -> &mut Self;

    /// Add an argument if the predicate evaluates to true
    fn arg_if<S: AsRef<OsStr>>(&mut self, arg: S, predicate: bool) -> &mut Self {
        if predicate {
            self.arg(arg)
        } else {
            self
        }
    }

    /// Add each argument in an iterator
    fn args<S: AsRef<OsStr>, I: Iterator<Item = S>>(&mut self, args: I) -> &mut Self {
        for arg in args {
            self.arg(arg);
        }
        self
    }

    /// For each argument, add it when predicate evaluates to true
    fn args_if<S: AsRef<OsStr>, I: Iterator<Item = S>, P: FnMut(&S) -> bool>(
        &mut self,
        args: I,
        predicate: P,
    ) -> &mut Self {
        let args = args.filter(predicate);
        self.args(args)
    }
}

impl Command for std::process::Command {
    #[inline]
    fn arg<S: AsRef<OsStr>>(&mut self, arg: S) -> &mut Self {
        self.arg(arg)
    }

    #[inline]
    fn args<S: AsRef<OsStr>, I: Iterator<Item = S>>(&mut self, args: I) -> &mut Self {
        self.args(args)
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::process::Command as StdCommand;

    #[test]
    fn test_arg() {
        let mut cmd = StdCommand::new("echo");
        cmd.arg("hello");
        assert_eq!(cmd.get_program().to_string_lossy(), "echo");
        assert_eq!(cmd.get_args().next().unwrap().to_string_lossy(), "hello");
    }

    #[test]
    fn test_args() {
        let mut cmd = StdCommand::new("echo");
        let args = vec!["hello", "world"];
        cmd.args(args.iter());
        assert_eq!(cmd.get_program().to_string_lossy(), "echo");
        assert_eq!(cmd.get_args().next().unwrap().to_string_lossy(), "hello");
        assert_eq!(cmd.get_args().nth(1).unwrap().to_string_lossy(), "world");
    }

    #[test]
    fn test_arg_if_true() {
        let mut cmd = StdCommand::new("echo");
        cmd.arg_if("hello", true);
        assert_eq!(cmd.get_program().to_string_lossy(), "echo");
        assert_eq!(cmd.get_args().next().unwrap().to_string_lossy(), "hello");
    }

    #[test]
    fn test_arg_if_false() {
        let mut cmd = StdCommand::new("echo");
        cmd.arg_if("hello", false);
        assert_eq!(cmd.get_program().to_string_lossy(), "echo");
        assert_eq!(cmd.get_args().next(), None);
    }

    #[test]
    fn test_args_if() {
        let mut cmd = StdCommand::new("echo");
        let args = vec!["hello", "world"];
        cmd.args_if(args.iter(), |arg| arg.to_string().starts_with("h"));
        assert_eq!(cmd.get_program().to_string_lossy(), "echo");
        assert_eq!(cmd.get_args().next().unwrap().to_string_lossy(), "hello");
        assert_eq!(cmd.get_args().nth(1), None);
    }
}
