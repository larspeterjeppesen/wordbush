use std::{
    net::TcpStream,
    io,
    io::{BufRead, BufWriter, BufReader, Write, Read},
};


fn run_game(mut server_stream: TcpStream) -> io::Result<()> {
    // server_stream.write("hej".as_bytes());

    // Initialising resources
    // let mut buf_server_writer = BufWriter::new(&server_stream);
    // let mut buf_server_reader = BufReader::new(&server_stream);  
    let stdin: io::Stdin = io::stdin();
    let mut input_buffer = String::new();
    let mut read_buffer = String::new();

    // Communication
    println!("Input a word to start:");
    let _ = stdin.read_line(&mut input_buffer); 
    let _ = input_buffer.pop(); // remove newline

    server_stream.write(input_buffer.as_bytes())?;
    server_stream.read_exact(&mut read_buffer)?;
    println!("{read_buffer}");
    buf_server_stream.flush();
    buf_server_stream.into_inner().unwrap().shutdown()


    Ok(())
}

fn main() -> std::io::Result<()> {
    println!("Welcome to Wordbush!");
    println!("Connecting to server...");

    let stream = TcpStream::connect("127.0.0.1:7878")?;
    println!("Connected!");

    run_game(stream)?;

    Ok(())
}
