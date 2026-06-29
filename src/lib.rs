mod connection;
pub mod message;
mod names;
pub mod wire_format;
pub use connection::Connection;
pub use names::{BusName, InterfaceName, MemberName, ErrorName};

pub const PRINT: bool = true;
pub const PRINT_HANDSHAKE: bool = PRINT;
