use std::{
    fs::{self, File},
    io::Write,
};

const LETTERS: [char; 12] = ['s', 'l', 'c', 'w', 'i', 'j', 'a', 'g', 'y', 'k', 'o', 'n'];

pub struct Word {
    pub string: String,
    pub unique_letters: Vec<char>,
}

impl Word {
    pub fn new(string: String) -> Self {
        let mut unique_letters = string.chars().collect::<Vec<char>>();
        unique_letters.sort();
        unique_letters.dedup();

        Self {
            string,
            unique_letters,
        }
    }
}

pub fn prune_dictionary(words: Vec<&str>) -> Vec<&str> {
    words
        .into_iter()
        // remove all words of len 3 or less
        .filter(|word| word.len() > 3)
        // remove all words with non-alpha characters
        .filter(|word| word.chars().all(|char| char.is_alphabetic()))
        // remove all words with repeated characters
        .filter(|word| {
            let mut chars = word.chars();
            let mut last_char = chars.next().unwrap();
            for char in chars {
                if char == last_char {
                    return false;
                }
                last_char = char;
            }
            return true;
        })
        .collect::<Vec<&str>>()
}

pub fn graph(word_list: &Vec<Word>) -> [Vec<usize>; 26] {
    let mut graph: [Vec<usize>; 26] = Default::default();

    for (i, word) in word_list.iter().enumerate() {
        let last_char = word.string.chars().next().unwrap();
        let char_index = last_char as usize - 'a' as usize;

        graph[char_index].push(i);
    }

    return graph;
}

pub fn shortest_path(
    graph: &[Vec<usize>; 26],
    words: &Vec<Word>,
    max_depth: usize,
) -> Option<Vec<usize>> {
    let mut current_depth = (0..words.len())
        .map(|usize| vec![usize])
        .collect::<Vec<Vec<usize>>>();
    let mut next_depth = Vec::new();

    for _ in 0..max_depth {
        while let Some(path) = current_depth.pop() {
            let mut letters = Vec::new();
            for i in path.clone() {
                letters.append(&mut words[i].unique_letters.clone());
            }
            letters.sort_unstable();
            letters.dedup();

            if letters.len() >= 12 {
                return Some(path);
            }

            let last_char = words[*path.last().unwrap()].string.chars().last().unwrap();
            for neighbor in &graph[last_char as usize - 'a' as usize] {
                let mut path = path.clone();
                path.push(*neighbor);
                next_depth.push(path);
            }
        }

        current_depth = next_depth;
        next_depth = Vec::with_capacity(current_depth.len());
    }

    return None;
}

pub fn shortest_paths(
    graph: &[Vec<usize>; 26],
    words: &Vec<Word>,
    max_depth: usize,
) -> Vec<Vec<usize>> {
    let mut current_depth = (0..words.len())
        .map(|usize| vec![usize])
        .collect::<Vec<Vec<usize>>>();
    let mut next_depth = Vec::new();

    let mut paths = Vec::new();
    let mut found = false;
    for _ in 0..max_depth {
        while let Some(path) = current_depth.pop() {
            let mut letters = Vec::new();
            for i in path.clone() {
                letters.append(&mut words[i].unique_letters.clone());
            }
            letters.sort_unstable();
            letters.dedup();

            if letters.len() >= 12 {
                found = true;
                paths.push(path.clone());
            }

            let last_char = words[*path.last().unwrap()].string.chars().last().unwrap();
            for neighbor in &graph[last_char as usize - 'a' as usize] {
                let mut path = path.clone();
                path.push(*neighbor);
                next_depth.push(path);
            }
        }

        if found {
            break;
        }

        current_depth = next_depth;
        next_depth = Vec::with_capacity(current_depth.len());
    }

    return paths;
}

fn main() {
    let args: Vec<String> = std::env::args().collect();
    if args.len() > 1 {
        let dictionary = fs::read_to_string("dictionary.txt").unwrap();
        let words = prune_dictionary(dictionary.lines().collect::<Vec<&str>>());

        let mut file = File::create("pruned.txt").unwrap();
        file.write_all(words.join("\n").as_bytes()).unwrap();
    };

    let pruned = fs::read_to_string("pruned.txt").unwrap();
    let words = pruned
        .lines()
        .filter(|line| {
            line.chars().collect::<Vec<char>>().windows(2).all(|slice| {
                let a = slice[0];
                let b = slice[1];

                let Some(a_i) = LETTERS.iter().position(|char| *char == a) else {
                    return false;
                };
                let Some(b_i) = LETTERS.iter().position(|char| *char == b) else {
                    return false;
                };

                // if a and b are on the same edge
                if a_i / 3 == b_i / 3 {
                    return false;
                }

                return true;
            })
        })
        .map(|str| Word::new(str.to_string()))
        .collect::<Vec<Word>>();

    let graph = graph(&words);
    // let path = shortest_path(&graph, &words, 5);

    // let Some(path) = path else {
    //     println!("No path found");
    //     return;
    // };

    // println!(
    //     "{}",
    //     path.into_iter()
    //         .map(|i| words[i].string.clone())
    //         .collect::<Vec<String>>()
    //         .join(" ")
    // );

    let paths = shortest_paths(&graph, &words, 5);
    println!(
        "{}",
        paths
            .into_iter()
            .map(|path| path
                .into_iter()
                .map(|i| words[i].string.clone())
                .collect::<Vec<String>>()
                .join(" "))
            .collect::<Vec<String>>()
            .join("\n")
    )
}
