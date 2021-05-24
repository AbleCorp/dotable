extern crate clap;
use clap::{App, Arg};

fn main() {
    let matches = App::new("DotAble")
        .version(env!("CARGO_PKG_VERSION"))
        .author("Able <abl3theabove@gmail.com>")
        .about("Helpfully manages dot files")
        .arg(
            Arg::with_name("add")
                .short("a")
                .long("add")
                .value_name("FILE")
                .help("Adds a dotfile to track.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("export")
                .short("e")
                .long("export")
                .value_name("FILE")
                .help("Exports all dotfiles to an archive.")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("import")
                .short("i")
                .long("import")
                .value_name("FILE")
                .help("Imports all dotfiles from an archive.")
                .takes_value(true),
        )
        .get_matches();

    if matches.is_present("add") {
        println!(
            "Printing debug info... {}",
            matches.value_of("add").unwrap()
        );
    } else {
        println!("No dotfiles added.");
    }
    if matches.is_present("export") {
        println!("Exporting to... {}", matches.value_of("export").unwrap());
    } else {
        println!("No exports requested");
    }
    if matches.is_present("import") {
        println!("Importing from... {}", matches.value_of("export").unwrap());
    } else {
        println!("No imports requested");
    }
}
