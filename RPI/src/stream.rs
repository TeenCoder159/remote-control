use std::{
    error::Error,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

/// Start listening at a provided IP
pub async fn start_server(ip_addr: &str) -> TcpListener {
    match TcpListener::bind(ip_addr) {
        Ok(a) => a,
        Err(e) => {
            panic!("{e}")
        }
    }
}

pub trait Stream {
    async fn read_stream(&mut self) -> Result<String, Box<dyn Error>>;
    async fn write_to_stream(&mut self, data: &str) -> Result<(), Box<dyn Error>>;
}

impl Stream for TcpStream {
    /// Read from the stream Asynchronously
    async fn read_stream(&mut self) -> Result<String, Box<dyn Error>> {
        let mut buf = [0u8; 1 << 20];
        self.read(&mut buf)?;

        let result = std::str::from_utf8(&mut buf)?.to_string();

        Ok(result)
    }
    /// Write to the stream Asynchronously
    async fn write_to_stream(&mut self, data: &str) -> Result<(), Box<dyn Error>> {
        self.write_all(data.as_bytes())?;
        Ok(())
    }
}
