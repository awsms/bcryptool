use bcrypt::{hash, verify, DEFAULT_COST};
use clap::{Arg, Command};
use std::process;

fn main() {
    let matches = Command::new("bcryptool")
        .version("1.0")
        .arg(
            Arg::new("compare")
                .short('c')
                .long("compare")
                .num_args(1)
                .value_name("HASH")
                .help("A bcrypt hash to verify against"),
        )
        .arg(
            Arg::new("string")
                .help("The string to hash or verify")
                .required(true)
                .index(1),
        )
        .get_matches();

    let input_string = matches.get_one::<String>("string").unwrap();

    if let Some(bcrypt_hash) = matches.get_one::<String>("compare") {
        match verify(input_string, bcrypt_hash) {
            Ok(is_match) => {
                if is_match {
                    println!("The hash matches the provided string.");
                    process::exit(0);
                } else {
                    println!("The hash does NOT match the provided string.");
                    process::exit(1);
                }
            }
            Err(err) => {
                eprintln!("Error verifying hash: {}", err);
                process::exit(2);
            }
        }
    } else {
        match hash(input_string, DEFAULT_COST) {
            Ok(hashed) => {
                println!("Hashed string: {}", hashed);
                process::exit(0);
            }
            Err(err) => {
                eprintln!("Error hashing string: {}", err);
                process::exit(3);
            }
        }
    }
}
