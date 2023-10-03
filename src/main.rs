extern crate regex;
mod spanish;
mod multi_lang;
mod entities;
mod icons;
mod read_line;
use std::env;
mod tests;
mod thisis;
use crate::tests::run_tests;
use crate::multi_lang::Lang;
use crate::multi_lang::print_update_at;
use crate::multi_lang::print_type_exit_to_exit;

fn main() {
    const VERSION: &str = env!("CARGO_PKG_VERSION");
    const APPNAME: &str = env!("CARGO_PKG_NAME");
    let mut lang: Lang = Lang::Spanish;
    let args: Vec<String> = env::args().collect();
    if args.len() > 1 {
        if args[1].eq("french") {
            lang=Lang::French;
        } else if args[1].eq("german") {
            lang=Lang::German;
        }
    }
    
    println!("############################");
    println!("#");
    println!("# {} v.{}", APPNAME, VERSION);
    print_update_at("2023-09-17", lang);
    println!("#");
    println!("############################");
    println!("");
    print_type_exit_to_exit(lang);
    run_tests(lang)
}
