use std::{
    net::TcpStream, 
    io::{Read, Write}
};

fn u8_to_u32(arr: [u8; 4]) -> u32 {
    arr[0] as u32
    + (arr[1] as u32) << 8 
    + (arr[2] as u32) << 16 
    + (arr[3] as u32) << 24
}

fn u32_to_u8(n: u32) -> [u8; 4] {
    let mut arr: [u8; 4] = [0; 4];
    arr[0] = (n & 0xff) as u8;
    arr[1] = ((n >> 8) & 0xff) as u8;
    arr[2] = ((n >> 16) & 0xff) as u8;
    arr[3] = ((n >> 24) & 0xff) as u8;
    arr
}

#[derive(Debug)]
pub struct Message {
    content_len: u32,
    content: String,
}

impl Message {
    pub fn build(content: String) -> Message {
        let content_len: u32 = content.as_bytes().len() as u32;
        Message {
            content_len,
            content
        }
    }

    pub fn from_stream(mut stream: TcpStream) -> Result<Message, Box<dyn std::error::Error>> {
        let mut content_len_buf: [u8; 4] = [0; 4];
        stream.read_exact(&mut content_len_buf)?;
        let content_len: u32 = u8_to_u32(content_len_buf);
        let mut content_buf = String::new();
        stream.take(content_len as u64).read_to_string(&mut content_buf)?;
        let message = Message {
            content_len,
            content: content_buf,
        };
        Ok(message)
    }

    pub fn send_contents_to_stream(self, mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
        let data: Vec<u8> = u32_to_u8(self.content_len)
            .iter()
            .chain(self.content.as_bytes().iter())
            .map(|b| *b)
            .collect::<Vec<u8>>();
        stream.write(&data[..])?;
        Ok(())
    }
}

