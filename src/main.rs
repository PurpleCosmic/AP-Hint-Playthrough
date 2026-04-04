extern crate getopts;
use directories::ProjectDirs;
use getopts::Options;
use playthrough_hinter::parser::get_seed_from_file;
use playthrough_hinter::server::get_checked_locations;
use playthrough_hinter::types::Check;
use std::env;
use std::fs;

use playthrough_hinter::{
    hint_generator::{generate_hint, read_hints, write_hint},
    parser::parse_spoiler,
};

fn main() {
    // ------- Argument handling ------ //
    let args: Vec<String> = env::args().collect();

    let mut opts = Options::new();
    opts.optopt("", "hint_file", "Set file used to store hints", "HINT_FILE");

    let matches = match opts.parse(&args[1..]) {
        Ok(m) => m,
        Err(f) => {
            panic!("{}", f.to_string())
        }
    };

    let server_url = &matches.free[0];

    let spoiler_file = &matches.free[1];

    let seed = get_seed_from_file(spoiler_file).expect("Could not read seed from spoiler file");

    let hint_file = match matches.opt_str("hint_file") {
        Some(str) => str,
        None => {
            if let Some(proj_dirs) = ProjectDirs::from("com", "Archipelago", "playthrough-hinter") {
                let data_dir = proj_dirs.data_dir();
                fs::create_dir_all(data_dir).unwrap();
                data_dir
                    .join(format!("generated_hints_{}.csv", seed))
                    .into_os_string()
                    .into_string()
                    .unwrap()
            } else {
                panic!("Coud not get hint storage file");
            }
        }
    };

    // ------- Spoiler file handling ------ //
    let spoiler_content = fs::read_to_string(spoiler_file).expect("Could not read spoiler");
    let (slots, playthrough) = parse_spoiler(&spoiler_content);

    // ------- Hint generation ------ //
    let mut ignored_checks: Vec<Check> = vec![];

    let hinted_checks = read_hints(&hint_file);
    ignored_checks.extend(hinted_checks.into_iter().map(Check::Spoiler));

    for slot in slots.iter() {
        let checks = get_checked_locations(&slot.player, server_url);
        ignored_checks.extend(checks.into_iter().map(Check::Location));
    }

    let (hint, sphere) =
        generate_hint(&playthrough, &ignored_checks).expect("Could not generate hint");

    let _ = write_hint(&hint_file, &hint);
    println!(
        "Location \"{}\" in {}'s world contains an important item! (Sphere {})",
        hint.location, hint.sender, sphere
    );
}
