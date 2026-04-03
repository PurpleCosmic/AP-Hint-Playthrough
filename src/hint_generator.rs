use std::{
    fs::{self, OpenOptions},
    io::Write,
};

use crate::parser::{Playthrough, SpoilerEntry, stored_hint_parser};
use chumsky::prelude::*;
use rand::prelude::*;

pub fn generate_hint(
    playthrough: &Playthrough,
    collected_checks: &Vec<SpoilerEntry>,
    hinted_checks: &Vec<SpoilerEntry>,
) -> Option<(SpoilerEntry, usize)> {
    for (i, sphere) in playthrough.iter().enumerate() {
        let filtered_checks = sphere
            .iter()
            .filter(|entrya| !collected_checks.contains(entrya) && !hinted_checks.contains(entrya));
        let res = filtered_checks.choose(&mut rand::rng());
        match res {
            Some(val) => return Some((val.clone(), i)),
            None => continue,
        };
    }
    None
}

pub fn write_hint(file_path: &String, hint: &SpoilerEntry) -> Result<(), std::io::Error> {
    let mut file = OpenOptions::new()
        .append(true)
        .create(true)
        .open(file_path)
        .expect("Unable to open file");

    let as_str = hint.location.clone()
        + ";"
        + hint.item.as_str()
        + ";"
        + hint.sender.as_str()
        + ";"
        + hint.receiver.as_str()
        + "\n";
    file.write_all(as_str.as_bytes())
}

pub fn read_hints(hint_file: &String) -> Vec<SpoilerEntry> {
    let f = match fs::read_to_string(hint_file) {
        Ok(s) => s,
        Err(_) => "".to_string(),
    };
    stored_hint_parser()
        .parse(&f)
        .into_result()
        .expect("Could not parse hint file")
}

#[cfg(test)]
mod hint_tests {
    use super::*;

    #[test]
    fn it_generates_hint() {
        let entrya = SpoilerEntry {
            location: "loc1".to_string(),
            sender: "send1".to_string(),
            item: "item1".to_string(),
            receiver: "recv1".to_string(),
        };
        let entryb = SpoilerEntry {
            location: "loc2".to_string(),
            sender: "send2".to_string(),
            item: "item2".to_string(),
            receiver: "recv2".to_string(),
        };
        let playthrough = vec![vec![entrya.clone(), entryb.clone()]];

        assert!(generate_hint(&playthrough, &vec![], &vec![]).is_some());
        assert_eq!(
            generate_hint(&playthrough, &vec![entrya.clone()], &vec![]),
            Some((entryb.clone(), 0))
        );
        assert_eq!(
            generate_hint(&playthrough.clone(), &vec![], &vec![entryb.clone()]),
            Some((entrya.clone(), 0))
        );
        assert_eq!(
            generate_hint(
                &playthrough.clone(),
                &vec![entrya.clone()],
                &vec![entryb.clone()]
            ),
            None
        );
    }
}
