use archipelago_rs::{self as ap, tags};

use crate::types::Location;

fn connect(url: &str, slot: &str) -> Result<ap::Connection<()>, ap::Error> {
    let mut connection = ap::Connection::<()>::new(
        url,
        slot,
        None::<String>,
        ap::ConnectionOptions::new().tags(vec![tags::NO_TEXT, tags::TRACKER, tags::AP]),
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

pub fn get_checks(slot: &str, url: &str) -> Vec<Location> {
    let mut connection = connect(url, slot).expect("Failed to connect to archipelago server");

    match connection.state_mut() {
        ap::ConnectionState::Connected(client) => client
            .unchecked_locations()
            .map(|location| Location {
                location: location.name().as_str().to_string(),
                player: slot.to_string(),
            })
            .collect::<Vec<_>>(),
        _ => panic!("Not connected!!!"),
    }
}
