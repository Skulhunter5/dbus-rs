use std::path::Path;

use dbus::{
    Connection, InterfaceName, MemberName,
    message::{Endianness, Flags, HeaderField, MajorProtocolVersion, Message, MessageType},
    types::ObjectPath,
};

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

    println!("Connection established to {}", connection.server_guid());

    call_hello(&mut connection);
    println!();
    // because the first thing after the reply to `Hello` we receive is a "NameAcquired" signal for
    // ourselves, we first need to read one more message before being able to actually read the
    // introspect response
    print!("< ");
    let message = connection.read_message().unwrap();
    println!("{:?}", &message);
    println!();
    call_introspect(&mut connection);
}

fn call_hello(connection: &mut Connection) {
    let header_fields = vec![
        HeaderField::Path(ObjectPath::try_from("/org/freedesktop/DBus").unwrap()),
        HeaderField::Member(MemberName::try_from("Hello").unwrap().into()),
        HeaderField::Interface(
            InterfaceName::try_from("org.freedesktop.DBus")
                .unwrap()
                .into(),
        ),
        HeaderField::Destination("org.freedesktop.DBus".to_owned()),
    ];
    let message = Message {
        endianness: Endianness::LittleEndian,
        ty: MessageType::MethodCall,
        flags: Flags::none(),
        major_protocol_version: MajorProtocolVersion(1),
        serial: 1,
        body: None,
        header_fields,
    };

    print!("> {:?}", &message);
    connection.write_message(&message).unwrap();
    println!();

    print!("< ");
    let message = connection.read_message().unwrap();
    println!("{:?}", &message);
}

fn call_introspect(connection: &mut Connection) {
    let header_fields = vec![
        HeaderField::Path(ObjectPath::try_from("/org/freedesktop/DBus").unwrap()),
        HeaderField::Member(MemberName::try_from("Introspect").unwrap().into()),
        HeaderField::Interface(
            InterfaceName::try_from("org.freedesktop.DBus.Introspectable")
                .unwrap()
                .into(),
        ),
        HeaderField::Destination("org.freedesktop.DBus".to_owned()),
    ];
    let message = Message {
        endianness: Endianness::LittleEndian,
        ty: MessageType::MethodCall,
        flags: Flags::none(),
        major_protocol_version: MajorProtocolVersion(1),
        serial: 2,
        body: None,
        header_fields,
    };

    print!("> {:?}", &message);
    connection.write_message(&message).unwrap();
    println!();

    print!("< ");
    let message = connection.read_message().unwrap();
    println!("{:?}", &message);
}
