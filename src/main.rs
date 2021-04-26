use std::env;
use std::fs::OpenOptions;
use std::io::prelude::*;
use std::io::SeekFrom;
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

    struct FieldName<'a>(String, &'a GitStatusElement);

    let namelist = [
        FieldName("Modified".to_string(), &stat.modified),
        FieldName("Added".to_string(), &stat.added),
        FieldName("Deleted".to_string(), &stat.deleted),
        FieldName("Renamed".to_string(), &stat.renamed),
    ];

    if sum == 1 {
        for tup in namelist.iter() {
            match tup {
                FieldName(name, list) => {
                    if list.n > 0 {
                        retstr.push_str(name.as_str());
                        retstr.push(' ');
                        if name == "Renamed" {
                            retstr.push_str(list.items[1].as_str());
                            retstr.push_str(" to ");
                        }
                        retstr.push_str(list.items[0].as_str());
                    }
                }
            }
        }
    } else if sum > 1 {
        let mut header: Vec<String> = Vec::new();
        let mut head_segment: String = "".to_string();

        for tup in namelist.iter() {
            match tup {
                FieldName(name, list) => {
                    if list.n > 0 {
                        head_segment.push_str(name.get(0..3).unwrap());
                        head_segment.push(' ');
                        head_segment.push_str(list.n.to_string().as_str());
                        header.push(head_segment.clone());
                        head_segment.clear();
                    }
                }
            }
        }

        retstr.push_str(header.join(", ").as_str());

        if sum <= 8 {
            retstr.push_str("\n");
            let mut to_name: String = "".to_string();
            for tup in namelist.iter() {
                match tup {
                    FieldName(name, list) => {
                        for item in list.items.iter() {
                            if name == "Renamed" && to_name.len() == 0 {
                                retstr.push('\n');
                                to_name = item.clone();
                                continue;
                            }
                            if to_name.len() > 0 {
                                if *item == to_name {
                                    retstr.push_str("Moved ");
                                    retstr.push_str(item);
                                } else {
                                    retstr.push_str("Renamed ");
                                    retstr.push_str(item);
                                    retstr.push_str(" => ");
                                    retstr.push_str(to_name.as_str());
                                }
                                to_name.clear();
                                continue;
                            }
                            retstr.push('\n');
                            retstr.push_str(name.as_str());
                            retstr.push(' ');
                            retstr.push_str(item);
                        }
                    }
                }
            }
        }
    }

    retstr
}

fn main() -> std::io::Result<()> {
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
        Ok(())
    } else {
        let mut pos = 0;
        let data = stat_string.as_bytes();
        let mut predata = String::new();
        let mut fh = OpenOptions::new().read(true).write(true).open(msg_option.unwrap())?;
        fh.read_to_string(&mut predata)?;

        fh.seek(SeekFrom::Start(0))?;

        fh.write_all(&data)?;
        fh.write("\n".as_bytes())?;
        fh.write_all(predata.as_bytes())?;

        Ok(())
    }
}
