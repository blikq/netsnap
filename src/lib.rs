use reqwest::{self, dns, Client};
use std::fs::File;
use std::time::Instant;
mod tools;
use tools::*;
mod inbuilt_errors;
use inbuilt_errors::*;
use inbuilt_errors::Error as InError;

pub struct NetSnap {
    url: String,
    port: u16,
    max: u64,
    rate: u64,
    interval: u64,
}

impl NetSnap {
    // initalize NetSnap Object
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            port: 80,
            max: 1,
            rate: 1,
            interval: 500
        }
    }

    // Change Port
    pub fn port(&mut self, port: u16) -> &mut Self {
        self.port = port;
        self
    }
    
    // Returning result to make sure config is valid
    pub fn config(&mut self, max: u64, rate: u64, interval: u64) -> &mut Self{
        if max >= rate{
            self.max = max;
            self.rate = rate;
            self.interval = interval;
            self
        }else {
            panic!("NETSNAP: INVALID CONFIG")
        }

    }



    // Run service
    pub async fn run(&self) {
        let mut max = self.max.clone() as i32;
        println!("Preparing to senf {max} requests");
        while max > 0 {
            println!("started");


            let batch_size = if max >= self.rate as i32 {
                self.rate as i32
            } else {
                max
            };
            for _ in 0..batch_size {
                let url_clone = self.url.clone();
                tokio::spawn( async  move {
                    process_request(url_clone).await;
                });

            }

            max -= batch_size;

            println!("remaining requests left {max}");
            if max > 0{
                println!("waiting...");
                tokio::time::sleep(tokio::time::Duration::from_millis(self.interval)).await;
            }
        }
        println!("Finished Benchmark")
    }








    // Debug methods
    pub fn print_url(&self) {
        println!("{}:{}", self.url, self.port)
    }
}
