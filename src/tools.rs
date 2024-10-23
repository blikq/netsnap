use tokio::net::lookup_host;

use std::io::ErrorKind;
use std::time::{Duration, Instant};

use reqwest::Client;

use super::inbuilt_errors::Error as InError;
use super::inbuilt_errors::*;

use serde_json;

pub async fn dns_lookup(url: &String) -> Result<Duration, InError> {
    // Start timer before DNS lookup
    let dns_start = Instant::now();
    match lookup_host(url.as_str()).await {
        Ok(obj) => {
            let dns_duration = dns_start.elapsed();
            Ok(dns_duration)
        }
        Err(e) => Err(InError::DnsLookupError(Info {
            url: url.clone(),
            kind: e.to_string(),
        })),
    }
}

pub async fn handshake_timer(url: &String) -> Result<Duration, InError> {
    let client = Client::new();

    let mut a_url = String::new();
    if !(url.starts_with("http://") | url.starts_with("https://")){
        a_url = format!("https://{}", url.clone());
    };

    // Measure TLS handshake + HTTP request time
    let handshake_start = Instant::now();

    // Send an HTTPS request to trigger the TLS handshake
    let response = client.get(a_url.as_str()).send().await;

    let handshake_duration = handshake_start.elapsed();

    if let Ok(resp) = response {
        Ok(handshake_duration)
    } else {
        Err(InError::HandshakeError(Info {
            url: a_url.clone(),
            kind: "Handshake failed".to_string(),
        }))
    }
}

pub async fn ttfb_timer(url: &String){
    let client = Client::new();

    let mut a_url = String::new();
    if !(url.starts_with("http://") | url.starts_with("https://")){
        a_url = format!("https://{}", url.clone());
    };

    // Measure  handshake + HTTP request time
    let ttfb_start = Instant::now();
    let response = client.get(a_url.as_str()).send();
    for i in response{
        
    }
}
