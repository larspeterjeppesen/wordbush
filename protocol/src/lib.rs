use std::{
    io::{Read, Write}, net::TcpStream, ops::Deref
};

const MAX_MSG_SIZE: usize = 128;

fn u8_to_u32(arr: [u8; 4]) -> u32 {
    arr[0] as u32
    + ((arr[1] as u32) << 8 )
    + ((arr[2] as u32) << 16) 
    + ((arr[3] as u32) << 24)
}

fn u32_to_u8(n: u32) -> [u8; 4] {
    let mut arr: [u8; 4] = [0; 4];
    arr[0] = (n & 0xff) as u8;
    arr[1] = ((n >> 8) & 0xff) as u8;
    arr[2] = ((n >> 16) & 0xff) as u8;
    arr[3] = ((n >> 24) & 0xff) as u8;
    arr
}

pub fn write_string_to_stream(mut stream: &TcpStream, content: &String) -> Result<(), Box<dyn std::error::Error>> {
    let content_bytes = content
        .chars()
        .map(|char| char as u8)
        .collect::<Vec<u8>>();

    let message_bytes: Vec<u8> = u32_to_u8(content_bytes.len() as u32)
        .iter()
        .chain(content.clone()
                .deref()
                .as_bytes()
                .iter())
        .map(|b| *b)
        .collect::<Vec<u8>>();
    let content_len = message_bytes.len();
    match content_len {
        0 => return Err(Box::<dyn std::error::Error>::from("cannot send empty message")),
        len if len > MAX_MSG_SIZE => return Err(Box::<dyn std::error::Error>::from("message content too large")),
        _ => (),
    }
    stream.write_all(&message_bytes)?;
    Ok(())
}

pub fn receive_string_from_stream(mut stream: &TcpStream) -> Result<String, Box<dyn std::error::Error>> {
    let mut content_len_buf: [u8; 4] = [0; 4];
    stream.read_exact(&mut content_len_buf)?;
    let content_len: u32 = u8_to_u32(content_len_buf);
    let mut content_buf: Vec<u8> = vec![0_u8; content_len as usize];
    stream.read_exact(&mut content_buf)?;

    let content = String::from_utf8(content_buf)?;
    Ok(content)
}


#[cfg(test)]
mod tests {
    use super::*;
    use std::net::TcpListener;

    #[test]
    fn receive_string_from_stream_test() {
        let addr = "127.0.0.1:50123";
        let tcp_listener = TcpListener::bind(addr).unwrap();
        let listener = tcp_listener.incoming();
        let sender_stream = TcpStream::connect(addr).unwrap();
        let receiver_stream = listener.take(1).next().unwrap().unwrap();

        let res: Result<(), Box<dyn std::error::Error>> = write_string_to_stream(&sender_stream, &String::from("test message"));
        assert!(res.is_ok_and(|x| x == ()));

        let received_string: String = receive_string_from_stream(&receiver_stream).unwrap();
        assert_eq!(received_string, String::from("test message"));
    }

    
}
