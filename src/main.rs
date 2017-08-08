use std::{ env, io, process };

fn make_command() -> process::Command {
    let mut args = env::args().skip(1);
    let mut cmd = process::Command::new(args.next().expect("You must supply a command argument"));
    args.fold(&mut cmd, |c, a| c.arg(a))
        .stdin(process::Stdio::piped())
        .stdout(process::Stdio::inherit());
    cmd
}

fn main() {
    use io::{ BufRead, Write };

    let stdin = io::stdin();
    stdin.lock().lines().map(|line| line.unwrap()).all(|line| {
        let line = line.as_bytes();
        let mut command = make_command();
        let mut child = command.spawn().unwrap();
        child.stdin.as_mut().unwrap().write_all(line).unwrap();
        let ret = child.wait().is_ok();
        println!("");
        ret
    });
}
