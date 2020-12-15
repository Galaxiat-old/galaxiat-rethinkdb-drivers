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
    pub fn connect(&self) -> Result<connect, error>
    {
        let mut stream = TcpStream::connect(format!("{}:{}", self.hostname, self.port));
        match stream {
            Ok(mut e) => {
                let result = e.write_all(b"0x34c2bdc3");
                match result {
                    Ok(_) => print!("toto"),
                    Err(_) => print!("ERR")
                }
                let buf = &mut [0; 128];
                let read = e.read(buf);
                match read {
                    Ok(data) => print!("{:?}", buf),
                    Err(_) => print!("ERROR")
                }
                return Ok(connect{stream : e })
            },
            Err(_) => return Err(error{})
        }
    }
}