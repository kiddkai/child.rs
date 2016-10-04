extern crate nix;

use std::{thread, time};
use nix::sys::signal;
use nix::sys::wait;
use nix::unistd::setpgid;
use nix::unistd::fork;
use nix::unistd::ForkResult::{Child, Parent};
use std::process::Command;
use std::os::unix::process::CommandExt;

extern fn handle_sigint(_:i32) {
    println!("Ctrl-C");
}

fn main() {
    let sig_action = signal::SigAction::new(signal::SigHandler::Handler(handle_sigint),
                                            signal::SaFlags::empty(),
                                            signal::SigSet::empty());
    unsafe {
        match signal::sigaction(signal::SIGINT, &sig_action) {
            Err(_) => println!("Fucked"),
            Ok(_) => println!("Registered")
        }
    }

    let pid = fork();

    match pid {
        Ok(Child) => {
            setpgid(0, 0);
            Command::new("node")
                   .arg("index.js")
                   .exec();
        },
        Ok(Parent { child }) => {
            println!("Parent: I focked a fucking child with pid {:?}, I will wait until it got killed", child);
            let wait_status = wait::waitpid(child, None);
            match wait_status {
                // assert that waitpid returned correct status and the pid is the one of the child
                Ok(wait::WaitStatus::Exited(pid_t, _)) =>  assert!(pid_t == child),
                // panic, must never happen
                Ok(_) => panic!("Child still alive, should never happen"),
                // panic, waitpid should never fail
                Err(_) => panic!("Error: waitpid Failed")
            }
        },
        Err(_) => println!("Fuck you")
    }


    //println!("{:?}", child.id());

    //let ecode = child.wait()
    //                 .expect("failed to wait child");
}

