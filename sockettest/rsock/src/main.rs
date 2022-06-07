use std::os::unix::net::UnixStream;
use std::io::prelude::*;
extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate rmp_serde as rmps;

use serde::{Deserialize, Serialize};
use rmps::{Deserializer, Serializer};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
struct pack {
    num: u8,
    float: f32,
    string: String
}

fn main() -> std::io::Result<()> {
    let mut stream = UnixStream::connect("/home/pythomancer/Documents/socket")?;
    loop {
        let mut response = vec![0u8; 200];
        stream.read_exact(&mut response).expect("heeehee haw haw");
        let a: (u8, f32, String) = rmp_serde::from_slice(&response).unwrap();
        println!("{}", a.get(3));
        // let thingy = String::from_utf8(response);
        // println!("{thingy}");
    }
    Ok(())
}
