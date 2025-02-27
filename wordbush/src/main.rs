use anyhow::Result;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;

// Todo:
// Build word dictionary
// create game loop and mechanics
// terminal graphics
 


// Jeg skal bygge en trie
// Det er en datastruktur, som består af structs der indeholder information, og pointers til nye structs
// Informationen er et eller flere chars. Hvis man løber fra stammen ned til en struct, kan man danne et ord



struct Radix {
    chars: &str,
    children: HashMap<&str, RadixNode>,
    is_word: bool,
}


fn insert(&self, root: &RadixNode, word: &str) -> Result<()> {
    let mut current_word = "";
    let next_node = root.children.get(word[..1]);
    match next_node {
        Some(child) => {
            if word == child.chars {
                println!("Word: {word} already exists in tree");
                ()
            }
            let common_prefix = zip(child.chars, word)
                .filter(|(a,b)| a == b)
                .collect()::<_>;

            if common_prefix.len() == word.len() {
                // Word 
            }            


            if common_prefix.len() == child.chars.len() {
                //Prefix exists, traverse tree further
                insert(child, word[common_prefix.len()..])
            }
            
            // Prefix not shared between child and word, split needed
            // Create intermediate node which replaces the child in the children map
            let updated_child = Radix { chars: child.chars[1..], children: child.children, is_word = child.is_word };

            let mut intermediate_node_children : HashMap<&str, RadixNode> = HashMap::from([(updated_child.chars, updated_child)]);
            let intermediate_node = Radix {chars: word[..1], }            



        }
    }
}



fn main() -> Result<()> {
    let f = File::open("data/words_alpha.txt")?;
    let mut reader = BufReader::new(f);
    let mut line = String::new();
    // let len = reader.read_line(&mut line)?;
    // println!("{len}");
    // println!("{line}");


    while let len = reader.read_line(&mut line)? {
        if len == 0 {
            break;
        }
        println!("{line}");
    }

    Ok(())
}
