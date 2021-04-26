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
    let mut modif: GitStatusElement = GitStatusElement {
        n: 0,
        items: Vec::new(),
    };
    let mut added: GitStatusElement = GitStatusElement {
        n: 0,
        items: Vec::new(),
    };
    let mut delet: GitStatusElement = GitStatusElement {
        n: 0,
        items: Vec::new(),
    };
    let mut renam: GitStatusElement = GitStatusElement {
        n: 0,
        items: Vec::new(),
    };

    for line in stat_lns {
        let typechar = line.get(0..1).unwrap();
        let arg = line.get(3..).unwrap();
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
        .arg("--porcelain=1")
        .arg("-z")
        .output()
        .expect("Failed to call git");

    let mut stdoutvec: Vec<u8> = output.stdout;

    if stdoutvec.len() == 0 {
        println!("No output from git!");
        std::process::exit(1);
    }

    // Strip the trailing null, if it exists
    let last_byte: u8 = stdoutvec.pop().unwrap();
    if last_byte != 0u8 {
        stdoutvec.push(last_byte);
    }

    println!("{:?}", stdoutvec);

    let outstr: &str = std::str::from_utf8(&stdoutvec).unwrap();

    println!("{:?}", outstr);

    let strings: Vec<&str> = outstr.split("\u{0}").collect();

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
