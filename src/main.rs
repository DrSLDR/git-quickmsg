use std::env;
use std::process::Command;

fn status() -> String {
    let output = Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .output()
        .expect("Failed to call git");

    let stdoutvec: Vec<u8> = output.stdout;

    println!("{:?}", stdoutvec);

    let newvec: Vec<char> = stdoutvec.into_iter().map(|x| x as char).collect();

    println!("{:?}", newvec);

    "lol".to_string()
}

fn main() {
    // Capture the argv
    let mut argv = env::args();
    argv.next();
    let msg_option: Option<String> = argv.next();
    if msg_option == None {
        println!("NO COMMIT MESSAGE FILE PASSED");
    }

    let git_status = status();
}
