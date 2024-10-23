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

    // Change Port
    pub fn port(&mut self, port: u16) -> &mut Self {
        self.port = port;
        self
    }

    // Run service
    pub async fn run(&self) {

        

        
    }








    // Debug methods
    pub fn print_url(&self) {
        println!("{}:{}", self.url, self.port)
    }
}
