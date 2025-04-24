# Command wrappers for cargo xtask pattern

> [!IMPORTANT]
> This is a work in progress, nothing is guaranteed (yet)

## Usage

The main idea is to define commands one of the following ways:

- Single/double dash specifier on arguments
  ```rust
    use std::ffi::OsStr;

    #[command]
    struct Echo {
        /// Do not output the trailing newline
        #[arg(single)]
        n: bool,
        /// Enable interpretation fo backslash escapes
        #[arg(single)]
        e: bool,
        /// Display help and exit
        #[arg(double)]
        help: bool,
        /// Display version info and exit
        #[arg(double)]
        version:bool,
        /// The string to echo
        string: OsStr,
    }
  ```
- Specifier for everything, sensible defaults
  - Defaults:
    - `prefix="--"`
    - `postfix=" "`
    - `name=IDENT`
    - `flag` is absent
  - Example:
    ```rust
    use std::ffi::OsStr;

    #[cmd(name="/bin/echo")]
    struct Echo {
        /// Do not output the trailing newline
        #[cmd::opt(flag, prefix="-", name="n")]
        no_newline: bool,
        /// Enable interpretation fo backslash escapes
        #[cmd::opt(flag, prefix="-", name="e")]
        escape: bool,
        /// Display help and exit
        #[cmd::opt(flag, prefix="--")]
        help: bool,
        /// Display version info and exit
        #[cmd::opt(flag, prefix="--")]
        version: bool,
        /// The string to echo
        string: OsStr,
    }
    ```
  - Result:
    ```sh
    /bin/echo [-n] [-e] [--help] [--version] STRING
    ```

