use anyhow::Result;
use std::io::prelude::*;
use std::io::BufReader;
use std::fs::File;
use std::iter::zip;
use std::collections::HashMap;
// Todo:
// Build word dictionary
// create game loop and mechanics
// terminal graphics
 


// Jeg skal bygge en trie
// Det er en datastruktur, som består af structs der indeholder information, og pointers til nye structs
// Informationen er et eller flere chars. Hvis man løber fra stammen ned til en struct, kan man danne et ord


#[derive(Clone, Debug)]
struct Radix {
    chars: String,
    children: HashMap<char, Radix>,
    is_word: bool,
}

fn lookup(root: &Radix, key: String) -> Result<(), &str> {
    let lower_key = key.to_lowercase();
    let common_prefix: String = zip(root.chars.chars(), lower_key.chars())
        .take_while(|(a,b)| a == b)
        .map(|(a,_)| a)
        .collect();

    if lower_key == root.chars {
        if !root.is_word {
            return Err("Key found, but is not marked as a word");
        }
        return Ok(());
    }

    if (common_prefix.len() == 0 && root.chars.len() != 0) || 
        (common_prefix.len() == lower_key.len()) {
        return Err("Key not found in tree");
    }


    //Partial match found, search further
    let subkey = lower_key[common_prefix.len()..].to_string();
    let child_node_match = root.children.get(&subkey.chars().next().unwrap());
    match child_node_match {
        Some(child) => lookup(child, subkey),
        None => return Err("Key not found in tree"),
    }
}

fn insert(root: &mut Radix, insertion_word: String) -> Result<()> {
    let next_node: Option<&mut Radix> = root.children.get_mut(&insertion_word.chars().next().unwrap());
    match next_node {
        None => {
            // no match, insert node
            let child_key: char = insertion_word.chars().next().unwrap();
            let child_node = Radix {
                chars: insertion_word,
                children: HashMap::new(),
                is_word: true,
            };
            root.children.insert(child_key, child_node);
            Ok(())
        },
        Some(child) => {
            if insertion_word == child.chars {
                if !child.is_word  {
                    // Child may not be recognized as a word until now, make sure it is
                    child.is_word = true;
                }
                return Ok(());
            }
            let common_prefix: String = zip(child.chars.chars(), insertion_word.chars())
                .take_while(|(a,b)| a == b)
                .map(|(a,_)| a)
                .collect();
            
            if common_prefix.len() < insertion_word.len() && common_prefix.len() < child.chars.len() {
                // child must be split to be the common prefix of both words.
                // the children of child must be set to the children of the node that will contain the suffix of child
                let insertion_word_child = Radix {chars: insertion_word[common_prefix.len()..].to_string(), children: HashMap::new(), is_word: true};
                let new_child = Radix {chars: child.chars[common_prefix.len()..].to_string(), children: child.children.clone(), is_word: child.is_word};

                let new_parent = Radix {
                    chars: common_prefix, 
                    children: HashMap::from([
                        (new_child.chars.chars().next().unwrap(), new_child), 
                        (insertion_word_child.chars.chars().next().unwrap(), insertion_word_child)]), 
                    is_word: false};  
                root.children.insert(new_parent.chars.chars().next().unwrap(), new_parent);             

                return Ok(());
            }

            // alternatively insertion_word.len() == common_prefix.len()
            if insertion_word.len() < child.chars.len() {
                // insert insertion_word as a parent of current_word
                let new_child = Radix {
                    chars: child.chars.chars().take(common_prefix.len()).collect(), 
                    children: child.children.clone(), 
                    is_word: child.is_word};
                let parent = Radix {
                    chars: common_prefix, 
                    children: HashMap::from([(new_child.chars.chars().next().unwrap(), new_child)]), 
                    is_word: true};
                let _ = root.children.insert(
                    insertion_word.chars().next().unwrap(), 
                    parent);
                return Ok(());
            }
            else {
                // Search further
                let sub_insertion_word: String = insertion_word.chars().skip(common_prefix.len()).collect();
                return insert(child, sub_insertion_word);
            }

        },
    }
}



fn main() -> Result<()> {
    let f = File::open("data/words_alpha.txt")?;
    let mut reader = BufReader::new(f);
    let mut line = String::new();

    let mut radix_root = Radix {
        chars: String::from(""),
        children: HashMap::new(), 
        is_word: false,
    };
    println!("Empty tree:\n{:?}", radix_root);

    println!("Insertion word hejsa");
    let res = insert(&mut radix_root, String::from("hejsa"));
    println!("insert res: {:?}", res);
    println!("tree:\n{:?}", radix_root);

    println!("Looking up word hejsa");
    let res = lookup(&radix_root, String::from("hejsa"));
    println!("lookup res: {:?}", res);

    println!("Looking up word hej");
    let res = lookup(&radix_root, String::from("hej"));
    println!("lookup res: {:?}", res);

    println!("Inserting word hej");
    let res = insert(&mut radix_root, String::from("hej"));
    println!("insert res: {:?}", res);
    println!("tree:\n{:?}", radix_root);




    // let mut i = 0;
    // let max = 370105;
    // loop {
    //     let len = reader.read_line(&mut line)?;
    //     if len == 0 || i == max {
    //         break;
    //     }
    //     println!("{line}");
    //     i += 1;
    // }
    Ok(())
}
