/// An error in parsing a text PROXY protocol header.
/// Partial headers are  missing their new line suffix (`MissingNewLine`).
/// All other parse errors are considered non-retryable (i.e. more data will not change the outcome).
#[derive(thiserror::Error, Debug, PartialEq)]
pub enum ParseError {
    #[error("Header must start with 'PROXY'.")]
    InvalidPrefix,
    #[error("Header does not end with the string '\\r\\n'.")]
    MissingNewLine,
    #[error("Header missing protocol.")]
    MissingProtocol,
    #[error("Header missing source address.")]
    MissingSourceAddress,
    #[error("Header missing destination address.")]
    MissingDestinationAddress,
    #[error("Header missing source port.")]
    MissingSourcePort,
    #[error("Header missing destination port.")]
    MissingDestinationPort,
    #[error("Header contains additional characters after the destination port, but before the '\\r\\n'.")]
    UnexpectedCharacters,
    #[error("Header does not fit within the expected buffer size of 107 bytes (plus 1 byte for null-terminated strings).")]
    HeaderTooLong,
    #[error("Header has an invalid protocol.")]
    InvalidProtocol,
    #[error("Header contains invalid IP address for the source.")]
    InvalidSourceAddress(#[source] std::net::AddrParseError),
    #[error("Header contains invalid IP address for the destination.")]
    InvalidDestinationAddress(#[source] std::net::AddrParseError),
    #[error("Header contains invalid TCP port for the source.")]
    InvalidSourcePort(#[source] Option<std::num::ParseIntError>),
    #[error("Header contains invalid TCP port for the destination.")]
    InvalidDestinationPort(#[source] Option<std::num::ParseIntError>),
}

/// An error in parsing a text PROXY protocol header that is represented as a byte slice.
#[derive(thiserror::Error, Debug, PartialEq)]
pub enum BinaryParseError {
    #[error(transparent)]
    Parse(#[from] ParseError),
    #[error("Header is not valid UTF-8.")]
    InvalidUtf8(#[from] std::str::Utf8Error),
}
