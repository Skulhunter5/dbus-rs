use std::path::Path;

use dbus::{Connection, PRINT};

fn main() {
    let dbus_session_bus_address =
        std::env::var("DBUS_SESSION_BUS_ADDRESS").expect("DBUS_SESSION_BUS_ADDRESS is not set");
    let path = if dbus_session_bus_address.starts_with("unix:path=") {
        let (_, path) = dbus_session_bus_address.split_once('=').unwrap();
        Path::new(path)
    } else {
        panic!("unknown format for DBUS_SESSION_BUS_ADDRESS");
    };
    let mut connection =
        Connection::init(path).expect("failed to initialize connection to dbus session bus");

    if crate::PRINT {
        println!("Connection established to {}", connection.server_guid());
    }

    let message = connection.read_message().unwrap();
    println!("message: {:?}", &message);
}
