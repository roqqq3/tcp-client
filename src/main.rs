use std::net::TcpStream;
use std::io::prelude::*;
use std::io::stdin;
use std::io::stdout;
use std::io::Result;
use std::str;

fn input_port() -> String {
    loop {
        println!("Enter port to connect to:");
        let mut port = String::new();
        match stdin().read_line(&mut port) {
            Ok(_) => return port,
            Err(e) => println!("Couldn't read given input. Error: {}", e)
        }
    }
}

fn input_ip() -> String {
    loop {
        println!("Enter IP address to connect to:");
        let mut ip = String::new();
        match stdin().read_line(&mut ip) {
            Ok(_) => return ip,
            Err(e) => println!("Couldn't read given input. Error: {}", e)
        }
    }
}

fn read_data(stream: &mut TcpStream, buffer: &mut [u8; 255]) -> Result<usize> {
    let n = stream.read(&mut buffer[..])?;
    return Ok(n);
}

fn print_data(buffer: &[u8; 255], n: usize) -> Result<()> {
    match str::from_utf8(&buffer[..n]) {
        Err(e) => {
            panic!("Data is in incorrect format! Only UTF-8 is supported. Error: {}", e);
        },
        Ok(s) => {
            print!("{}", s);
            stdout().flush()?;
            return Ok(());
        }
    }
}

fn main_loop(mut stream: &mut TcpStream, 
            mut readbuffer: &mut [u8; 255]) -> Result<()> {
    let n = read_data(&mut stream, &mut readbuffer)?;
    print_data(&readbuffer, n)?;
    return Ok(());
}

fn main() {
    loop {
        let ip = input_ip();
        let port = input_port();
        let formatted_info: String = format!("{}:{}", ip.trim(), port.trim());
        let con_info = formatted_info;
        println!("Connecting to {}..", con_info);
        let mut readbuffer = [0; 255];
        match TcpStream::connect(con_info) {
            Err(_) => println!("Failed to connect. Try again."),
            Ok(mut stream) => {
                println!("Connected!");
                loop {
                    match main_loop(&mut stream, &mut readbuffer) {
                        Err(e) => println!("Error: {}", e),
                        Ok(_) => continue
                    }
                }
            }
        }
    }
}
