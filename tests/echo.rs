use xtask_cmdwrap::command;

#[command(name = "/bin/echo")]
struct Echo {
    /// Do not output the trailing newline
    #[arg(no_val, prefix = "-", name = "n")]
    no_newline: bool,
    /// Enable interpretation fo backslash escapes
    #[arg(no_val, prefix = "-", name = "e")]
    escape: bool,
    /// Display help and exit
    #[arg(no_val)]
    help: bool,
    /// Display version info and exit
    #[arg(no_val)]
    version: bool,
    /// The string to echo
    #[arg(no_opt)]
    string: &str,
}

#[test]
fn echo_hello_world() {
    let mut echo = Echo::new();

    echo.string("hello world");
    assert_eq!(echo.string_repr(), "/bin/echo hello world".to_string());

    let res = echo.cmd().output().unwrap().stdout;
    assert_eq!(String::from_utf8(res).unwrap(), "hello world\n")
}

#[test]
fn echo_escaped_no_newline() {
    let mut echo = Echo::new();
    echo.escape().no_newline().string("hello\\nworld");
    assert_eq!(echo.string_repr(), "/bin/echo -e -n hello\\nworld".to_string());

    let res = echo.cmd().output().unwrap().stdout;
    assert_eq!(String::from_utf8(res).unwrap(), "hello\nworld")
}