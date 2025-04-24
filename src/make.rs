//! Wrapper for GNU make

use crate::command;

#[command]
struct Make {
    /// Change to DIRECTORY before doing anything.
    #[arg(prefix="-", name="C")]
    directory: String,
    /// Environment variables override makefiles.
    #[arg(no_val, prefix="-", name="e")]
    env_overwrites: bool,
    /// Evaluate STRING as a makefile statement.
    #[arg()]
    eval: String,
    /// Read FILE as a makefile.
    #[arg()]
    file: String,
    /// Ignore errors from recipes.
    #[arg(no_val, prefix="-", name="i")]
    ignore_errors: bool,
    /// Search DIRECTORY for included makefiles.
    #[arg(prefix="-", name="I")]
    include_dir: String,
    /// Specify makefile target
    #[arg(no_opt)]
    target: String,
}