mod radix;
mod protocol;
use std::{
    arch::x86_64, 
    fmt, 
    fs::File, 
    io::{self, prelude::*, BufReader, BufWriter}, 
    net::{self, TcpListener, TcpStream}, str, thread
};

use crate::radix::Radix;


fn build_word_radix(path: &str) -> Result<Radix, Box<dyn std::error::Error>> {
    let f = File::open(path)?;
    let mut reader = BufReader::new(f);
    // let mut line = String::new();

    let mut root = Radix::new();

    let mut i = 0;
    // let max = 370105;
    let max = 10i32.pow(6);
    loop {
        let mut line = String::new();
        let len = reader.read_line(&mut line)?;
        if len == 0 || i == max {
            break;
        }
        let word: String = line.chars().take(line.len()-2).collect();
        // println!("{word}");
        let _ = root.insert(word);


        i += 1;
        if i == max {
            break;
        }
    }
    println!("Loaded {i} words");
    Ok(root)

}

#[derive(Debug)]
struct WriteError;

impl std::fmt::Display for WriteError{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "Could not write entire response to client")
    }
}

impl std::error::Error for WriteError {}


fn run_game_instance(word_radix: &Radix, mut stream: TcpStream) -> Result<(), Box<dyn std::error::Error>> {
    let mut word_buf = String::new();
    let mut read_buf: [u8; 128] = [0; 128]; 
    println!("Waiting for client input");
    // loop {
        // let message: Message = Message::from(&mut stream);

        stream.read_exact(&mut read_buf);
        let word_buf = String::from_utf8(Vec::from(read_buf)).unwrap();
        // println!("Received word: {word_buf}");

        // match word_radix.lookup(&word_buf) {
        //     Ok(()) => break,
        //     Err(_) => {
        //         let response = "Input is not recognized as a word, please try again.";
        //         let bytes_written: usize = buf_client_writer.write(response.as_bytes())?;
        //         if bytes_written != response.as_bytes().len() {
        //             println!("Failed to write response at line: {}", line!());
        //             return Err(Box::new(WriteError)); 
        //         }
        //     }
        // }
    // }

    // let res = buf_client_writer.into_inner().unwrap().shutdown(net::Shutdown::Both);
    // println!("Shutdown result: {res:?}");
    Ok(())
}



// fn run_local_game(word_radix: &Radix, mut stream: TcpStream) -> Result<()> {
//     let mut buf_reader = BufReader::new(&stream);


    
    // let mut data: [u8; MAX_WORD_SIZE_U8] = [0; MAX_WORD_SIZE_U8];
    // buf_reader.read_exact(&mut data);    
    // let word: [char; MAX_WORD_SIZE_CHAR] = data
    //     .as_slice()
    //     .chunks_exact(4)
    //     .map(|bytes| char::from_u32((&raw const bytes) as u32).unwrap())
    //     .collect();





//     let mut primary_word_buffer = String::new();
//     let stdin: Stdin = stdin();

//     println!("Input a word to start:");
//     loop {
//         primary_word_buffer.clear();
//         let _ = stdin.read_line(&mut primary_word_buffer);
//         let _ = primary_word_buffer.pop(); // Remove newline

//         match lookup(word_radix, &primary_word_buffer) {
//             Ok(()) => break,
//             Err(_) => println!("Input is not a word, please try again."),
//         } 
//     }

//     // radix tree to track used words
//     let mut used_words_radix = Radix::new();
//     let _ = insert(&mut used_words_radix, primary_word_buffer.clone());

//     let mut points: usize = 0;
//     let mut combo: usize = 1;
//     let max_combo: usize = 8;
//     let mut character_word_buffer = String::new();

//     loop {
//         println!("Your word is {primary_word_buffer}. Points: {points}. Combo: {combo}.");
//         let mut word_char_iter = primary_word_buffer.chars();

//         // One letter words should not have their first char skipped
//         if primary_word_buffer.len() != 1 {
//             let _ = word_char_iter.next();
//         }
//         while let Some(c) = word_char_iter.next() {
//             println!("{c}");

//             loop {
//                 character_word_buffer.clear();
//                 let _ = stdin.read_line(&mut character_word_buffer);
//                 let _ = character_word_buffer.pop(); // Remove newline

//                 let first_char = character_word_buffer.chars().next().unwrap();

//                 if let Err(_) = lookup(&word_radix, &character_word_buffer) {
//                     combo = 1;
//                     println!("Your input is not a word. Combo: {combo}");
//                     continue;
//                 }

//                 if first_char.to_lowercase().next().unwrap() != c.to_lowercase().next().unwrap() {
//                     combo = 1;
//                     println!("Your word needs to start with the same letter as your current character. Combo: {combo}");
//                     continue;
//                 }

//                 if let Ok(()) = lookup(&used_words_radix, &character_word_buffer) {
//                     combo = 1;
//                     println!("You have already used that word. Combo: {combo}");
//                     continue;
//                 }

//                 let _ = insert(&mut used_words_radix, character_word_buffer.clone());
//                 let awarded_points: usize = combo * character_word_buffer.len();
//                 if combo < max_combo {
//                     combo += 1;
//                 }
//                 points += awarded_points; 
//                 println!("Registered word {character_word_buffer}. Points awarded: {awarded_points}. Total points: {points}. Combo: {combo}");

//                 break;
//             }
//         } 
//         if primary_word_buffer == character_word_buffer {
//             return Ok(());
//         }
//         primary_word_buffer = character_word_buffer.clone();
//     }


// }


fn main() -> Result<(), Box<dyn std::error::Error>> {
    // Initialize word db
    // let n: [u8; 4] = n.to_be_bytes();
    let path: &str = "data/words_alpha.txt";
    let word_radix = build_word_radix(path)?;

    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    for stream in listener.incoming().take(2) {
        let stream = stream.unwrap();

        run_game_instance(&word_radix, stream)?;
    } 

    Ok(())
}

