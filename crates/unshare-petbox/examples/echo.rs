extern crate unshare_petbox;

use std::process::exit;


fn main() {
    let mut cmd = unshare_petbox::Command::new("/bin/echo");
    cmd.arg("hello");
    cmd.arg("world!");

    match cmd.status().unwrap() {
        // propagate signal
        unshare_petbox::ExitStatus::Exited(x) => exit(x as i32),
        unshare_petbox::ExitStatus::Signaled(x, _) => exit((128+x as i32) as i32),
    }
}
