use std::{
    io::{Read as _, Write as _},
    os::unix::net::{UnixListener, UnixStream},
    path::Path,
    thread,
};

use byteorder::ReadBytesExt as _;
use dbus::{
    message::Message,
    wire_format::{MessageReader, MessageWriter},
};

const PRINT_HANDSHAKE: bool = false;

fn main() {
    let dbus_session_bus_address =
        std::env::var("DBUS_SESSION_BUS_ADDRESS").expect("DBUS_SESSION_BUS_ADDRESS is not set");
    let path = if dbus_session_bus_address.starts_with("unix:path=") {
        let (_, path) = dbus_session_bus_address.split_once('=').unwrap();
        Path::new(path)
    } else {
        panic!("unknown format for DBUS_SESSION_BUS_ADDRESS");
    };

    thread::scope(|scope| {
        let listener = UnixListener::bind("./proxy-bus-socket").unwrap();
        for res in listener.incoming() {
            let stream = match res {
                Ok(res) => res,
                Err(error) => {
                    eprintln!("connection failed because of: {:?}", error);
                    continue;
                }
            };
            scope.spawn(|| {
                let mut stream_c = stream;
                let mut stream_s = UnixStream::connect(path).unwrap();
                handle_connection(&mut stream_c, &mut stream_s).unwrap();
            });
        }
    });
}

fn handle_connection(stream_c: &mut UnixStream, stream_s: &mut UnixStream) -> std::io::Result<()> {
    init_connection(stream_c, stream_s)?;

    fn read_message(stream: &mut UnixStream) -> std::io::Result<Message> {
        Message::read_from(MessageReader::new(stream))
    }
    fn write_message(stream: &mut UnixStream, message: &Message) -> std::io::Result<()> {
        message.write_to(MessageWriter::new(stream))
    }

    let mut buffer = [0u8; 1024];
    let bytes_read = stream_c.read(&mut buffer).unwrap();
    println!("{:?}", &String::from_utf8_lossy(&buffer[..bytes_read]));

    print!("< ");
    let message = read_message(stream_c).unwrap();
    println!("{:?}", &message);
    write_message(stream_s, &message).unwrap();

    Ok(())
}

fn init_connection(stream_c: &mut UnixStream, stream_s: &mut UnixStream) -> std::io::Result<()> {
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
                    println!(">>> {:?}", String::from_utf8_lossy(recv_buffer));
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

    let first_byte = stream_c.read_u8()?;
    assert_eq!(first_byte, 0);
    stream_s.write_all(&[0])?;

    loop {
        let message = read(stream_c)?;
        write(stream_s, &message)?;

        if message == "BEGIN" {
            if PRINT_HANDSHAKE {
                println!();
                println!("Handshake completed");
                println!();
            }
            break;
        }

        let message = read(stream_s)?;
        write(stream_c, &message)?;
    }

    Ok(())
}
