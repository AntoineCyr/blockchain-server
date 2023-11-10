use std::net::TcpStream;
use std::str;
use std::io::{BufRead,BufReader,Write};
use crate::errors::Result;


pub struct Client{

}


impl Client{

    pub fn new()-> Result<Client>{
        Ok(Client {
        })
    }

pub fn get_balance(&self,address:String) {
    let mut stream = TcpStream::connect("127.0.0.1:8888").expect("Could not connect to ser$");
        let mut buffer: Vec<u8> = Vec::new();
        stream.write(address.as_bytes()).expect("Failed to write to server");

        let mut reader = BufReader::new(&stream);

        reader.read_until(b'\n',&mut buffer).expect("Could not read into buffer");
        print!("{}", str::from_utf8(&buffer).expect("Could not write buffer as string"));
}

pub fn add_transaction(&self,from:String,to:String,amount:i32) {
    let mut stream = TcpStream::connect("127.0.0.1:8888").expect("Could not connect to ser$");
        let mut buffer: Vec<u8> = Vec::new();
        let input = format!("{from}_{to}_{amount}");

        stream.write(input.as_bytes()).expect("Failed to write to server");

        let mut reader = BufReader::new(&stream);

        reader.read_until(b'\n',&mut buffer).expect("Could not read into buffer");
        print!("{}", str::from_utf8(&buffer).expect("Could not write buffer as string"));
}
}
