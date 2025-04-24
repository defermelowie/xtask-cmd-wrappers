// Export procedural macros
pub use cmd_wrappers_macro::{command, command2};

mod cmd;
pub use cmd::Command;

pub fn add(left: u64, right: u64) -> u64 {
    left + right
}

mod tests {
    use super::*;
    use std::ffi::OsStr;

    // #[command]
    // struct Echo {
    //     /// Do not output the trailing newline
    //     #[arg(single)]
    //     n: bool,
    //     /// Enable interpretation fo backslash escapes
    //     #[arg(single)]
    //     e: bool,
    //     /// Display help and exit
    //     #[arg(double)]
    //     help: bool,
    //     /// Display version info and exit
    //     #[arg(double)]
    //     version: bool,
    //     /// The string to echo
    //     string: &OsStr,
    // }

    #[command2(name="/bin/echo")]
    struct Echo2 {
        /// Do not output the trailing newline
        #[arg(flag, name="n", prefix="-")]
        no_newline: bool,
        /// Enable interpretation fo backslash escapes
        #[arg(flag, name="e", prefix="-")]
        escape: bool,
        /// Display help and exit
        #[arg(flag, postfix="=", prefix="--")]
        help: bool,
        /// Display version info and exit
        #[arg(flag, prefix="--")]
        version: bool,
        /// The string to echo
        string: &OsStr,
    }
}
