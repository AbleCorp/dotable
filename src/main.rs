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
                .help("Adds a dotfile to track")
                .takes_value(true),
        )
        .get_matches();

    if matches.is_present("add") {
        println!(
            "Printing debug info... {}",
            matches.value_of("add").unwrap()
        );
    } else {
        println!("No changes detected");
    }
}
