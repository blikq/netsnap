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
    interval_in_milli: u64,
}

impl NetSnap {
    // initalize NetSnap Object
    pub fn new(url: &str) -> Self {
        Self {
            url: url.to_string(),
            port: 80,
            max: 1,
            rate: 1,
            interval_in_milli: 500
        }
    }

    // Change Port
    pub fn port(&mut self, port: u16) -> &mut Self {
        self.port = port;
        self
    }
    
    // Returning result to make sure config is valid
    pub fn config(&mut self, max: u64, rate: u64, interval_in_milli: u64) -> &mut Self{
        if max >= rate{
            self.max = max;
            self.rate = rate;
            self.interval_in_milli = interval_in_milli;
            self
        }else {
            panic!("NETSNAP: INVALID CONFIG")
        }

    }



    // Run service
    pub async fn run(&self) {
        let mut successful:u64 = 0; 
        let mut unsuccessful:u64 = 0;

        let mut max = self.max.clone();
        println!("Preparing to send {max} requests");
        while max > 0 {
            println!("started");


            let batch_size = if max >= self.rate {
                self.rate
            } else {
                max
            };
            let mut tasks = vec![];
            
            for _ in 0..batch_size {
                let url_clone = self.url.clone();
                tasks.push(tokio::spawn(async move {
                    process_request(url_clone).await;
                    
                }));
            }
            for task in tasks {
                let r = task.await;
                if let Ok(_) = r{
                    println!("yes");
                    successful += 1;
                
                }else {
                    println!("no");

                    unsuccessful += 1;
                }
            }

            max -= batch_size;

            println!("remaining requests left {max}");
            if max > 0{
                println!("waiting...");
                tokio::time::sleep(tokio::time::Duration::from_millis(self.interval_in_milli)).await;
            }
        }
        println!("Finished Benchmark");
        println!("results\n\tsuccess = {successful}\n\tFailed = {unsuccessful}");
        let per = (successful/self.max) * 100;
        let mut grade:String;
        if per >= 75 {grade=String::from("A")}else if (75>per && per>=65) {grade=String::from("B")}else if (65>per && per>=50) {grade=String::from("C")}else{grade=String::from("F")} 
        println!("Benchmarking grade {grade}")
    }








    // Debug methods
    pub fn print_url(&self) {
        println!("{}:{}", self.url, self.port)
    }
}
