use std::env;
use std::process::Command;

struct GitStatusElement<'a> {
    n: u16,
    items: Vec<&'a str>,
}

struct GitStatus<'a> {
    modified: GitStatusElement<'a>,
    added: GitStatusElement<'a>,
    deleted: GitStatusElement<'a>,
    renamed: GitStatusElement<'a>,
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
    let mut prev_rename: bool = false;

    for line in stat_lns {
        if prev_rename {
            renam
                .items
                .push(line.split("/").collect::<Vec<&str>>().pop().unwrap());
            prev_rename = false;
            continue;
        }
        let typechar = line.get(0..1).unwrap();
        let arg = line.get(3..).unwrap();
        match typechar {
            "M" => {
                modif.n += 1;
                modif
                    .items
                    .push(arg.split("/").collect::<Vec<&str>>().pop().unwrap());
            }
            "A" => {
                added.n += 1;
                added
                    .items
                    .push(arg.split("/").collect::<Vec<&str>>().pop().unwrap());
            }
            "D" => {
                delet.n += 1;
                delet
                    .items
                    .push(arg.split("/").collect::<Vec<&str>>().pop().unwrap());
            }
            "R" => {
                renam.n += 1;
                renam
                    .items
                    .push(arg.split("/").collect::<Vec<&str>>().pop().unwrap());
                prev_rename = true;
            }
            _ => continue,
        }
    }

    GitStatus {
        modified: GitStatusElement {
            n: modif.n,
            items: modif.items,
        },
        added: GitStatusElement {
            n: added.n,
            items: added.items,
        },
        deleted: GitStatusElement {
            n: delet.n,
            items: delet.items,
        },
        renamed: GitStatusElement {
            n: renam.n,
            items: renam.items,
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

    // let outstr: &str = std::str::from_utf8(&stdoutvec).unwrap();
    let outstr: &str = "R  LICENSE.bak\u{0}LICENSE\u{0}MM src/main.rs\u{0}A  foo.bar\u{0}D  bar.foo\u{0}M  bar.bar\u{0} M foo.foo";

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
