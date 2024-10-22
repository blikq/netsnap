use tokio::net::lookup_host;

use std::io::ErrorKind;
use std::time::{Duration, Instant};



use super::inbuilt_errors::Error as InError;
use super::inbuilt_errors::*;

pub async fn dns_lookup(url: &String) -> Result<Duration, InError> {
    // Start timer before DNS lookup
    let dns_start = Instant::now();
    match lookup_host(url.as_str()).await {
        Ok(obj) => {
            let dns_duration = dns_start.elapsed();
            Ok(dns_duration)
        },
        Err(e) => {
            Err(InError::DnsLookupError(Info {
                url: url.clone(),
                kind: e.to_string(),
            }))
        }
    }

}