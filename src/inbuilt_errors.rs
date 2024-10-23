#[derive(Debug)]
pub enum Error {
    DnsLookupError(Info),
    HandshakeError(Info),
}

#[derive(Debug)]
pub struct Info {
    pub url: String,
    pub kind: String,
}

