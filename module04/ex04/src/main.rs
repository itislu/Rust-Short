use std::env;
use std::io::Read;
use std::process;

struct ChildProc {
    cmd: process::Command,
    proc: process::Child,
}

fn main() {
    // Input parsing
    let mut args: Vec<String> = env::args().collect();
    args.remove(0);
    if args.is_empty() {
        return;
    }

    let args_list = parse_args_list(&args);
    let cmd_list = setup_cmd_list(&args_list);
    let mut child_list = spawn_child_procs(cmd_list);

    // Waiting for child processes
    let mut i = 0;
    while !child_list.is_empty() {
        i %= child_list.len();

        let child = &mut child_list[i];
        match child.proc.try_wait() {
            Ok(Some(_)) => {
                // Child exited
                print_stdout(child);
                child_list.remove(i);
            }
            _ => i += 1,
        }
    }
}

fn parse_args_list(args: &Vec<String>) -> Vec<Vec<String>> {
    let mut args_list: Vec<Vec<String>> = Vec::new();
    let mut arg_list: Vec<String> = Vec::new();

    if args.is_empty() {
        return args_list;
    }
    for arg in args {
        match arg == "," {
            false => arg_list.push(arg.clone()),
            true => {
                args_list.push(arg_list.clone());
                arg_list.clear();
            }
        }
    }
    args_list.push(arg_list.clone());
    args_list
}

fn setup_cmd_list(args_list: &Vec<Vec<String>>) -> Vec<process::Command> {
    let mut cmd_list: Vec<process::Command> = Vec::new();

    for arg_list in args_list {
        let mut cmd = process::Command::new(&arg_list[0]);
        cmd.args(&arg_list[1..])
            .stdout(process::Stdio::piped())
            .stderr(process::Stdio::null());
        cmd_list.push(cmd);
    }
    cmd_list
}

fn spawn_child_procs(cmd_list: Vec<process::Command>) -> Vec<ChildProc> {
    let mut child_list: Vec<ChildProc> = Vec::new();

    for mut cmd in cmd_list {
        if let Ok(child) = cmd.spawn() {
            child_list.push(ChildProc { cmd, proc: child });
        }
    }
    child_list
}

fn print_stdout(child: &mut ChildProc) {
    if let Some(mut output) = child.proc.stdout.take() {
        let mut buf = String::new();
        if output.read_to_string(&mut buf).is_ok() {
            println!(
                "===== {} =====\n{buf}",
                child.cmd.get_program().to_string_lossy()
            );
        }
    }
}
