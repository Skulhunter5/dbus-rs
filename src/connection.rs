use std::{io::{Read as _, Write as _}, os::unix::net::UnixStream, path::Path};

use crate::PRINT_HANDSHAKE;

#[derive(Debug)]
pub struct Connection {
    stream: UnixStream,
    server_guid: String,
}

impl Connection {
    pub fn init(path: impl AsRef<Path>) -> std::io::Result<Self> {
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
                let message_part = match str::from_utf8(&recv_buffer) {
                    Ok(message_part) => message_part,
                    Err(_) => return Err(std::io::Error::new(std::io::ErrorKind::InvalidData, "received invalid utf8 during handshake")),
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
        let mut stream = UnixStream::connect(path.as_ref())?;

        stream.write_all(&[0])?;

        write(&mut stream, "AUTH EXTERNAL")?;

        let auth_response = read(&mut stream)?;
        if auth_response != "DATA" {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("unexpected response to auth message during handshake: {:?}", auth_response)));
        }

        write(&mut stream, "DATA")?;

        let data_response = read(&mut stream)?;
        if data_response.starts_with("REJECTED") {
            return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("dbus session bus rejected the authentication: {:?}", data_response)));
        }
        let server_guid = {
            let Some((left, right)) = data_response.split_once(' ') else {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("unexpected response to data message during handshake: {:?}", data_response)));
            };
            if left != "OK" {
                return Err(std::io::Error::new(std::io::ErrorKind::Other, format!("unexpected response to data message during handshake: {:?}", data_response)));
            }
            right.to_owned()
        };

        write!(&mut stream, "BEGIN")?;

        Ok(Self { stream, server_guid })
    }

    pub fn server_guid(&self) -> &str {
        self.server_guid.as_str()
    }
}
