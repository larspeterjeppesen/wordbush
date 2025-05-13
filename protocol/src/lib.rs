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

#[derive(Debug, PartialEq)]
pub struct Message {
    content_len: u32,
    content: String,
}

impl Message {
    pub fn build(content: &String) -> Result<Message, Box<dyn std::error::Error>>{
        if content.len() == 0 {
            return Err(Box::<dyn std::error::Error>::from("cannot build a message with content of length zero"));
        }
        let message = Message {
            content_len: content.len() as u32,
            content: content.clone(),
        };
        Ok(message)
    }

    pub fn send_message(mut stream: &TcpStream, content: &String) -> Result<(), Box<dyn std::error::Error>> {
        let content_len = content.len() as u32;
        match content_len {
            0 => return Err(Box::<dyn std::error::Error>::from("cannot send empty message")),
            len if len > MAX_MSG_SIZE as u32=> return Err(Box::<dyn std::error::Error>::from("message content too large")),
            _ => (),
        }

        let message_bytes: Vec<u8> = u32_to_u8(content_len)
            .iter()
            .chain(content.clone()
                    .deref()
                    .as_bytes()
                    .iter())
            .map(|b| *b)
            .collect::<Vec<u8>>();
        let n = stream.write(&message_bytes)?;
        Ok(())
    }

    pub fn from_stream(mut stream: &TcpStream) -> Result<Message, Box<dyn std::error::Error>> {
        let mut content_len_buf: [u8; 4] = [0; 4];
        stream.read_exact(&mut content_len_buf)?;
<<<<<<< HEAD

        println!("len buf: {:?}", content_len_buf);
        let content_len: u32 = u8_to_u32(content_len_buf);
        println!("content_len: {:?}", content_len);

        let mut content_buf = String::new();
        stream.take(content_len as u64).read_to_string(&mut content_buf)?;
=======
        let content_len: u32 = u8_to_u32(content_len_buf);
        // let mut content_buf = String::new();
        let mut content_buf: Vec<u8> = Vec::with_capacity(content_len as usize);
        stream.read_exact(&mut content_buf);

        let mut u32_data: Vec<u32> = vec![0_u32; content_buf.len()/4];
        // let mut u8_ref = unsafe {
        //     std::slice::from_raw_partsu32_data.as_mut_ptr().cast::<u8>();
        // }
        let u8_ref: &mut [u8] = unsafe{
            std::slice::from_raw_parts_mut(u32_data.as_mut_ptr() as *mut u8, 
            content_buf.len())};
        // std::slice::copy_from_slice(u8_data, raw_data);
        u8_ref.copy_from_slice(&content_buf);

        let content: String = u32_data
            .iter()
            .map(|x| x.to_string())
            .collect();
        // stream.take(content_len as u64).read_to_string(&mut content_buf)?;
>>>>>>> 0ed361d (all work pushed to work on laptop)
        let message = Message {
            content_len,
            content: content,
        };
        Ok(message)
    }

}

#[cfg(test)]
mod tests {
    use super::*;
    use std::net::TcpListener;
    #[test]
    fn build_message() {
        let content: String = String::from("testword");
        let message: Message = Message {
            content_len: content.len() as u32, 
            content: content.clone(),
        };
        let built_message: Result<Message, Box<dyn std::error::Error>> = Message::build(&content);
        assert!(built_message.is_ok_and(|m| m == message));
    }

    #[test]
    fn build_empty_message() {
        let content: String = String::new();
        let built_message: Result<Message, Box<dyn std::error::Error>> = Message::build(&content);
        assert!(built_message.is_err_and(|e| (*e).to_string() == "cannot build a message with content of length zero"));
    }

    #[test]
    fn from_stream() {
        let addr = "127.0.0.1:50123";
        let tcp_listener = TcpListener::bind(addr).unwrap();
        let listener = tcp_listener.incoming();
        let sender_stream = TcpStream::connect(addr).unwrap();
        let receiver_stream = listener.take(1).next().unwrap().unwrap();

        let res = Message::send_message(&sender_stream, &String::from("test message"));
        assert!(res.is_ok_and(|x| x == ()));

        let received_message: Message = Message::from_stream(&receiver_stream).unwrap();
        let test_message = Message::build(&String::from("test message")).unwrap();
        assert_eq!(received_message, test_message);
    }
}
