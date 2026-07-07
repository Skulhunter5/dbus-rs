use std::{
    io::{Read as _, Write as _},
    os::unix::net::{UnixListener, UnixStream},
    thread,
};

use byteorder::ReadBytesExt as _;
use dbus::{message::Message, wire_format::MessageReader};

const PRINT_HANDSHAKE: bool = true;

fn main() {
    let listener = UnixListener::bind("./fake-bus-socket").unwrap();
    for res in listener.incoming() {
        let stream = match res {
            Ok(res) => res,
            Err(error) => {
                eprintln!("connection failed because of: {:?}", error);
                continue;
            }
        };
        thread::spawn(|| {
            let mut stream = stream;
            handle_connection(&mut stream).unwrap();
        });
    }
}

fn handle_connection(stream: &mut UnixStream) -> std::io::Result<()> {
    init_connection(stream)?;

    fn read_message(stream: &mut UnixStream) -> std::io::Result<Message> {
        Message::read_from(MessageReader::new(stream))
    }
    print!("< ");
    let message = read_message(stream).unwrap();
    println!("{:?}", &message);

    Ok(())
}

fn init_connection(stream: &mut UnixStream) -> std::io::Result<()> {
    fn write(stream: &mut UnixStream, message: impl AsRef<str>) -> std::io::Result<()> {
        if PRINT_HANDSHAKE {
            println!("> {:?}", message.as_ref());
        }

        stream.write_all(message.as_ref().as_bytes())?;
        stream.write_all(b"\r\n")
    }
    fn read(stream: &mut UnixStream) -> std::io::Result<String> {
        let mut buffer = [0u8; 1024];
        let mut message = String::new();
        while !message.ends_with("\r\n") {
            let bytes_read = stream.read(&mut buffer)?;
            let recv_buffer = &buffer[..bytes_read];
            let message_part = match str::from_utf8(recv_buffer) {
                Ok(message_part) => message_part,
                Err(_) => {
                    return Err(std::io::Error::new(
                        std::io::ErrorKind::InvalidData,
                        "received invalid utf8 during handshake",
                    ));
                }
            };
            message.push_str(message_part);
        }
        assert_eq!(message.pop(), Some('\n'));
        assert_eq!(message.pop(), Some('\r'));
        if PRINT_HANDSHAKE {
            println!("< {:?}", message);
        }
        Ok(message)
    }

    const SERVER_GUID: &str = "85eb6538cf16f31df2375cd343fbbc2a";

    let first_byte = stream.read_u8()?;
    assert_eq!(first_byte, 0);

    let first_message = read(stream)?;
    assert!(first_message.starts_with("AUTH EXTERNAL"));

    write(stream, format!("OK {SERVER_GUID}"))?;

    let second_message = read(stream)?;
    assert_eq!(second_message, "NEGOTIATE_UNIX_FD");

    // write(stream, "ERROR no idea what to do for this")?;
    write(stream, "AGREE_UNIX_FD")?;

    let third_message = read(stream)?;
    assert_eq!(third_message, "BEGIN");

    Ok(())
}
