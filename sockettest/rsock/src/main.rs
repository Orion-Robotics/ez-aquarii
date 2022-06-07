use std::os::unix::net::UnixStream;
use std::io::prelude::*;

fn main() -> std::io::Result<()> {
    let mut stream = UnixStream::connect("/home/pythomancer/Documents/socket")?;
    stream.write_all(b"sussus amogus\n")?;
    let mut response = vec![0u8; 14];
    stream.read_exact(&mut response)?;
    let thingy = String::from_utf8(response).expect("heeehee haw haw");
    println!("{thingy}");
    Ok(())
}