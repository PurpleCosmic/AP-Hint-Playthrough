use archipelago_rs::{self as ap, CreateAsHint, tags};

use crate::types::{Location, SpoilerEntry};

fn connect(url: &str, slot: &str) -> Result<ap::Connection<()>, ap::Error> {
    let mut connection = ap::Connection::<()>::new(
        url,
        slot,
        None::<String>,
        ap::ConnectionOptions::new().tags(vec![
            tags::TRACKER,
            tags::AP,
            tags::NO_TEXT,
            "APPlaythroughHinter",
        ]),
    );
    while connection.is_connecting() {
        connection.update();
        for event in connection.update() {
            if let ap::Event::Print(print) = event {
                println!("{}", print);
            }
        }
    }
    if connection.is_connected() {
        Ok(connection)
    } else {
        Err(connection.into_err())
    }
}

pub fn get_checked_locations(slot: &str, url: &str) -> Vec<Location> {
    let mut connection = connect(url, slot).expect("Failed to connect to archipelago server");

    match connection.state_mut() {
        ap::ConnectionState::Connected(client) => client
            .checked_locations()
            .map(|location| Location {
                location: location.name().as_str().to_string(),
                player: slot.to_string(),
                id: location.id(),
            })
            .collect::<Vec<_>>(),
        _ => panic!("Not connected!!!"),
    }
}

pub fn hint_spoiler_entry(url: &str, entry: &SpoilerEntry) {
    let mut connection = connect(url, &entry.sender)
        .expect("Failed to connect to archipelago server while trying to hint location");

    match connection.state_mut() {
        ap::ConnectionState::Connected(client) => {
            let check = {
                let mut c = client.unchecked_locations();
                c.find(|loc| loc.name() == entry.location)
                    .clone()
                    .expect("Could not find location in player's locations")
            };

            client.scout_locations(vec![check], CreateAsHint::All);
        }
        _ => {
            panic!("Not connected!!!")
        }
    }
}
