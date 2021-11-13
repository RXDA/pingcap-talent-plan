#[macro_use]
extern crate clap;
use std::process::exit;

use clap::App;

fn main() {
    let yaml = load_yaml!("cli.yml");
    let m = App::from_yaml(yaml).get_matches();
    
    match m.subcommand(){
        ("set",Some(_matchs))=>{
            println!("{:#?}",_matchs);
            exit(1);
        }
        ("get",Some(_matchs))=>{
            eprintln!("unimplemented");
            exit(1);
        }
        ("rm",Some(_matchs))=>{
            eprintln!("unimplemented");
            exit(1);
        }
        _=> unreachable!()
    }
}