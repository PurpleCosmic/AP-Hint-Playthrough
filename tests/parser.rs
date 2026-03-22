use playthrough_hinter::parser::*;
use std::fs;

#[test]
fn parse_spoiler_test() {
    let spoiler_text =
        fs::read_to_string("./tests/dummy_spoiler.txt").expect("Could not read file");

    assert_eq!(
        parse_spoiler(&spoiler_text),
        (
            vec![
                Slot {
                    player: String::from("slot1")
                },
                Slot {
                    player: String::from("slot2")
                },
                Slot {
                    player: String::from("hi_I_have_name")
                }
            ],
            vec![
                vec![
                    SpoilerEntry {
                        location: String::from("Server"),
                        sender: String::from("Archipelago"),
                        item: String::from("Anakin Skywalker (Jedi)"),
                        receiver: String::from("slot1")
                    },
                    SpoilerEntry {
                        location: String::from("Server"),
                        sender: String::from("Archipelago"),
                        item: String::from("Blelt"),
                        receiver: String::from("slot2")
                    },
                ],
                vec![
                    SpoilerEntry {
                        location: String::from("Farm Arrays - Echo"),
                        sender: String::from("slot1"),
                        item: String::from("Shrek's Swamp Unlock"),
                        receiver: String::from("slot2")
                    },
                    SpoilerEntry {
                        location: String::from("Beat Meg"),
                        sender: String::from("hi_I_have_name"),
                        item: String::from("Meg Victory"),
                        receiver: String::from("hi_I_have_name")
                    },
                    SpoilerEntry {
                        location: String::from("Clear Score 0001"),
                        sender: String::from("slot1"),
                        item: String::from("Upslash"),
                        receiver: String::from("slot2")
                    }
                ],
                vec![SpoilerEntry {
                    location: String::from("AF (Depths): Upper Right Chest (1st Reward)"),
                    sender: String::from("slot1"),
                    item: String::from("Gate: Outskirts to Drainage System"),
                    receiver: String::from("slot2")
                }],
                vec![SpoilerEntry {
                    location: String::from("Goal"),
                    sender: String::from("slot1"),
                    item: String::from("Victory"),
                    receiver: String::from("slot1")
                }]
            ]
        )
    );
}
