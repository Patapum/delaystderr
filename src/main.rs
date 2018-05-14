#![feature(io)]

use std::io::Write as ioWrite;
use std::io::Read;
use std::{ffi::OsStr, process::{Command, Stdio}};
use std::fmt::Write as fmtWrite;

fn spawn<S: AsRef<OsStr>, I: IntoIterator>(program: S, args: I) -> std::process::Child
    where
        I::Item: AsRef<OsStr>
{
    return Command::new(program)
        .args(args)
        .stderr(Stdio::piped())
        .spawn()
        .unwrap();
}

fn main() {
    let mut args = std::env::args_os();
    let mut cmd = spawn(
        args.nth(1).expect("Command should be provided"),
        args);

    let mut errors = String::new();
    for char in cmd.stderr.as_mut().unwrap().chars() {
        let char = char.unwrap();
        write!(&mut errors, "{}", char).unwrap();
        print!("{}", char);
        std::io::stdout().flush().expect("Could not flush stdout");
    };

    let result = cmd.wait().unwrap().code().unwrap();

    if result != 0 {
        eprint!("{}", errors);
    }

    std::process::exit(result);
}
