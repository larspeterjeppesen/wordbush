// use anyhow::Result;
use std::iter::zip;
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Radix {
    chars: String,
    children: HashMap<char, Radix>,
    is_word: bool,
}

impl Radix {
    pub fn new() -> Radix {
        Radix {
            chars: String::from(""),
            children: HashMap::new(),
            is_word: false,
        }
    }


    pub fn lookup<'a>(&'a self, key: &String) -> Result<(), &'a str> {
        let lower_key = key.to_lowercase();
        let common_prefix: String = zip(self.chars.chars(), lower_key.chars())
            .take_while(|(a,b)| a == b)
            .map(|(a,_)| a)
            .collect();

        if lower_key == self.chars {
            if !self.is_word {
                return Err("Key found, but is not marked as a word");
            }
            return Ok(());
        }

        if (common_prefix.len() == 0 && self.chars.len() != 0) || 
            (common_prefix.len() == lower_key.len()) {
            return Err("Key not found in tree");
        }


        //Partial match found, search further
        let subkey = lower_key[common_prefix.len()..].to_string();
        let child_node_match = self.children.get(&subkey.chars().next().unwrap());
        match child_node_match {
            Some(child) => child.lookup(&subkey),
            None => return Err("Key not found in tree"),
        }
    }

    pub fn insert(&mut self, insertion_word: String) -> Result<(), Box<dyn std::error::Error>> {
        let next_node: Option<&mut Radix> = self.children.get_mut(&insertion_word.chars().next().unwrap());
        match next_node {
            None => {
                // no match, insert node
                let child_key: char = insertion_word.chars().next().unwrap();
                let child_node = Radix {
                    chars: insertion_word,
                    children: HashMap::new(),
                    is_word: true,
                };
                self.children.insert(child_key, child_node);
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
                    self.children.insert(new_parent.chars.chars().next().unwrap(), new_parent);             

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
                    let _ = self.children.insert(
                        insertion_word.chars().next().unwrap(), 
                        parent);
                    return Ok(());
                }
                else {
                    // Search further
                    let sub_insertion_word: String = insertion_word.chars().skip(common_prefix.len()).collect();
                    return child.insert(sub_insertion_word);
                }

            },
        }
    }
}



#[cfg(test)]
mod tests {
    use super::*;
    use std::io::prelude::*;
    use std::io::BufReader;
    use std::fs::File;
    use std::io::SeekFrom;

    #[test]
    fn tree_insert_one_word() {
        let mut root = Radix {
            chars: String::from(""),
            children: HashMap::new(),
            is_word: false,
        };

        let _ = root.insert(String::from("hej"));
        let child: Option<&Radix> = root.children.get(&'h');
        assert_eq!(child.is_some_and(|w| w.chars == "hej"), true);        
    }

    #[test]
    fn tree_insert_parent_then_child() {
        let mut root = Radix::new();

        let _ = root.insert(String::from("hej"));
        let _ = root.insert(String::from("hejsa"));


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

        let _ = root.insert(String::from("hejsa"));
        let _ = root.insert(String::from("hej"));

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

        let _ = root.insert(String::from("hejsa"));
        let _ = root.insert(String::from("hejse"));
        
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

        let _ = root.insert(String::from("hejsa"));
        let _ = root.insert(String::from("hejse"));
        let _ = root.insert(String::from("hejs"));

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

        let _ = root.insert(String::from("hej"));
        let res = root.lookup(&String::from("hej"));
        assert_eq!(
            res,
            Ok(()),
            "Failed with tree structure: {root:?}"
        );
    }

    #[test]
    fn tree_lookup_negative() {
        let mut root = Radix::new();
        
        let _ = root.insert(String::from("hej"));
        let res = root.lookup(&String::from("hehe"));
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
            let _ = root.insert(word);

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
            let res = root.lookup(&word);

            assert_eq!(res, Ok(()));

            if i == max {
                break;
            }
        }
    }


}