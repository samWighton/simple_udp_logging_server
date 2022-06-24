use std::fs::{File,OpenOptions};
use std::io::prelude::*;
use std::net::UdpSocket;
use std::path::Path;
use std::str;

extern crate chrono;

const BUFFER_LENGTH: usize = 4096;

fn start_server(file: &mut File) -> std::io::Result<()> {
    {
        let socket = UdpSocket::bind("0.0.0.0:7000")?;
        let mut buf;
        let mut utc: chrono::DateTime<chrono::Utc>;

        loop {
            buf = [0; BUFFER_LENGTH];
            let (amt, src) = match socket.recv_from(&mut buf){
                Ok(v) => v,
                Err(e) => {
                    //TODO log ip address of invalid message
                    println!("Invalid UTF-8 sequence: {}", e);
                    continue
                },
            };
            // println!("{:?}", buf);

            if amt == BUFFER_LENGTH {
                //TODO log ip address of invalid message
                println!("message longer than buffer length");
                continue;
            }

            match str::from_utf8(&buf[..amt]) {
                Ok(s) => {
                    utc = chrono::Utc::now();
                    match file.write_all(format!("{}|{}|{}|\n", utc, src, s.trim_end()).as_bytes()) {
                        Err(why) => panic!("couldn't write to file: {}", why),
                        Ok(_) => {},
    }
                    println!("{}|{}|{}|", utc, src, s.trim_end());
                },
                Err(e) => println!("Invalid UTF-8 sequence: {}", e),
            };

            if false {
                break;
            }
        }
    }
    Ok(())
}

fn main() {

    let logging_file_name: String = chrono::Utc::now().to_string()
        .replace(" ", "_").replace("-", "_") + ".log";
    let path = Path::new("/tmp").join(&logging_file_name);

    match OpenOptions::new().append(true).create(true).open(&path) {
        Err(e) => panic!("couldn't create {}: {}", path.display(), e),
        Ok(mut file) => {
            //TODO handle failure and open a new file 
            start_server(&mut file);
        },
    };
}
