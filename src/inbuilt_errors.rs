#[derive(Debug)]
pub enum Error {
    DnsLookupError(Info),
}

#[derive(Debug)]
pub struct Info {
    pub url: String,
    pub kind: String,
}