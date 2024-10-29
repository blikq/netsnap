#[derive(Debug)]
pub enum Error {
    DnsLookupError(Info),
    HandshakeError(Info),
    InvalidConfig(Info),
    RequestFailed(Info),
}

#[derive(Debug)]
pub struct Info {
    pub url: String,
    pub kind: String,
}

