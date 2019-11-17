extern crate clap;
use std::process;
use clap::{Arg, App, SubCommand};


fn main() {
    //println!("Hello, world!")
    let matches = App::new("kvs")
            .version(env!("CARGO_PKG_VERSION"))
            .about(env!("CARGO_PKG_DESCRIPTION"))
            .author(env!("CARGO_PKG_AUTHORS"))
            .subcommand( // kvs set <KEY> <VALUE>
                SubCommand::with_name("set")
                    .about("Set the value of a string key to a string")
                    .arg(Arg::with_name("KEY"))
                    .arg(Arg::with_name("VALUE"))         
            )
            .subcommand( // kvs get <KEY>
                SubCommand::with_name("get")
                    .about("Get the string value of a given key")
                    .arg(Arg::with_name("KEY"))         
            )
            .subcommand( // kvs rm <KEY>
                SubCommand::with_name("rm")
                    .about("Remove a given key")
                    .arg(Arg::with_name("KEY"))         
            )  
            
            .get_matches();
    
    match matches.subcommand_name() {
        Some("set") => {
            eprintln!("unimplemented");
            process::exit(1);
        },
        Some("get") => {
            eprintln!("unimplemented");
            process::exit(1);
        },
        Some("rm") => {
            eprintln!("unimplemented");
            process::exit(1);
        },
        None        => {
            //eprintln!("unimplemented");
            process::exit(1);
        },
        _           => {
            //eprintln!("unimplemented");
            process::exit(1);
        } 
           
    } 
    
}

