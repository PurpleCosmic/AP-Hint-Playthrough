use chumsky::prelude::*;

#[derive(Debug, PartialEq)]
pub enum Expr<'src> {
    Slot(&'src str),
}

#[derive(Debug, PartialEq, Clone)]
pub struct Slot {
    player: String,
}

#[derive(Debug, PartialEq)]
pub struct Thingymabob {
    object: String,
    player: String,
}

#[derive(Debug, PartialEq)]
pub struct SpoilerEntry {
    location: String,
    item: String,
    sender: String,
    receiver: String,
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
    just("Playthrough")
        .padded()
        .then_ignore(just(":"))
        .padded()
        .then_ignore(just("\n").repeated())
        .ignore_then(
            (sphere_parser(slots).then_ignore(just("\n").repeated()))
                .repeated()
                .collect::<Vec<_>>(),
        )
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

    #[test]
    fn it_parses_slot_names() {
        assert_eq!(
            slot_parser().parse("Player 1: slot1").into_result(),
            Ok(Slot {
                player: String::from("slot1")
            })
        );
        assert_eq!(
            slot_parser().parse("Player 2: slot5").into_result(),
            Ok(Slot {
                player: String::from("slot5")
            })
        );
        assert_eq!(
            slot_parser().parse("Player 12: slot12").into_result(),
            Ok(Slot {
                player: String::from("slot12")
            })
        );
    }

    #[test]
    fn it_identifies_slots() {
        assert_eq!(
            parse_from_existing_slot(&slot_names())
                .parse("slot1")
                .into_result(),
            Ok(Slot {
                player: String::from("slot1")
            })
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
            Ok(SpoilerEntry {
                location: String::from("Server"),
                item: String::from("item"),
                sender: String::from("Archipelago"),
                receiver: String::from("slot2")
            })
        );

        assert_eq!(
            check_parser(&slot_names())
                .parse("item (sub-name) (slot2)")
                .into_result(),
            Ok(SpoilerEntry {
                location: String::from("Server"),
                item: String::from("item (sub-name)"),
                sender: String::from("Archipelago"),
                receiver: String::from("slot2")
            })
        );
    }

    #[test]
    fn it_parses_checks() {
        assert_eq!(
            check_parser(&slot_names())
                .parse("loc (slot1): item (slot2)")
                .into_result(),
            Ok(SpoilerEntry {
                location: String::from("loc"),
                item: String::from("item"),
                sender: String::from("slot1"),
                receiver: String::from("slot2")
            })
        );

        assert_eq!(
            check_parser(&slot_names())
                .parse("loc (sub-loc) (slot1): item (sub-item) (slot2)")
                .into_result(),
            Ok(SpoilerEntry {
                location: String::from("loc (sub-loc)"),
                item: String::from("item (sub-item)"),
                sender: String::from("slot1"),
                receiver: String::from("slot2")
            })
        );

        assert_eq!(
            check_parser(&slot_names())
                .parse(
                    "AF (Depths): Upper Right Chest (1st Reward) (slot1): Gate: item (sub-item) (slot2)"
                )
                .into_result(),
            Ok(SpoilerEntry {
                location: String::from("AF (Depths): Upper Right Chest (1st Reward)"),
                item: String::from("Gate: item (sub-item)"),
                sender: String::from("slot1"),
                receiver: String::from("slot2")
            })
        );
    }

    #[test]
    fn it_parses_a_sphere() {
        assert_eq!(
            sphere_parser(&slot_names())
                .parse("1: {\n  loc1 (slot1): item1 (slot2)\n  loc2 (slot1): item2 (slot2)\n}")
                .into_result(),
            Ok(vec![
                SpoilerEntry {
                    location: String::from("loc1"),
                    sender: String::from("slot1"),
                    item: String::from("item1"),
                    receiver: String::from("slot2"),
                },
                SpoilerEntry {
                    location: String::from("loc2"),
                    sender: String::from("slot1"),
                    item: String::from("item2"),
                    receiver: String::from("slot2"),
                },
            ])
        );
    }

    #[test]
    fn it_parses_playthroughs() {
        assert_eq!(
            playthrough_parser(&slot_names())
                .parse("Playthrough:\n\n\n1: {\n  loc1 (slot1): item1 (slot2)\n  loc2 (slot1): item2 (slot2)\n}")
                .into_result(),
            Ok(vec![vec![
                SpoilerEntry {
                    location: String::from("loc1"),
                    sender: String::from("slot1"),
                    item: String::from("item1"),
                    receiver: String::from("slot2"),
                },
                SpoilerEntry {
                    location: String::from("loc2"),
                    sender: String::from("slot1"),
                    item: String::from("item2"),
                    receiver: String::from("slot2"),
                },
            ]])
        );
        assert_eq!(
            playthrough_parser(&slot_names())
                .parse(
                    "Playthrough: \
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
                    }"
                )
                .into_result(),
            Ok(vec![
                vec![
                    SpoilerEntry {
                        location: String::from("Server"),
                        sender: String::from("Archipelago"),
                        item: String::from("item1"),
                        receiver: String::from("slot2"),
                    },
                    SpoilerEntry {
                        location: String::from("Server"),
                        sender: String::from("Archipelago"),
                        item: String::from("item2"),
                        receiver: String::from("slot2"),
                    },
                ],
                vec![
                    SpoilerEntry {
                        location: String::from("loc1"),
                        sender: String::from("slot1"),
                        item: String::from("item3"),
                        receiver: String::from("slot2"),
                    },
                    SpoilerEntry {
                        location: String::from("loc2"),
                        sender: String::from("slot2"),
                        item: String::from("item4"),
                        receiver: String::from("slot1"),
                    },
                ]
            ])
        );
    }
}
