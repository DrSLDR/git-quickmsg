use std::env;
use std::process::Command;

struct GitStatusElement {
    n: u32,
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
            renam.items.push(
                line.split("/")
                    .collect::<Vec<&str>>()
                    .pop()
                    .unwrap()
                    .to_string(),
            );
            prev_rename = false;
            continue;
        }
        let typechar = line.get(0..1).unwrap();
        let arg = line.get(3..).unwrap().to_string();
        match typechar {
            "M" => {
                modif.n += 1;
                modif.items.push(
                    arg.split("/")
                        .collect::<Vec<&str>>()
                        .pop()
                        .unwrap()
                        .to_string(),
                );
            }
            "A" => {
                added.n += 1;
                added.items.push(
                    arg.split("/")
                        .collect::<Vec<&str>>()
                        .pop()
                        .unwrap()
                        .to_string(),
                );
            }
            "D" => {
                delet.n += 1;
                delet.items.push(
                    arg.split("/")
                        .collect::<Vec<&str>>()
                        .pop()
                        .unwrap()
                        .to_string(),
                );
            }
            "R" => {
                renam.n += 1;
                renam.items.push(
                    arg.split("/")
                        .collect::<Vec<&str>>()
                        .pop()
                        .unwrap()
                        .to_string(),
                );
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

    println!(
        "mod: {}:{:?}\nadd: {}:{:?}\ndel: {}:{:?}\nren: {}:{:?}",
        status_obj.modified.n,
        status_obj.modified.items,
        status_obj.added.n,
        status_obj.added.items,
        status_obj.deleted.n,
        status_obj.deleted.items,
        status_obj.renamed.n,
        status_obj.renamed.items
    );

    status_obj
}

fn render_status(stat: GitStatus) -> String {
    let sum: u32 = stat.modified.n + stat.added.n + stat.deleted.n + stat.renamed.n;
    let mut retstr: String = "".to_string();

    if sum == 1 {
        if stat.modified.n > 0 {
            retstr.push_str("Modified ");
            retstr.push_str(stat.modified.items[0].as_str());
        } else if stat.added.n > 0 {
            retstr.push_str("Added ");
            retstr.push_str(stat.added.items[0].as_str());
        } else if stat.deleted.n > 0 {
            retstr.push_str("Deleted ");
            retstr.push_str(stat.deleted.items[0].as_str());
        } else if stat.renamed.n > 0 {
            retstr.push_str("Renamed ");
            retstr.push_str(stat.renamed.items[1].as_str());
            retstr.push_str(" to ");
            retstr.push_str(stat.renamed.items[0].as_str());
        }
    } else if sum > 1 {
        let mut header: Vec<String> = Vec::new();
        let mut head_segment: String = "".to_string();
        if stat.modified.n > 0 {
            head_segment.push_str("Mod ");
            head_segment.push_str(stat.modified.n.to_string().as_str());
            header.push(head_segment.clone());
            head_segment.clear();
        }
        if stat.added.n > 0 {
            head_segment.push_str("Add ");
            head_segment.push_str(stat.added.n.to_string().as_str());
            header.push(head_segment.clone());
            head_segment.clear();
        }
        if stat.deleted.n > 0 {
            head_segment.push_str("Del ");
            head_segment.push_str(stat.deleted.n.to_string().as_str());
            header.push(head_segment.clone());
            head_segment.clear();
        }
        if stat.renamed.n > 0 {
            head_segment.push_str("Ren ");
            head_segment.push_str(stat.renamed.n.to_string().as_str());
            header.push(head_segment.clone());
            head_segment.clear();
        }

        retstr.push_str(header.join(", ").as_str());
    }

    retstr
}

fn main() {
    // Capture the argv
    let mut argv = env::args();
    argv.next();
    let msg_option: Option<String> = argv.next();

    let git_status = status();
    let mut stat_string = render_status(git_status);

    if stat_string.len() == 0 {
        std::process::exit(2);
    }

    if msg_option == None {
        stat_string.push_str("\n\nQuick-committed");
        println!("{}", stat_string);
    } else {
        // Do some dandy write-to-file shit
    }
}
