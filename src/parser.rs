use chumsky::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub struct Slot {
    pub player: String,
}

#[derive(Debug, PartialEq)]
struct Thingymabob {
    object: String,
    player: String,
}

#[derive(Debug, PartialEq)]
pub struct SpoilerEntry {
    pub location: String,
    pub item: String,
    pub sender: String,
    pub receiver: String,
}

type Sphere = Vec<SpoilerEntry>;

pub fn slot_parser<'src>() -> impl Parser<'src, &'src str, Slot> {
    just("Player ")
        .ignore_then(text::int(10).repeated())
        .ignore_then(just(":"))
        .padded()
        .ignore_then(text::ascii::ident())
        .map(|s: &str| Slot {
            player: String::from(s),
        })
}

pub fn slots_parser<'src>() -> impl Parser<'src, &'src str, Vec<Slot>> {
    any()
        .and_is(slot_parser().not())
        .repeated()
        .ignore_then(slot_parser())
        .repeated()
        .collect::<Vec<_>>()
        .then_ignore(any().repeated())
}

fn parse_from_existing_slot<'src>(slots: &Vec<Slot>) -> impl Parser<'src, &'src str, Slot> {
    choice(
        slots
            .iter()
            .cloned()
            .map(|slot| {
                let name = slot.player.clone();
                just(name).to(slot)
            })
            .collect::<Vec<_>>(),
    )
}

fn object_parser<'src>(slots: &Vec<Slot>) -> impl Parser<'src, &'src str, Thingymabob> {
    let delimited_slot_parser =
        || parse_from_existing_slot(slots).delimited_by(just("("), just(")"));
    let location = none_of("{}")
        .and_is(delimited_slot_parser().not())
        .repeated()
        .collect::<String>();

    location
        .then(delimited_slot_parser())
        .map(|(location, slot)| Thingymabob {
            object: String::from(location.trim()),
            player: slot.player.clone(),
        })
}

fn check_parser<'src>(slots: &Vec<Slot>) -> impl Parser<'src, &'src str, SpoilerEntry> {
    let full_check_parser = object_parser(slots)
        .padded()
        .then_ignore(just(":"))
        .padded()
        .then(object_parser(slots))
        .map(|(send, recv)| SpoilerEntry {
            location: send.object,
            item: recv.object,
            sender: send.player,
            receiver: recv.player,
        });

    choice((
        full_check_parser,
        object_parser(slots).map(|recv| SpoilerEntry {
            location: String::from("Server"),
            item: recv.object,
            sender: String::from("Archipelago"),
            receiver: recv.player,
        }),
    ))
}

fn sphere_parser<'src>(slots: &Vec<Slot>) -> impl Parser<'src, &'src str, Sphere> {
    text::int(10).then_ignore(just(":")).padded().ignore_then(
        (just("\n")
            .repeated()
            .padded()
            .ignore_then(check_parser(slots))
            .then_ignore(just("\n").repeated())
            .padded())
        .repeated()
        .collect::<Sphere>()
        .delimited_by(just("{"), just("}")),
    )
}

pub fn playthrough_parser<'src>(slots: &Vec<Slot>) -> impl Parser<'src, &'src str, Vec<Sphere>> {
    any()
        .and_is(just("Playthrough").not())
        .repeated()
        .ignore_then(
            just("Playthrough")
                .padded()
                .then_ignore(just(":"))
                .padded()
                .then_ignore(just("\n").repeated())
                .ignore_then(
                    (sphere_parser(slots).then_ignore(just("\n").repeated()))
                        .repeated()
                        .collect::<Vec<_>>(),
                ),
        )
        .then_ignore(any().repeated())
}

pub fn parse_spoiler(input: &String) -> (Vec<Slot>, Vec<Sphere>) {
    let slots = slots_parser()
        .parse(&input)
        .into_result()
        .expect("Invalid Slots");

    let playthrough = playthrough_parser(&slots)
        .parse(&input)
        .into_result()
        .expect("Invalid Playthrough");

    (slots, playthrough)
}

#[cfg(test)]
mod test_parser {
    use super::*;
    fn slot_names() -> Vec<Slot> {
        vec![
            Slot {
                player: String::from("slot1"),
            },
            Slot {
                player: String::from("slot2"),
            },
        ]
    }
    fn slot(s: &str) -> Slot {
        Slot {
            player: String::from(s),
        }
    }
    fn spoiler_entry(loc: &str, sender: &str, item: &str, receiver: &str) -> SpoilerEntry {
        SpoilerEntry {
            location: String::from(loc),
            item: String::from(item),
            sender: String::from(sender),
            receiver: String::from(receiver),
        }
    }

    #[test]
    fn it_parses_slot_names() {
        assert_eq!(
            slot_parser().parse("Player 1: slot1").into_result(),
            Ok(slot("slot1"))
        );
        assert_eq!(
            slot_parser().parse("Player 2: slot5").into_result(),
            Ok(slot("slot5"))
        );
        assert_eq!(
            slot_parser().parse("Player 12: slot12").into_result(),
            Ok(slot("slot12"))
        );
    }

    #[test]
    fn it_parses_a_sequence_of_slot_names() {
        assert_eq!(
            // Test if it consumes extra end input
            slots_parser()
                .parse("Player 1: slot1\nblebleble\nblo\nPlayer 2: slot2\nPlayer 3: slot13\n")
                .into_result(),
            Ok(vec![slot("slot1"), slot("slot2"), slot("slot13")])
        );
        assert_eq!(
            // Test if it ignores start input
            slots_parser()
                .parse("blabla\nPlayer 1: slot1\nblebleble\nblo\nPlayer 2: slot2\nPlayer 3: slot13")
                .into_result(),
            Ok(vec![slot("slot1"), slot("slot2"), slot("slot13")])
        );
    }

    #[test]
    fn it_identifies_slots() {
        assert_eq!(
            parse_from_existing_slot(&slot_names())
                .parse("slot1")
                .into_result(),
            Ok(slot("slot1"))
        );
    }

    #[test]
    fn it_parses_objects() {
        assert_eq!(
            object_parser(&slot_names())
                .parse("Location (slot1)")
                .into_result(),
            Ok(Thingymabob {
                object: String::from("Location"),
                player: String::from("slot1")
            })
        );
    }

    #[test]
    fn it_parses_sublocations_properly() {
        assert_eq!(
            object_parser(&slot_names())
                .parse("Location (sub-location) (slot1)")
                .into_result(),
            Ok(Thingymabob {
                object: String::from("Location (sub-location)"),
                player: String::from("slot1")
            })
        );
    }

    #[test]
    fn it_parses_starting_checks() {
        assert_eq!(
            check_parser(&slot_names())
                .parse("item (slot2)")
                .into_result(),
            Ok(spoiler_entry("Server", "Archipelago", "item", "slot2"))
        );

        assert_eq!(
            check_parser(&slot_names())
                .parse("item (sub-name) (slot2)")
                .into_result(),
            Ok(spoiler_entry(
                "Server",
                "Archipelago",
                "item (sub-name)",
                "slot2"
            ))
        );
    }

    #[test]
    fn it_parses_checks() {
        assert_eq!(
            check_parser(&slot_names())
                .parse("loc (slot1): item (slot2)")
                .into_result(),
            Ok(spoiler_entry("loc", "slot1", "item", "slot2"))
        );

        assert_eq!(
            check_parser(&slot_names())
                .parse("loc (sub-loc) (slot1): item (sub-item) (slot2)")
                .into_result(),
            Ok(spoiler_entry(
                "loc (sub-loc)",
                "slot1",
                "item (sub-item)",
                "slot2"
            ))
        );

        assert_eq!(
            check_parser(&slot_names())
                .parse(
                    "AF (Depths): Upper Right Chest (1st Reward) (slot1): Gate: item (sub-item) (slot2)"
                )
                .into_result(),
            Ok(spoiler_entry("AF (Depths): Upper Right Chest (1st Reward)", "slot1", "Gate: item (sub-item)", "slot2"))
        );
    }

    #[test]
    fn it_parses_a_sphere() {
        assert_eq!(
            sphere_parser(&slot_names())
                .parse("1: {\n  loc1 (slot1): item1 (slot2)\n  loc2 (slot1): item2 (slot2)\n}")
                .into_result(),
            Ok(vec![
                spoiler_entry("loc1", "slot1", "item1", "slot2"),
                spoiler_entry("loc2", "slot1", "item2", "slot2")
            ])
        );
    }

    #[test]
    fn it_parses_playthroughs() {
        assert_eq!(
            playthrough_parser(&slot_names())
                .parse("Playthrough:\n\n\n1: {\n  loc1 (slot1): item1 (slot2)\n  loc2 (slot1): item2 (slot2)\n}\n")
                .into_result(),
            Ok(vec![vec![
                spoiler_entry("loc1", "slot1", "item1", "slot2"),
                spoiler_entry("loc2", "slot1", "item2", "slot2")
            ]])
        );
        assert_eq!(
            playthrough_parser(&slot_names())
                .parse(
                    "\nPlaythrough: \
                    \
                    \
                    \
                    0: {\
                        item1 (slot2)\
                        item2 (slot2)\
                    }\
                    \
                    \
                    1: {\
                        loc1 (slot1): item3 (slot2)\
                        loc2 (slot2): item4 (slot1)\
                    }\n"
                )
                .into_result(),
            Ok(vec![
                vec![
                    spoiler_entry("Server", "Archipelago", "item1", "slot2"),
                    spoiler_entry("Server", "Archipelago", "item2", "slot2")
                ],
                vec![
                    spoiler_entry("loc1", "slot1", "item3", "slot2"),
                    spoiler_entry("loc2", "slot2", "item4", "slot1")
                ]
            ])
        );
    }
}
