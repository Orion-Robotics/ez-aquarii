use std::thread;
use std::os::unix::net::{UnixStream, UnixListener};

fn handle_client(stream: UnixStream) -> std::io::Result<()>{
    let mut stream = UnixStream::connect("/path/to/my/socket")?;
    stream.write_all(b"hello world")?;
    let mut response = String::new();
    stream.read_to_string(&mut response)?;
    println!("{response}");
    Ok(())
}

fn main() -> std::io::Result<()> {
    let listener = UnixListener::bind("/path/to/the/socket")?;

    // accept connections and process them, spawning a new thread for each one
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                /* connection succeeded */
                thread::spawn(|| handle_client(stream));
            }
            Err(_err) => {
                println!("err");
                break;
            }
        }
    }
    Ok(())
}