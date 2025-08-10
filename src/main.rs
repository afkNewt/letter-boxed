use petgraph::{Directed, Graph, graph::NodeIndex};
use std::{
    fs::{self, File},
    io::Write,
};

const LETTERS: [char; 12] = ['s', 'l', 'c', 'w', 'i', 'j', 'a', 'g', 'y', 'k', 'o', 'n'];

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

pub fn graph(word_list: &Vec<&str>) -> Graph<Vec<char>, ()> {
    let mut graph =
        Graph::<Vec<char>, (), Directed>::with_capacity(word_list.len(), word_list.len() * 100);

    for word in word_list.iter() {
        let mut unique = word.chars().collect::<Vec<char>>();
        unique.sort_unstable();
        unique.dedup();

        graph.add_node(unique);
    }

    for (i, word) in word_list.iter().enumerate() {
        let end_char = word.chars().last().unwrap();
        for (j, word) in word_list.iter().enumerate() {
            let start_char = word.chars().next().unwrap();
            if end_char == start_char {
                graph.add_edge(NodeIndex::new(i), NodeIndex::new(j), ());
            }
        }
    }

    graph.shrink_to_fit();

    return graph;
}

pub fn shortest_path(graph: &Graph<Vec<char>, ()>, max_depth: usize) -> Option<Vec<NodeIndex>> {
    let mut current_depth = graph
        .node_indices()
        .map(|index| vec![index])
        .collect::<Vec<Vec<NodeIndex>>>();
    let mut next_depth = Vec::new();

    for _ in 0..max_depth {
        while let Some(path) = current_depth.pop() {
            let mut letters = Vec::new();
            for index in &path {
                letters.append(&mut graph.node_weight(*index).unwrap().to_owned());
            }
            letters.sort_unstable();
            letters.dedup();

            if letters.len() >= 12 {
                return Some(path);
            }

            for neighbor in graph.neighbors(*path.last().unwrap()) {
                let mut path = path.clone();
                path.push(neighbor);
                next_depth.push(path);
            }
        }

        current_depth = next_depth;
        next_depth = Vec::with_capacity(current_depth.len());
    }

    return None;
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
        .collect::<Vec<&str>>();

    let graph = graph(&words);
    println!(
        "words: {}\nedges: {}",
        graph.node_count(),
        graph.edge_count()
    );
    let path = shortest_path(&graph, 5);

    let Some(path) = path else {
        println!("No path found");
        return;
    };
    println!(
        "Shortest path: {}",
        path.into_iter()
            .map(|index| words[index.index()])
            .collect::<Vec<&str>>()
            .join(" ")
    );
}
