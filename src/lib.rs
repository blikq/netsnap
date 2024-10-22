use reqwest::{self, dns};
use std::fs::File;
use std::time::Instant;
mod tools;
use tools::*;
mod inbuilt_errors;

pub struct NetSnap {
    url: String,
    port: u16,
}

impl NetSnap {

    // initalize NetSnap Object
    pub fn new(url: &str) -> Self {
        Self { 
            url: url.to_string(),
            port: 80,
        }
    }

    pub async fn run(&self) {
        // DNS LookUp
        //TODO: Resolve different types of urls
        let dns_lookup_duration = dns_lookup(&format!("{}:{}",self.url, self.port).to_string()).await.unwrap();
        
    }

    pub fn print_url(&self) {
        println!("{}:{}", self.url, self.port)
    }

    pub fn port(&mut self, port: u16) -> &mut Self {
        self.port = port;
        self
    }
}
