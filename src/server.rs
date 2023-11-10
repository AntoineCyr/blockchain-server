use std::net::{TcpListener,TcpStream};
use std::thread;
use std::io::{Read,Write};
use std::str;
use crate::errors::Result;
use crate::blockchain::Blockchain;


pub struct Server{

}


impl Server{

    pub fn new()-> Result<Server>{
        Ok(Server {
        })
    }

pub fn run_server(&self) {
    fn handle_client(mut stream: TcpStream) -> Result<()> {
        let mut bc= Blockchain::create_blockchain()?;
        println!("Incoming connection from: {}", stream.peer_addr()?);
        let mut buf = [0;512];
        loop {
            let bytes_read = stream.read(&mut buf)?;
            if bytes_read == 0 {return Ok(())}
            //stream.write(&)?;
            let parts = str::from_utf8(&buf)?.split("_");
            let collection = parts.collect::<Vec<&str>>();
            if collection.len() == 3{
                let mut amount = String::from("");
                for c in collection[2].chars() {
                    if c.is_digit(10){
                            amount.push(c);
                    }
                 };
                bc.add_transaction(String::from(collection[0]), String::from(collection[1]), amount.parse().unwrap())?;
                stream.write("transaction added to mempool!".as_bytes()).expect("Failed to write response!");
            }
            else {
            let address = collection[0];
            let balance = bc.get_balance(String::from(address));
            let output = format!("Balance of '{address}'; {balance} ");
            stream.write(output.as_bytes()).expect("Failed to write response!");
        }
            stream.write(&[b'\n'])?;
        }
    }



    let listener = TcpListener::bind("0.0.0.0:8888").expect("Could not bind");
    for stream in listener.incoming(){
        match stream {
            Err(e) => {eprintln!("failed: {}",e)}
            Ok(stream) => {
                thread::spawn(move || {
                    handle_client(stream).unwrap_or_else(|error| eprintln!("{:?}", error));
                });
            }
        }
    }
}
}