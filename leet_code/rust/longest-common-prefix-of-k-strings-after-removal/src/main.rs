//https://leetcode.com/problems/longest-common-prefix-of-k-strings-after-removal/
struct Solution;

use std::collections::HashMap;

#[derive(Clone, Debug)]
struct NodePass1 {
    cnt: i32,
    nodes: Vec<Option<Box<NodePass1>>>,
}

#[derive(Clone)]
struct Node {
    cnt: i32,
    nodes: HashMap<u8, Node>,
}

impl Solution {
    fn search_in_trie(
        search_word: &Vec<u8>,
        level: usize,
        k: i32,
        trie: &Node,
    ) -> usize {
        if level < search_word.len()
            && let Some(node) = trie.nodes.get(&search_word[level])
        {
            if node.cnt >= k {
                return Self::search_in_trie(search_word, level + 1, k, &node);
            }
        }
        level 
    }

    fn build_trie(word: &String, mut trie: &mut Node) {
        for &c in word.as_bytes() {
            trie.nodes
                .entry(c)
                .and_modify(|e| {
                    e.cnt += 1;
                })
                .or_insert(Node {
                    cnt: 1,
                    nodes: HashMap::new(),
                });
            trie = trie.nodes.get_mut(&c).unwrap();
        }
    }

    fn find_trie(word: &String, k: i32, mut trie: &Node) -> i32 {
        let mut level = 0;
        for c in word.as_bytes() {
            if let Some(node) = trie.nodes.get(c) && node.cnt >= k {
                level+=1;
                trie = trie.nodes.get(&c).unwrap();
            }
        }
        level
    }
    
    fn remove_trie(word: &String, mut trie: &mut Node) {
        for &c in word.as_bytes() {
            trie.nodes
                .entry(c)
                .and_modify(|e| {
                    e.cnt -= 1;
                });
            trie = trie.nodes.get_mut(&c).unwrap();
        }
    }

    pub fn longest_common_prefix(words: Vec<String>, k: i32) -> Vec<i32> {
        let mut trie = Node {
            cnt: 0,
            nodes: HashMap::new(),
        };

        let mut result: Vec<i32> = Vec::new();
        for word in words.iter() {
            Self::build_trie(&word, &mut trie);
        }

        let mut memory_a: HashMap<&String, usize> = HashMap::new();
        let mut memory: HashMap<(&String, &String), usize> = HashMap::new();

        for i in 0..words.len() {
            let mut lcp_a = 0;
            result.push(*memory_a.entry(&words[i]).or_insert_with(|| {
                Self::remove_trie(&words[i], &mut trie);
                for j in 0..words.len() {
                    if i != j {
                        lcp_a = lcp_a.max(*memory.entry((&words[i],&words[j])).or_insert_with(|| {
                            let loop_s = Self::find_trie(
                                &words[j],
                                k,
                                &mut trie,
                            );
                            //println!("{rec_s} == {loop_s} -> {}", rec_s == loop_s as usize);
                            loop_s as usize
                        }));
                    }
                }
                Self::build_trie(&words[i], &mut trie);
                lcp_a
            }) as i32);
        }
        result
    }
}

struct SolutionPass1;
impl SolutionPass1 {
    fn search_in_trie(
        search_word: &Vec<char>,
        level: usize,
        k: i32,
        skip_word: &Vec<char>,
        mut trie: &Option<Box<NodePass1>>,
    ) -> (i32, i32, Vec<char>) {
        let mut subset_s: Vec<char> = Vec::new();
        let mut matches = i32::MAX;
        let mut ml = level as i32;
        //println!("Search reached to level = {level}");
        if level < search_word.len()
            && let Some(curr_node) = trie
        {
            if let Some(ref node) = curr_node.nodes[search_word[level] as usize - 97] {
                println!(
                    " Curr letter {} Curr_node count {} >= k {k}",
                    String::from_iter(vec![search_word[level]]),
                    node.cnt
                );

                let curr_node_count = node.cnt
                    - if level < skip_word.len() && skip_word[0..=level] == search_word[0..=level] {
                        1
                    } else {
                        0
                    };
                println!(
                    " Curr letter {} Curr_node count {} >= k {k}",
                    String::from_iter(vec![search_word[level]]),
                    curr_node_count
                );

                if curr_node_count >= k {
                    println!(
                        " Lets seach for the first character -> {}",
                        search_word[level] as u8 - 97
                    );
                    trie = &curr_node.nodes[search_word[level] as usize - 97];
                    let (mut m, mut l, suffixes) =
                        Self::search_in_trie(search_word, level + 1, k, skip_word, trie);
                    if suffixes.len() == 0 {
                        println!("I never come here now!");
                        subset_s.push(search_word[level]);
                        m = curr_node_count;
                        println!(" Tail count : {m}");
                    } else {
                        let mut sfx = vec![search_word[level]];
                        sfx.extend(suffixes);
                        subset_s = sfx;
                    }
                    ml = l as i32;
                    matches = matches.min(m);
                } else {
                    ml = level as i32;
                }
            }
        }
        (matches, ml, subset_s)
    }

    fn print_trie(mut trie: &Option<Box<NodePass1>>) -> Vec<Vec<char>> {
        let mut subset_s: Vec<Vec<char>> = Vec::new();
        if let Some(curr_node) = trie {
            for (i, node) in curr_node.nodes.iter().enumerate() {
                if node.is_some() {
                    trie = node;
                    let suffixes = Self::print_trie(trie);
                    if suffixes.len() == 0 {
                        subset_s.push(vec![(i + 97) as u8 as char]);
                    } else {
                        for suffix in Self::print_trie(trie) {
                            let mut sfx = vec![(i + 97) as u8 as char];
                            sfx.extend(suffix);
                            subset_s.push(sfx);
                        }
                    }
                }
            }
        }
        subset_s
    }

    fn decode_trie(mut trie: &Option<Box<NodePass1>>) {
        //println!("## decode");
        let mut s: Vec<char> = Vec::new();
        while let Some(curr_node) = trie {
            for (i, node) in curr_node.nodes.iter().enumerate() {
                if node.is_some() {
                    println!(" {i} {node:?}");
                    s.push((i + 97) as u8 as char);
                    trie = node;
                    break;
                } else {
                    trie = &None;
                }
            }
        }
        println!("Decoded -> {s:?} {}", String::from_iter(&s));
    }

    fn build_tree(word: &Vec<u8>, trie: &mut Option<Box<NodePass1>>) {
        //    let mut container = Some(Box::new(trie));
        let mut curr_node = trie.as_mut();
        //println!("Adding to trie : {word}");
        for c in word {
            if let Some(node) = curr_node {
                if let Some(next_node) = node.nodes[(c - 97) as usize].as_mut() {
                    next_node.cnt += 1;
                } else {
                    let next_node = Some(Box::new(NodePass1 {
                        cnt: 1,
                        nodes: vec![None; 26],
                    }));
                    node.as_mut().nodes[(c - 97) as usize] = next_node;
                }
                curr_node = node.as_mut().nodes[(c - 97) as usize].as_mut();
            }
        }
    }

    pub fn longest_common_prefix(words: Vec<String>, k: i32) -> Vec<i32> {
        println!("--- Words = {words:?}\n--- k={k}");
        let mut word_freq: HashMap<String, i32> = HashMap::new();
        let mut trie = Some(Box::new(NodePass1 {
            cnt: 0,
            nodes: vec![None; 26],
        }));

        let mut result: Vec<i32> = Vec::new();
        for word in words.iter() {
            Self::build_tree(&word.as_bytes().to_vec(), &mut trie);
        }

        let mut memory: HashMap<(&String, &String), i32> = HashMap::new();
        for (i, skip_word) in words.iter().enumerate() {
            println!("##");
            println!("## Skip -> {skip_word}");
            let mut lcp = 0;
            for (j, search_word) in words.iter().enumerate() {
                if i != j {
                    println!("\tSearch -> {search_word}");
                    let e = memory.entry((skip_word, search_word)).or_insert_with(|| {
                        println!("Not found");
                        let s = Self::search_in_trie(
                            &search_word.chars().collect(),
                            0,
                            k,
                            &skip_word.chars().collect(),
                            &trie,
                        );
                        if s.0 >= k {
                            println!(
                                "\tMatched String: {} Matches: {} Match length: {}",
                                String::from_iter(&s.2),
                                s.0,
                                s.1
                            );
                            s.1
                        } else {
                            0
                        }
                    });
                    lcp = lcp.max(*e);
                }
            }
            println!("Max match: {lcp}");
            result.push(lcp);
        }
        Self::decode_trie(&trie);
        result
    }
}
fn main() {
    println!("Hello, world!");
}
#[cfg(test)]
mod tests {
    fn testcases() -> Vec<(Vec<String>, i32, Vec<i32>)> {
        vec![
            (
                vec![
                    "dogish".to_string(),
                    "racer".to_string(),
                    "race".to_string(),
                    "dog".to_string(),
                    "rack".to_string(),
                    "caramel".to_string(),
                    "car".to_string(),
                ],
                2,
                vec![4, 3, 3, 4, 4, 4, 4],
            ),
            (
                vec![
                    "jump".to_string(),
                    "run".to_string(),
                    "run".to_string(),
                    "jump".to_string(),
                    "run".to_string(),
                ],
                2,
                vec![3, 4, 4, 3, 4],
            ),
            (
                vec!["dog".to_string(), "racer".to_string(), "car".to_string()],
                2,
                vec![0, 0, 0],
            ),
            (
                vec![
                    "ccd".to_string(),
                    "adc".to_string(),
                    "dba".to_string(),
                    "bff".to_string(),
                    "cbfae".to_string(),
                    "fcae".to_string(),
                    "cbbc".to_string(),
                ],
                3,
                vec![0, 1, 1, 1, 0, 1, 0],
            ),
            (
                vec!["jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjika".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjidaikjkkjgajc".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjiddkdj".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjii".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjibhbdcfjibfke".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjijfj".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjibeeg".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicji".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjiddckgich".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjiidcjedgaeicd".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjiagfi".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjicafiijihehkh".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjihijkbj".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjifcbjjfhjh".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjifaabkdgjccd".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjigcafkbhgj".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjifkacbgefkfkcdk".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjicbdbhfbcffjhbf".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjihgjg".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjiecf".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjiabddff".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjiecheadifj".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicji".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjigaidk".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjidbafgjg".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjibchgadfbdjfikhj".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjiaiekibjkgkfed".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjibkbdjege".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjigfgahibbbjcfj".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjikefjdhdiadg".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjidkadeifgeed".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjikchkbgb".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjibkdaighfcadfefc".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicji".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjifgiegcji".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjieaggheafdbfe".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjibedceiba".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjiefige".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjiccfgcghheifkkj".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjie".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjichd".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjikcgbckgbfcffbb".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjihikcjc".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjibg".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjihfejgjafbdjbfd".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjibebbjdgejichddc".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjiib".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjidkifekjchgjhg".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicji".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjiakkc".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjihkffca".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjijffhgjf".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjikebhabebe".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjifhgfccdcjifdcdc".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjidkcbckbfchbf".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjicfbbjdbcb".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicji".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjiikfdadgdfajebck".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjihkffkcihahkk".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjiaa".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjifhka".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjiafeacab".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjicabaccbkgiif".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjickgdcgkeikbec".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjifibbiccegi".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjibaj".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjiccjfakjkciee".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjia".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjiefjdk".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjiagfabigfdjfkdi".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjificffeijjibej".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjigjgdfidafcfca".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjii".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjiackiiieekbdki".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjijkdkijbaekjb".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjigiefigaifg".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjiaaaigi".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicji".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjiidkhhekjhga".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjigddbhdhjhg".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjiaikkekkgheicijb".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjicdgdaacaefj".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjifcaccbihkk".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjidi".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjibckhajeheb".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjihh".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjibefhee".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjibkfabefkfbakdh".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjickcbjakbgf".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicji".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjiidk".to_string(),
"jbbbccghdhckicffdfgibkejkfdbkkejfaakjgcdbjjajhhgejgjhbbcbkcbfhdickhkadccifdejeigcjgjbfaeiikikjacdbhfjaajkkgdieebgkbdidddehcedfadggfghickaehddhibkfahehhgbcjjjaebdikkfgbgjekgidikhahgbiakekgjekbddbfkfbgdhdafddghbgjkdcgefkifcjcdabjdaiedkfdeckbadkjcaihedaaghhfekgabhegbjbhacbceaiaibhbfchjdibiehfiiajedjecdjhhgkhgjahidfieiabhebkcjihfggheecdjckeeehcjaeffhghhkbhcbdgfegjeggkicbaefggiaafiaccdecjikeidcgjeaiibbhhdhafjahcgijakccjabbgeadadbaeidedfdigfcddaibcdgicfdkkjjfccefhikkikfhcgikjgkahgedkijcaabfjjgehgekhibejgeakhdehkjgccjcfdedgkfcahghjabekififkhdaaaeffaefcbeafedkkhgkkebjhiackikchafghabhgkdabidfcjejcejfhicefhgihhkheeigbcejacgjgbgcachjhbffjaaagfiebiajdijdiccgkkdaagfjhbcgeiafjijgcafhbkbkiagcjjjijcfgkidkjeafhjdckjfcgadekhfikggkiikkfjjeadjdgjgdfcbbbeahcghkdhgffkbehigdkfaegbjedhgjgecceegkafchahdbcchfeafkdbeijgjfkfdecdgkeakhbkhbdadicjij".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgaj".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgajhkgdjhibakfhe".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgakekjbfcbhke".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkga".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgakehg".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgahiifahdfejjca".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgaekfbhkjjdj".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgai".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgajbhifkihkje".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgaihjcaab".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkga".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgafhfcafjb".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgaicfddidkdid".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgajjbcgebdacic".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgadbj".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgahdbjdbj".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgaaebfg".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgae".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgaffifh".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgafchbgb".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgahcidi".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgakffjk".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgacj".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgahdd".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgadgc".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgacegfja".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgagbii".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgacakghbefhiejci".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgadidfbibacd".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgadfj".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgafdbae".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgaebjf".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgakakhdggbcccheik".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgafiecijjfjb".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgaa".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgafiikie".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgahddehj".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgac".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgahcjdecbckckc".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgakafedhcifggf".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgadkcid".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgacci".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkga".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgaaiihhfa".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgahahakgbejhadaj".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgahg".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgadje".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgadhhfjecddckg".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgadbbcfikfcech".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgacbbjkkbga".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgaagbdb".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgachfeifdheiakch".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgadce".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgacbfkhfk".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgakbig".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgajiibd".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgafk".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgaak".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgaeih".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgajfaijf".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgaddidaidcf".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgakjjakkhkkdg".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgadhkc".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgaihechdchj".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgagaghach".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgahefeahjiiagb".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgabdcedgjhhj".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgakfhcjchejdicbe".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkga".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgacgdkbbibicj".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgahkajdiegeh".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgajkchfbgca".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgakh".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgaecbjhag".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgafdhcc".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgaii".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgagbcbifgkdbbjkk".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgafje".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgaaegjhbcjkfajhek".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgabhdcchh".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgag".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgabbdbbgjgaegehaa".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgakckice".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgabkfcijcagbihc".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgabhaagibajkffbff".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgak".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgagijba".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgaieibccefabja".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgafeedhbfcbk".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkga".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgabaecjdkhid".to_string(),
"bdajgjegaecjbhabhhaekgffcjjgdfdijeaigbaihbjhfkcjfkgbdkcdjijcbcekiafebdgcfcfghikbhbaegkkfkgecajikbiicghcaigjhcfhkgfjdfbifdccgghieagfgkjfbcjigagibkbaedecaafcgcbbbicdekdcchcccjfcfijkakckihkkdkjciiddfhdghfiaiiebbcajggkgagegeifeiacjag".to_string(),
"ghkdedhifbhiikah".to_string(),
"ghkdedkffigigeg".to_string(),
"ghkdedaedbkcadecgjh".to_string(),
"ghkdedkcjgeh".to_string(),
"ghkdedgjc".to_string(),
"ghkdedigkkbikfdeagdaj".to_string(),
"ghkdedjfabedjggkged".to_string(),
"ghkdeda".to_string(),
"ghkdedikbkcjkckijcbk".to_string(),
"ghkdedagicbajickidhj".to_string(),
"ghkdedaiidagk".to_string(),
"ghkdedgjdiha".to_string(),
"ghkdedfhb".to_string(),
"ghkdedeaheeji".to_string(),
"ghkdedikjbkacjbhbib".to_string(),
"ghkdedg".to_string(),
"ghkdedjiacgiakd".to_string(),
"ghkdeddgfjebdgf".to_string(),
"ghkdedikiadcbhkh".to_string(),
"ghkdedbhjcbgjj".to_string(),
"ghkdedefkbffb".to_string(),
"ghkdedfegfejddgaeb".to_string(),
"ghkdedb".to_string(),
"ghkdedbjdigif".to_string(),
"ghkdedbec".to_string(),
"ghkdedkbed".to_string(),
"ghkdedcaifeeg".to_string(),
"ghkdedaef".to_string(),
"ghkdedak".to_string(),
"ghkded".to_string(),
"ghkdedbjhaefjeied".to_string(),
"ghkdedkgcif".to_string(),
"ghkdedgdbk".to_string(),
"ghkded".to_string(),
"ghkdedf".to_string(),
"ghkded".to_string(),
"ghkdediejgee".to_string(),
"ghkdedfh".to_string(),
"ghkdedkbaci".to_string(),
"ghkdedjjachddeff".to_string(),
"ghkdedbijhdkdbci".to_string(),
"ghkdedg".to_string(),
"ghkdedagiffjfkcee".to_string(),
"ghkdedcaehkji".to_string(),
"ghkdedhgegeejeh".to_string(),
"ghkdedi".to_string(),
"ghkdedfcgke".to_string(),
"ghkdedddiiig".to_string(),
"ghkdedicbc".to_string(),
"ghkdedaghdigbdh".to_string(),
"ghkdedkkdbckchkadiia".to_string(),
"ghkdedjaih".to_string(),
"ghkdedgddccc".to_string(),
"ghkdedekckki".to_string(),
"ghkdedakfadbkikhadifg".to_string(),
"ghkdedeihick".to_string(),
"ghkdedhbkih".to_string(),
"ghkdedhe".to_string(),
"ghkded".to_string(),
"ghkdedgigf".to_string(),
"ghkdedjhgdid".to_string(),
"ghkdeddgkafc".to_string(),
"ghkdedd".to_string(),
"ghkdedef".to_string(),
"ghkdedgihbjdjhckhhg".to_string(),
"ghkdedhkdhgjagfhk".to_string(),
"ghkdedibjgjkedbiiaa".to_string(),
"ghkdedicgaiegehdc".to_string(),
"ghkdeddicijk".to_string(),
"ghkdeddfcjiikjigi".to_string(),
"ghkdeddbddbhbibffgfei".to_string(),
"ghkdedjg".to_string(),
"ghkdedgdbfbe".to_string(),
"ghkdedhe".to_string(),
"ghkdedkjgb".to_string(),
"ghkdedfiejgk".to_string(),
"ghkdedbjkgdk".to_string(),
"ghkdedgj".to_string(),
"ghkdedjebddjjhhkdbf".to_string(),
"ghkdedgbhfjbkecdbijca".to_string(),
"ghkdedijehkkifkecfdff".to_string(),
"ghkdedebiachd".to_string(),
"ghkdedahcjdjahf".to_string(),
"ghkdediccbhhcge".to_string(),
"ghkdedbhjdckhkbe".to_string(),
"ghkdedf".to_string(),
"ghkdedkehjhedd".to_string(),
"ghkdedaciddhjfka".to_string(),
"ghkdedcahfiadbkdih".to_string(),
"ghkdeddidjffhbbejckba".to_string(),
"ghkdedaafhhjhcikiekb".to_string(),
"ghkdedcaabgeegc".to_string(),
"ggdadeihcacihjhiagg".to_string(),
"ggdadacgidgbcfjdjb".to_string(),
"ggdabjckhahhdbcbbk".to_string(),
"ggdaadbihfaai".to_string(),
"ggdadgihkkfbcke".to_string(),
"ggdajbfkdgbakek".to_string(),
"ggdagkd".to_string(),
"ggdaie".to_string(),
"ggdabkk".to_string(),
"ggdak".to_string(),
"ggdakkjcjdaedekbef".to_string(),
"ggdabdc".to_string(),
"ggdabcjjighijhkeff".to_string(),
"ggdadhgi".to_string(),
"ggdabcfiheaejghj".to_string(),
"ggdagb".to_string(),
"ggda".to_string(),
"ggdaebhedkhkcdabbb".to_string(),
"ggdacadjjccchabck".to_string(),
"ggdaaacjge".to_string(),
"ggdaikfghd".to_string(),
"ggdaabche".to_string(),
"ggdadbagfkakkei".to_string(),
"ggdaijdhggd".to_string(),
"ggdahd".to_string(),
"ggdab".to_string(),
"ggda".to_string(),
"ggdaki".to_string(),
"ggdagdied".to_string(),
"ggdaefefihhfhicb".to_string(),
"ggdadikhfbjdgbabchf".to_string(),
"ggdagbabb".to_string(),
"ggdaehhfcabgk".to_string(),
"ggdab".to_string(),
"ggdajbfhbaihieejbic".to_string(),
"ggdacccfjabigjjcfeb".to_string(),
"ggdajchk".to_string(),
"ggdadfbca".to_string(),
"ggdaachgdfk".to_string(),
"ggdadediabgkkgg".to_string(),
"ggdai".to_string(),
"ggda".to_string(),
"ggdakhbc".to_string(),
"ggdadfdedb".to_string(),
"ggda".to_string(),
"ggdafjhcifj".to_string(),
"ggdaa".to_string(),
"ggdabaafhigjbjdfbe".to_string(),
"ggdaikkfghhb".to_string(),
"ggdaikfie".to_string(),
"ggdabkfacbbhd".to_string(),
"ggdadieikih".to_string(),
"ggdaf".to_string(),
"ggdaggcja".to_string(),
"ggdafciaij".to_string(),
"ggdacgccdj".to_string(),
"ggdagachdiefiajh".to_string(),
"ggdaijigajbfih".to_string(),
"ggdadkgkkkhbc".to_string(),
"ggdabgeiihbgjfeabg".to_string(),
"ggdakcciad".to_string(),
"ggdaiaehejcddfja".to_string(),
"ggdafchjhcdkhgh".to_string(),
"ggdai".to_string(),
"ggdakkgbfhk".to_string(),
"ggdakdbafgfhji".to_string(),
"ggdabbff".to_string(),
"ggdaf".to_string(),
"ggdakiggigbhedgga".to_string(),
"ggdahdjibcfebj".to_string(),
"ggdahea".to_string(),
"ggdakcc".to_string(),
"ggdajdfkdk".to_string(),
"ggdahkkik".to_string(),
"ggdaigbaceih".to_string(),
"ggdajhiejfacji".to_string(),
"ggdaihecccaeabihf".to_string(),
"ggdaedaaji".to_string(),
"ggdajcdd".to_string(),
"ggdaicediffei".to_string(),
"ggdagbk".to_string(),
"ggdakiaj".to_string(),
"ggdah".to_string(),
"ggdackjajejicbijj".to_string(),
"ggdadbb".to_string(),
"ggdahh".to_string(),
"ggdagaegaehjdhkeac".to_string(),
"ggdaebjgbjiceei".to_string(),
"ggdaedfhhjjcaaeabd".to_string(),
"ggdadh".to_string(),
"ggdaiafjciigffik".to_string(),
"ggdabjh".to_string(),
"abajigdgbeaei".to_string(),
"aakbcicdfgbhka".to_string(),
"ahchda".to_string(),
"aagagieabdk".to_string(),
"abbaj".to_string(),
"addeggkefcfb".to_string(),
"a".to_string(),
"acbkahagjja".to_string(),
"abieejbahkei".to_string(),
"acfjdhcjkghgddkk".to_string(),
"agejdkegbediajaa".to_string(),
"afbhjgbkbdgfkie".to_string(),
"abkcbcadjiific".to_string(),
"ad".to_string(),
"ahfkghghgiaagec".to_string(),
"acaddk".to_string(),
"a".to_string(),
"adcbjjac".to_string(),
"acigjjdek".to_string(),
"akeeeb".to_string(),
"ad".to_string(),
"aij".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string(),
"a".to_string()],
                92,
                vec![216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,216,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829,829],
            )
        ]
    }

    use super::Solution;
    #[test]
    fn test_longest_common_prefix() {
        for (words, k, want) in testcases() {
            assert_eq!(Solution::longest_common_prefix(words, k), want);
        }
    }
}
