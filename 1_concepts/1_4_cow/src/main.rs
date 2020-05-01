extern crate clap;

use clap::{Arg, App};

use std::borrow::{Cow};

fn main() {
    let matches = App::new("Nan")
        .version("1.0")
        .arg(Arg::with_name("conf")
            .long("conf")
            .value_name("FILE")
            .help("Sets a custom config file")
            .takes_value(true))
        .get_matches();

    let p = String::from("/etc/app/app.conf");
    let mut cow = Cow::from(p);
    let env_var = std::env::var("APP_CONF");
    match env_var {
        Ok(conf) => {
            let mut_cow = cow.to_mut();
            *mut_cow = conf;
        }
        Err(_) => { }
    }
    if matches.is_present("conf") {
        let cow_mut = cow.to_mut();
        *cow_mut = matches.value_of("conf").unwrap().to_owned();
    }
    println!("{}", cow.to_owned());

}
