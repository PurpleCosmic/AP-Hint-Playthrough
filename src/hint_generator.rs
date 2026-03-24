use crate::parser::{Playthrough, SpoilerEntry};
use rand::prelude::*;

pub fn generate_hint(
    playthrough: Playthrough,
    collected_checks: Vec<SpoilerEntry>,
    hinted_checks: Vec<SpoilerEntry>,
) -> Option<SpoilerEntry> {
    for sphere in playthrough {
        let filtered_checks = sphere
            .iter()
            .filter(|entrya| !collected_checks.contains(entrya) && !hinted_checks.contains(entrya));
        let res = filtered_checks.choose(&mut rand::rng());
        match res {
            // I think... I should be using references to SpoilerEntries...
            // instead of full on spoilerentries...
            Some(val) => return Some(val.clone()),
            None => continue,
        };
    }
    None
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

        assert!(generate_hint(playthrough.clone(), vec![], vec![]).is_some());
        assert_eq!(
            generate_hint(playthrough.clone(), vec![entrya.clone()], vec![]),
            Some(entryb.clone())
        );
        assert_eq!(
            generate_hint(playthrough.clone(), vec![], vec![entryb.clone()]),
            Some(entrya.clone())
        );
        assert_eq!(
            generate_hint(
                playthrough.clone(),
                vec![entrya.clone()],
                vec![entryb.clone()]
            ),
            None
        );
        assert!(false);
    }
}
