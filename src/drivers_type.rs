use std::io::prelude::*;
use std::net::TcpStream;

pub struct client{
    port : i32,
    hostname : &'static str,

}
pub struct connect{
    stream : TcpStream
}
pub struct error{

}
impl Default for client{
    fn default() -> client{
        return client {
            port : 28015,
            hostname : "localhost"
        }
    }
    
}
impl client {
    fn connect(&self) -> Result<connect, error>
    {
        let mut stream = TcpStream::connect(format!("{}:{}", self.hostname, self.port));
        match stream {
            Ok(stream) => return Ok(connect{stream}),
            Err(_) => return Err(error{})
        }
    }
}