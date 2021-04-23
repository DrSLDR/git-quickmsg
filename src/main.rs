use std::env;
use std::process::Command;

struct GitStatusElement<'a> {
    n: u16,
    items: Vec<&'a str>
}

struct GitStatus<'a> {
    modified: GitStatusElement<'a>,
    added: GitStatusElement<'a>,
    deleted: GitStatusElement<'a>,
    renamed: GitStatusElement<'a>
}

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
