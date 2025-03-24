use anyhow::Result;
use std::io::prelude::*;
use std::io::BufReader;
use std::io::SeekFrom;
use std::io;
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

impl Radix {
    fn new() -> Radix {
        Radix {
            chars: String::from(""),
            children: HashMap::new(),
            is_word: false,
        }
    }
}


fn lookup<'a>(root: &'a Radix, key: &String) -> Result<(), &'a str> {
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
        Some(child) => lookup(child, &subkey),
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
                    chars: child.chars.chars().skip(common_prefix.len()).collect(), 
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


fn build_word_radix(path: &str) -> Result<Radix> {
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
        let _ = insert(&mut root, word);


        i += 1;
        if i == max {
            break;
        }
    }
    println!("Loaded {i} words");
    Ok(root)

}


fn main() -> Result<()> {
    let path: &str = "data/words_alpha.txt";
    let mut word_radix_root = build_word_radix(path)?;


    // loop where a word is first selected by the player, then we iterate through each character in the word, asking for words for each character.
    // When a word for the last word is typed, that word should become the word to iterate through for the next round

    let mut primary_word_buffer = String::new();
    let stdin: io::Stdin = io::stdin();

    println!("Input a word to start:");
    loop {
        primary_word_buffer.clear();
        let _ = stdin.read_line(&mut primary_word_buffer);
        let _ = primary_word_buffer.pop();
        let res = lookup(&mut word_radix_root, &primary_word_buffer);
        match res {
            Ok(()) => break,
            Err(_) => println!("Input is not a word, please try again."),
        } 
    }

    let mut character_word_buffer = String::new();

    loop {
        println!("Your word is {primary_word_buffer}");
        let mut word_char_iter = primary_word_buffer.chars();
        while let Some(c) = word_char_iter.next() {
            println!("{c}");

            character_word_buffer.clear();
            let _ = stdin.read_line(&mut character_word_buffer);
            let _ = character_word_buffer.pop();

            if character_word_buffer.chars().next().unwrap() != c {
                println!("You word needs to start with the same letter as your current character. You lose!");
                return Ok(());
            }

            let res = lookup(&mut word_radix_root, &character_word_buffer);
            if let Err(_) = res {
                println!("input is not a word, you lose!"); 
                return Ok(());
            }
        }
        
        primary_word_buffer = character_word_buffer.clone();

    }



    Ok(())
}





#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tree_insert_one_word() {
        let mut root = Radix {
            chars: String::from(""),
            children: HashMap::new(),
            is_word: false,
        };

        let _ = insert(&mut root, String::from("hej"));
        let child: Option<&Radix> = root.children.get(&'h');
        assert_eq!(child.is_some_and(|w| w.chars == "hej"), true);        
    }

    #[test]
    fn tree_insert_parent_then_child() {
        let mut root = Radix::new();

        let _ = insert(&mut root, String::from("hej"));
        let _ = insert(&mut root, String::from("hejsa"));


        let first_child: Option<&Radix> = root.children.get(&'h');
        assert_eq!(
            first_child.is_some_and(|node| node.chars == "hej"), 
            true,        
            "Failed with tree structure: {root:?}"
        );
        
        let second_child: Option<&Radix> = first_child.unwrap().children.get(&'s');
        assert_eq!(
            second_child.is_some_and(|node| node.chars == String::from("sa")), 
            true,
            "Failed with tree structure: {root:?}"
        );
    }

    #[test]
    fn tree_insert_child_then_parent() {
        let mut root = Radix::new();

        let _ = insert(&mut root, String::from("hejsa"));
        let _ = insert(&mut root, String::from("hej"));

        let first_child: Option<&Radix> = root.children.get(&'h');
        assert_eq!(
            first_child.is_some_and(|node| node.chars == "hej"), 
            true,
            "Failed with tree structure: {root:?}"
        );        
        
        let second_child: Option<&Radix> = first_child.unwrap().children.get(&'s');
        assert_eq!(
            second_child.is_some_and(|node| node.chars == String::from("sa")), 
            true,
            "Failed with tree structure: {root:?}"
        );
    }

    #[test]
    fn tree_split_common_prefix_into_parent() {
        let mut root = Radix::new();

        let _ = insert(&mut root, String::from("hejsa"));
        let _ = insert(&mut root, String::from("hejse"));
        
        let level_one_child: Option<&Radix> = root.children.get(&'h');
        assert_eq!(
            level_one_child.is_some_and(|node| node.chars == String::from("hejs")), 
            true,
            "Failed with tree structure: {root:?}"
        );

        let level_one_child = level_one_child.unwrap();
        assert_eq!(
            level_one_child.is_word,
            false,
            "Failed with tree structure: {root:?}"
        );

        let level_two_child_one: Option<&Radix> = level_one_child.children.get(&'a');
        assert_eq!(
            level_two_child_one.is_some_and(|node| node.chars == String::from("a")), 
            true,
            "Failed with tree structure: {root:?}"
        );

        let level_two_child_two: Option<&Radix> = level_one_child.children.get(&'e');
        assert_eq!(
            level_two_child_two.is_some_and(|node| node.chars == String::from('e')), 
            true,
            "Failed with tree structure: {root:?}"
        );
    }


    #[test]
    fn tree_mark_existing_node_as_word() {
        let mut root = Radix::new();

        let _ = insert(&mut root, String::from("hejsa"));
        let _ = insert(&mut root, String::from("hejse"));
        let _ = insert(&mut root, String::from("hejs"));

        let level_one_child: Option<&Radix> = root.children.get(&'h');
        assert_eq!(
            level_one_child.is_some_and(|node| node.is_word == true), 
            true,
            "Failed with tree structure: {root:?}"
        );
    }
    

    // Lookup tests
    // These all assume that the above insertion tests are passing

    #[test]
    fn tree_lookup_positive() {
        let mut root = Radix::new();

        let _ = insert(&mut root, String::from("hej"));
        let res = lookup(&root, String::from("hej"));
        assert_eq!(
            res,
            Ok(()),
            "Failed with tree structure: {root:?}"
        );
    }

    #[test]
    fn tree_lookup_negative() {
        let mut root = Radix::new();
        
        let _ = insert(&mut root, String::from("hej"));
        let res = lookup(&root, String::from("hehe"));
        assert_eq!(
            res,
            Err("Key not found in tree"),
            "Failed with tree structure: {root:?}",
        )
    }


    #[test]
    fn tree_insert_lookup_whole_db() {
        let f = File::open("/home/lars/Git/wordbush/wordbush/data/words_alpha.txt").unwrap_or_else(|_| panic!("Could not locate word database"));
        let mut reader = BufReader::new(f);
        // let mut line = String::new();

        let mut root = Radix::new();

        let mut i = 0;
        let max = 370105;
        loop {
            let mut line = String::new();
            let len = reader.read_line(&mut line).unwrap();
            if len == 0 || i == max {
                break;
            }
            let word: String = line.chars().take(line.len()-2).collect();
            let _ = insert(&mut root, word);

            if i == max {
                break;
            }
        }

        reader.seek(SeekFrom::Start(0)).unwrap();

        i = 0;
        loop {
            let mut line = String::new();
            let len = reader.read_line(&mut line).unwrap();
            if len == 0 || i == max {
                break;
            }
            let word: String = line.chars().take(line.len()-2).collect();
            // println!("{word}");
            let res = lookup(&root, word);

            assert_eq!(res, Ok(()));

            if i == max {
                break;
            }
        }
    }


}