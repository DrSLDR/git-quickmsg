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

fn parse_status(stat_lns: Vec<&str>) -> GitStatus {
    let mut modif: GitStatusElement = GitStatusElement {n: 0, items: vec![]};
    let mut added: GitStatusElement = GitStatusElement {n: 0, items: vec![]};
    let mut delet: GitStatusElement = GitStatusElement {n: 0, items: vec![]};
    let mut renam: GitStatusElement = GitStatusElement {n: 0, items: vec![]};

    for line in stat_lns {
        let typechar = line.get(0..1).unwrap();
        let arg = line.get(3..).unwrap();
        println!("{}-{}", typechar, arg);
        match typechar {
            "M" => continue,
            _   => continue,
        }
    }

    GitStatus {
        modified: GitStatusElement {
            n: 0,
            items: vec!()
        },
        added: GitStatusElement {
            n: 0,
            items: vec!()
        },
        deleted: GitStatusElement {
            n: 0,
            items: vec!()
        },
        renamed: GitStatusElement {
            n: 0,
            items: vec!()
        },
    }
}

fn status() -> String {
    let output = Command::new("git")
        .arg("status")
        .arg("--porcelain")
        .output()
        .expect("Failed to call git");

    let mut stdoutvec: Vec<u8> = output.stdout;

    if stdoutvec.len() == 0 {
        println!("No output from git!");
        std::process::exit(1);
    }

    let last_byte: u8 = stdoutvec.pop().unwrap();
    if last_byte != 10u8 {
        stdoutvec.push(last_byte);
    }

    println!("{:?}", stdoutvec);

    let outstr: String = stdoutvec.into_iter().map(|x| x as char).collect();

    println!("{:?}", outstr);

    let strings: Vec<&str> = outstr.split("\n").collect();

    for s in strings.clone() {
        println!("{}", s);
    }

    let status_obj = parse_status(strings);

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
