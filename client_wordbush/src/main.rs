use std::{
    net::TcpStream,
    io,
    io::{BufRead, BufWriter, BufReader, Write, Read},
};
use protocol;

fn run_game(server_stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    // server_stream.write("hej".as_bytes());

    // Initialising resources
    let stdin: io::Stdin = io::stdin();
    let mut input_buffer = String::new();

    let server_response = protocol::receive_string_from_stream(&server_stream)?;
    println!("{server_response}");

    // Communication
    // println!("Input a word to start:");
    let _ = stdin.read_line(&mut input_buffer); 
    let _ = input_buffer.pop(); // remove newline
    protocol::write_string_to_stream(&server_stream, &input_buffer)?;
    let server_response = protocol::receive_string_from_stream(&server_stream)?;
    println!("{server_response}");
    // let message = Message {content: input_buffer.copy()}

    // server_stream.write(input_buffer.as_bytes())?;
    // server_stream.read_exact(&mut read_buffer)?;
    // println!("{read_buffer}");
    // server_stream.flush();
    // server_stream.into_inner().unwrap().shutdown();


    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Welcome to Wordbush!");
    println!("Connecting to server...");

    let stream = TcpStream::connect("127.0.0.1:7878")?;
    println!("Connected!");

    run_game(stream)?;

    Ok(())
}
