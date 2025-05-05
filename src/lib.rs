use std::{ffi::OsStr, iter::Iterator};

pub use xtask_cmdwrap_macro::cmd as command;

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

// TODO: guard behind feature flags
#[cfg(feature = "make")]
pub mod make;
