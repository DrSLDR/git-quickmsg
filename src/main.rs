use std::env;
use std::process::Command;

struct GitStatusElement {
    n: u16,
    items: Vec<String>,
}

struct GitStatus {
    modified: GitStatusElement,
    added: GitStatusElement,
    deleted: GitStatusElement,
    renamed: GitStatusElement,
}

fn parse_status(stat_lns: Vec<String>) -> GitStatus {
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
                .push(line.split("/").collect::<Vec<&str>>().pop().unwrap().to_string());
            prev_rename = false;
            continue;
        }
        let typechar = line.get(0..1).unwrap();
        let arg = line.get(3..).unwrap().to_string();
        match typechar {
            "M" => {
                modif.n += 1;
                modif
                    .items
                    .push(arg.split("/").collect::<Vec<&str>>().pop().unwrap().to_string());
            }
            "A" => {
                added.n += 1;
                added
                    .items
                    .push(arg.split("/").collect::<Vec<&str>>().pop().unwrap().to_string());
            }
            "D" => {
                delet.n += 1;
                delet
                    .items
                    .push(arg.split("/").collect::<Vec<&str>>().pop().unwrap().to_string());
            }
            "R" => {
                renam.n += 1;
                renam
                    .items
                    .push(arg.split("/").collect::<Vec<&str>>().pop().unwrap().to_string());
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

fn status() -> GitStatus {
    let output = Command::new("git")
        .arg("status")
        .arg("--porcelain=1")
        .arg("-z")
        .output()
        .expect("Failed to call git");

    let mut stdoutvec: Vec<u8> = output.stdout;

    if cfg!(debug_assertions) {
        stdoutvec = b"R  LICENSE.bak\0LICENSE\0MM src/main.rs\0A  foo.bar\0D  bar.foo\0M  bar.bar\0 M foo.foo\0R  path/path/path/foobar\0path/foobar".to_vec();
    }

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

    let status_obj = parse_status(strings.into_iter().map(|x| String::from(x)).collect());

    println!("mod: {}:{:?}\nadd: {}:{:?}\ndel: {}:{:?}\nren: {}:{:?}", status_obj.modified.n, status_obj.modified.items, status_obj.added.n, status_obj.added.items, status_obj.deleted.n, status_obj.deleted.items, status_obj.renamed.n, status_obj.renamed.items);

    status_obj
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
