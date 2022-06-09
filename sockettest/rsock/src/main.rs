use tokio::net::UnixStream;
use std::io::prelude::*;
use serde::{Deserialize, Serialize};
use rmps::{Deserializer, Serializer};
use anyhow::Result;
use tokio::io::{AsyncRead, AsyncReadExt, Take};

extern crate serde;
#[macro_use]
extern crate serde_derive;
extern crate rmp_serde as rmps;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Packet {
    num: i32,
    float: f32,
    string: String
}

pub async fn read<T: AsyncRead + Unpin>(mut input: T) -> Result<Take<T>> {
	let length = input.read_i32_le().await?;
	Ok(input.take(length as u64))
}

pub async fn read_msgpack<'a, T, R>(input: R) -> Result<T>
where
	R: AsyncRead + Unpin,
	T: serde::de::DeserializeOwned,
{
	let mut buf = Vec::new();
    println!("{:?}", buf);
	read(input).await?.read_to_end(&mut buf).await?;
	Ok(rmp_serde::from_slice::<T>(&buf)?)
}
#[tokio::main]
async fn main() -> anyhow::Result<()> {
    let mut stream = UnixStream::connect("/home/pythomancer/Documents/socket").await?;
    loop {
        let p: Packet = read_msgpack(&mut stream).await?;
        println!("{}", p.float);
        // let mut len_response = vec![0u8; 4];
        // stream.read_exact(&mut len_response).expect("invalid len send");
        // let length: i32 = stream.read_i32_le();
        // let mut response = vec![0u8; ]
        // let a: (i32, f32, String) = rmp_serde::from_slice(&response).unwrap();
        // println!("{}", a.0);
        // let thingy = String::from_utf8(response);
        // println!("{thingy}");
        if p.float > 0.8{
            break;
        } 
    }
    anyhow::Ok(())
}
