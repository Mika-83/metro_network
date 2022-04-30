use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, Clone, PartialEq)]
pub struct Node {
    pub kanji: String,
    pub kana: String,
    pub roman: String,
    pub shozoku: String,
}

pub fn read(path: &str) -> BufReader<File> {
    let f = File::open(path).unwrap();
    BufReader::new(f)
}

pub fn read_node(reader: BufReader<File>) -> Vec<Node> {
    let mut lines: Vec<Node> = Vec::new();
    for (i, line_) in reader.lines().enumerate() {
        if i == 0 {
            continue;
        }
        let line = line_.unwrap();
        let l: Vec<&str> = line.split(',').collect();
        lines.push(Node {
            kanji: String::from(l[0]).replace(' ', ""),
            kana: String::from(l[1]).replace(' ', ""),
            roman: String::from(l[2]).replace(' ', ""),
            shozoku: String::from(l[3]).replace(' ', ""),
        })
    }
    lines
}
#[derive(Debug, Clone)]
pub struct Edge {
    pub tail: String,
    pub head: String,
    pub line: String,
    pub dist: f32,
    pub time: usize,
}

pub fn read_edge(reader: BufReader<File>) -> Vec<Edge> {
    let mut lines : Vec<Edge> = Vec::new();
    for (i, line_) in reader.lines().enumerate() {
        if i == 0 {
            continue;
        }
        let line = line_.unwrap();
        let l: Vec<&str> = line.split(',').collect();
        lines.push(Edge{
            tail: l[0].replace(' ', ""),
            head: l[1].replace(' ', ""),
            line: l[2].replace(' ', ""),
            dist: l[3].replace(' ', "").parse::<f32>().unwrap(),
            time: l[4].replace(' ', "").parse().unwrap(),
        });
    }
    lines
}