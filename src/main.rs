#[macro_use] extern crate clap;

use std::{ io, ffi, process };

const STATUS_ERROR: i32 = 1;

const ERROR_NO_COMMAND: &'static str = "You must supply a command argument";

fn make_command<It: Iterator<Item = S>, S: AsRef<ffi::OsStr>>(mut args: It) -> process::Command {
    let mut cmd = process::Command::new(args.next().expect(ERROR_NO_COMMAND));
    args.fold(&mut cmd, |c, a| {
        let os_arg: &ffi::OsStr = a.as_ref();
        c.arg(os_arg)
    }).stdin(process::Stdio::piped()).stdout(process::Stdio::inherit());

    cmd
}

fn exit_status_result(status: &process::ExitStatus) -> Result<(), String> {
    if status.success() {
        Ok(())
    } else {
        let message = match status.code() {
            Some(code) => format!("Child process exited with code: {}", code),
            None => format!("Child process terminated by signal"),
        };
        Err(message)
    }
}

fn main() {
    use io::{ BufRead, Write };

    let matches = clap::App::new(option_env!("CARGO_PKG_NAME").unwrap_or("eachline"))
        .author(crate_authors!("\n"))
        .version(option_env!("CARGO_PKG_VERSION").unwrap_or(""))
        .about("stdin line shuffling utility")
        .arg(clap::Arg::with_name("newlines")
            .short("n")
            .long("newlines")
            .help("Print newlines after each child process's output")
            .takes_value(false))
        .arg(clap::Arg::with_name("verbose")
             .short("v")
             .long("verbose")
             .help("Print verbose output")
             .takes_value(false))
        .arg(clap::Arg::with_name("command")
             .multiple(true)
             .last(true))
        .get_matches();

    let stdin = io::stdin();
    let mut exit_statuses = stdin.lock().lines().map(|line| line.unwrap()).map(|line| {
        let line = line.as_bytes();
        let mut command = make_command(matches.values_of("command").expect(ERROR_NO_COMMAND));
        let mut child = command.spawn().unwrap();
        child.stdin.as_mut().unwrap().write_all(line).unwrap();
        let status = exit_status_result(&child.wait().unwrap());
        if matches.is_present("newlines") {
            println!("");
        }
        if matches.is_present("verbose") {
            if let Err(ref err) = status {
                eprintln!("{}", err);
            }
        }
        status
    });
    let fst = exit_statuses.next().unwrap_or(Ok(()));
    let final_status = exit_statuses
        .fold(fst, |a, b| a.and(b));
    if final_status.is_err() {
        process::exit(STATUS_ERROR);
    }
}
