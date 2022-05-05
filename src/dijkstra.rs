use crate::model::Node;

#[derive(Debug, PartialEq, Clone)]
pub struct EkiT {
    pub name: String,
    pub shortest_dist: f32,
    pub prevs: Vec<String>,
}

impl EkiT {
    pub fn new(name: String) -> EkiT {
        EkiT{
        name: name,
        shortest_dist: f32::INFINITY,
        prevs: Vec::<String>::new()}
    }
}

fn make_eki_list(lst: &Vec<Node>) -> Vec<EkiT> {
    match lst.len() {
        0 => Vec::<EkiT>::new(),
        _ => {
            let mut l: Vec<EkiT> = vec![EkiT::new(lst[0].kanji.clone())];
            l.append(&mut make_eki_list(&lst[1..].to_vec()));
            l
        }
    }
}

fn shokika(mut lst: Vec<EkiT>, name: &String) -> Vec<EkiT> {
    match lst.len() {
        0 => Vec::new(),
        _ => {
            if &lst[0].name == name {
                lst[0].shortest_dist = 0.0;
                lst[0].prevs = vec![name.clone()];
                lst
            } else {
                let mut l = vec![lst[0].clone()];
                l.append(&mut shokika(lst[1..].to_vec(), name));
                l
            }
        }
    }
}

#[warn(dead_code)]
fn insert_node(n: Node, mut lst: Vec<Node>) -> Vec<Node> {
    if lst.is_empty() {
        vec![n]
    } else if lst[0].kana == n.kana || lst[lst.len() - 1].kana == n.kana {
        insert_node(lst[lst.len() - 1].clone(), lst[..lst.len() - 1].to_vec())
    } else if lst[0].kana > n.kana {
        let mut answer = vec![n];
        answer.append(&mut lst);
        answer
    } else if lst[lst.len() - 1].kana < n.kana {
        lst.push(n);
        lst
    } else {
        let mut answer = vec![lst[0].clone()];
        answer.append(&mut insert_node(n, lst[1..].to_vec()));
        answer
    }
}

fn seiretsu(lst: Vec<Node>) -> Vec<Node> {
    match lst.len() {
        0 => Vec::<Node>::new(),
        _ => insert_node(
            lst[lst.len() - 1].clone(),
            seiretsu(lst[..lst.len() - 1].to_vec()),
        ),
    }
}

fn koushin(p: EkiT, mut q: EkiT, dist: f32) -> EkiT {
    let d: f32 = p.shortest_dist + dist;
    if d < q.shortest_dist {
        q.shortest_dist = d;
        q.prevs= vec![p.name.clone()];
    };
    q
}

fn dijkstra() {}

#[cfg(test)]
mod tests {
    use crate::model::{read, read_node, Edge};
    use crate::get_metro_info::get_ekikan_kyori;

    use super::*;

    #[test]
    fn test_make_eki_list_1() {
        let lst = read_node(read("data/eki.csv"));
        let eki_lst = make_eki_list(&lst);
        assert_eq!(
            eki_lst[0],
            EkiT {
                name: "代々木上原".to_string(),
                shortest_dist: f32::INFINITY,
                prevs: Vec::<String>::new()
            }
        )
    }
    #[test]
    fn test_make_eki_list_2() {
        let lst = read_node(read("data/eki.csv"));
        let eki_lst = make_eki_list(&lst);
        assert_eq!(
            eki_lst[167],
            EkiT {
                name: "和光市".to_string(),
                shortest_dist: f32::INFINITY,
                prevs: Vec::<String>::new()
            }
        )
    }
    #[test]
    fn test_shokika_1() {
        let lst = read_node(read("data/eki.csv"));
        let eki_lst = make_eki_list(&lst);
        assert_eq!(
            shokika(eki_lst, &"和光市".to_string())[167],
            EkiT {
                name: "和光市".to_string(),
                shortest_dist: 0.0,
                prevs: vec!["和光市".to_string()]
            }
        )
    }
    #[test]
    fn test_insert_node_1() {
        let n = Node {
            kanji: "代々木上原".to_string(),
            kana: "よよぎうえはら".to_string(),
            roman: "yoyogiuehara".to_string(),
            shozoku: "千代田線".to_string(),
        };
        let expected = vec![n.clone()];
        assert_eq!(insert_node(n, vec![]), expected)
    }
    #[test]
    fn test_insert_node_2() {
        let n = Node {
            kanji: "代々木上原".to_string(),
            kana: "よよぎうえはら".to_string(),
            roman: "yoyogiuehara".to_string(),
            shozoku: "千代田線".to_string(),
        };
        let m = Node {
            kanji: "原宿".to_string(),
            kana: "はらじゅく".to_string(),
            roman: "harajyuku".to_string(),
            shozoku: "どっか".to_string(),
        };
        let v = vec![n.clone(), m.clone()];
        let expected = vec![m, n.clone()];
        assert_eq!(insert_node(n, v), expected)
    }
    #[test]
    fn test_seiretsu_1() {
        assert_eq!(seiretsu(vec![]), vec![])
    }
    #[test]
    fn test_seiretsu_2() {
        let n = Node {
            kanji: "代々木上原".to_string(),
            kana: "よよぎうえはら".to_string(),
            roman: "yoyogiuehara".to_string(),
            shozoku: "千代田線".to_string(),
        };
        let m = Node {
            kanji: "原宿".to_string(),
            kana: "はらじゅく".to_string(),
            roman: "harajyuku".to_string(),
            shozoku: "どっか".to_string(),
        };
        let v = vec![n.clone(), n.clone(), m.clone()];
        let expected = vec![m, n];
        assert_eq!(seiretsu(v), expected)
    }
    #[test]
    fn test_koushin_1() {
        let p = EkiT{
            name: "原宿".to_string(),
            shortest_dist: 0.0,
            prevs: vec!["原宿".to_string()],
        };
        let q = EkiT::new("代々木".to_string());
        let lst: Vec<Edge> = vec![Edge{
            tail: "原宿".to_string(),
            head: "代々木".to_string(),
            line: "どっか".to_string(),
            dist: 1.0,
            time: 30,
        }];
        let expected = EkiT{
            name: "代々木".to_string(),
            shortest_dist: 1.0,
            prevs: vec!["原宿".to_string()],
        };
        let tail = p.name.clone();
        let head = q.name.clone();
        assert_eq!(koushin(p, q, get_ekikan_kyori(&lst, &tail, &head)), expected)
    }
    #[test]
    fn test_koushin_2() {
        let p = EkiT{
            name: "原宿".to_string(),
            shortest_dist: 0.5,
            prevs: vec!["原宿".to_string()],
        };
        let q = EkiT{
            name: "代々木".to_string(),
            shortest_dist:2.0,
            prevs: vec!["秋葉原".to_string()],
        };
        let lst: Vec<Edge> = vec![Edge{
            tail: "原宿".to_string(),
            head: "代々木".to_string(),
            line: "どっか".to_string(),
            dist: 1.0,
            time: 30,
        }];
        let expected = EkiT{
            name: "代々木".to_string(),
            shortest_dist: 1.5,
            prevs: vec!["原宿".to_string()],
        };
        let tail = p.name.clone();
        let head = q.name.clone();
        assert_eq!(koushin(p, q, get_ekikan_kyori(&lst, &tail, &head)), expected)
    }
}
