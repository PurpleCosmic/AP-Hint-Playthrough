extern crate getopts;
use directories::ProjectDirs;
use getopts::Options;
use std::env;
use std::fs;

use playthrough_hinter::{
    hint_generator::{generate_hint, read_hints, write_hint},
    parser::parse_spoiler,
};

fn main() {
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt("", "hint_file", "Set file used to store hints", "HINT_FILE");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!("{}", f.to_string())
        }
    };

    let _server_url = &matches.free[0];

    let spoiler_file = &matches.free[1];
    let hint_file = match matches.opt_str("hint_file") {
        Some(str) => str,
        None => {
            if let Some(proj_dirs) = ProjectDirs::from("com", "Archipelago", "playthrough-hinter") {
                let data_dir = proj_dirs.data_dir();
                fs::create_dir_all(data_dir).unwrap();
                data_dir
                    .join("generated_hints.csv")
                    .into_os_string()
                    .into_string()
                    .unwrap()
            } else {
                panic!("Coud not hint storage file");
            }
        }
    };

    let spoiler_content = fs::read_to_string(spoiler_file).expect("Could not read spoiler");
    let (_slots, playthrough) = parse_spoiler(&spoiler_content);

    let mut hinted_checks = read_hints(&hint_file);
    let (hint, sphere) =
        generate_hint(&playthrough, &vec![], &hinted_checks).expect("Could not generate hint");
    hinted_checks.push(hint.clone());

    let _ = write_hint(&hint_file, &hint);
    println!(
        "Location \"{}\" in {}'s world contains an important item! (Sphere {})",
        hint.location, hint.sender, sphere
    );
}
