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


#[derive(Clone)]
struct Radix {
    chars: String,
    children: HashMap<char, Radix>,
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



fn insert(mut root: &mut Radix, insertion_word: String) -> Result<()> {
    let next_node: Option<&mut Radix> = root.children.get_mut(&insertion_word.chars().next().unwrap());
    match next_node {
        None => {
            // no match, insert node
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

                let new_parent = Radix {chars: common_prefix, children: HashMap::from([(new_child.chars.chars().next().unwrap(), new_child), (insertion_word_child.chars.chars().next().unwrap(), insertion_word_child)]), is_word: false};  
                root.children.insert(new_parent.chars.chars().next().unwrap(), new_parent);             

                return Ok(());
            }

            // alternatively insertion_word.len() == common_prefix.len()
            if insertion_word.len() < child.chars.len() {
                // insert insertion_word as a parent of current_word
                let new_child = Radix {chars: child.chars.chars().take(common_prefix.len()).collect(), children: child.children.clone(), is_word: child.is_word};
                let parent = Radix {chars: common_prefix, children: HashMap::from([(new_child.chars.chars().next().unwrap(), new_child)]), is_word: true};
                let _ = root.children.insert(insertion_word.chars().next().unwrap(), parent);
                return Ok(());
            }
            else {
                // Search further
                return insert(child, insertion_word.chars().take(common_prefix.len()).collect());
            }

        },
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
