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

// Let's think about insertion into a radix tree
// a node in a radix tree has a hashmap pointing to its' children.
// the hashmap keys are always a single character.
// say i have the following tree:
// root -> husk -> y
//      -> hush

/*


what cases do I need to consider?
Case 1: insertion onto a leaf, eg insert "husky" at a node containing chars "husk".
    We add a child to the leaf

Case 2: inserting parent to a leaf, eg insert "husk" before "husky"
Because "husk" prefixes "husky", we need to split "husky" into two nodes: "husk" and "y"
If "husky" has children, they will be added to the children of "y"

What if we have the tree "lea" -> "der" and want to insert "lead"?
When looking for the insertion spot, we find "lea" then check the value at key "d".
Since the value does not equal "d", we need to split the child into "d" -> "er"

The rule seems to be, keep matching the first char of the rest of the word until either 
ending up at a leaf or landing on a node that diverges from the insertion word

The word is guaranteed to fit there, as we are "building" the word as we look up the children




*/



fn insert(&self, root: &RadixNode, insertion_word: &str) -> Result<()> {
    let next_node = root.children.get(word[..1]);
    match next_node {
        Some(child) => {
            if insertion_word == child.chars {
                println!("Word: {word} already exists in tree");
                ()
            }
            let common_prefix = zip(child.chars, insertion_word)
                .take_while(|(a,b)| a == b)
                .map(|(a,_) a)
                .collect()::<_>;
            
            if common_prefix.len() < insertion_word.len() && common_prefix.len() < child.chars.len() {
                // child must be split to be the common prefix of both words.
                // the children of child must be set to the children of the node that will contain the suffix of child

            }

            if insertion_word.len() < child.chars.len() {
                // insert insertion_word as a parent of current_word
                Radix {chars: common_prefix}
            }
            else {
                // Search further
                return insert(&child, insertion_word[common_prefix.len()..])
            }

struct Radix {
    chars: &str,
    children: HashMap<&str, RadixNode>,
    is_word: bool,
}   




        },
        None => {
            //No match on first character, insert child
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
