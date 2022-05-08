use crate::model::Node;

#[derive(Debug, PartialEq, Clone)]
pub struct EkiT {
    pub name: String,
    pub shortest: f32,
    pub prevs: Vec<String>,
}

impl EkiT {
    pub fn new(name: String) -> EkiT {
        EkiT {
            name: name,
            shortest: f32::INFINITY,
            prevs: Vec::<String>::new(),
        }
    }
}

fn make_eki_list(lst: &Vec<Node>) -> Vec<EkiT> {
    lst.iter().map(|n| EkiT::new(n.kanji.clone())).collect::<Vec<EkiT>>()
}

fn shokika(mut lst: Vec<EkiT>, name: &String) -> Vec<EkiT> {
    match lst.len() {
        0 => Vec::new(),
        _ => {
            if &lst[0].name == name {
                lst[0].shortest = 0.0;
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

fn make_initial_eki_list(lst: &Vec<Node>, name: &String) -> Vec<EkiT> {
    let f = |x: &Node| {if &x.kanji == name {
        EkiT{name: x.kanji.clone(), shortest: 0.0, prevs: vec![x.kanji.clone()]}
    } else {EkiT::new(x.kanji.clone())}};
    lst.into_iter().map(f).collect::<Vec<EkiT>>()
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

fn koushin1(p: EkiT, mut q: EkiT, dist: f32) -> EkiT {
    let d: f32 = p.shortest + dist;
    if d < q.shortest {
        q.shortest = d;
        q.prevs = vec![p.name.clone()];
    };
    q
}

fn koushin(p: EkiT, v: Vec<EkiT>) -> Vec<EkiT> {
    if v.is_empty() {
        Vec::<EkiT>::new()
    } else if &p.name == &v[0].name {
        v[1..].to_vec()
    } else {
        let mut l = vec![v[0].clone()];
        l.append(&mut koushin(p, v[1..].to_vec()));
        l
    }
}

// 空リストが渡されたときエラーを返すようにしたい
fn saitan_wo_bunri(lst: Vec<EkiT>) -> (EkiT, Vec<EkiT>) {
    match lst.len() {
        0 => (EkiT::new("".to_string()), Vec::<EkiT>::new()),
        1 => (lst[0].clone(), Vec::<EkiT>::new()),
        _ => {
            let first = lst[0].clone();
            let mut rem = saitan_wo_bunri(lst[1..].to_vec());
            if rem.0.shortest < first.shortest {
                rem.1.push(first);
                (rem.0, rem.1)
            } else {
                rem.1.push(rem.0);
                (first, rem.1)
            }
        }
    }
}

fn sum_list(lst: Vec<i32>) -> Vec<i32> {
    fn hojo(lst: Vec<i32>, total: i32) -> Vec<i32> {
        let mut res = Vec::<i32>::new();
        match lst.len() {
            0 => res,
            _ => {
                let t = total+lst[0];
                res.push(t);
                res.append(&mut hojo(lst[1..].to_vec(), t));
                res
            }
        }
    }
    hojo(lst, 0)
}

fn dijkstra() {}

#[cfg(test)]
mod tests {
    use crate::get_metro_info::get_ekikan_kyori;
    use crate::model::{read, read_node, Edge};

    use super::*;

    #[test]
    fn make_eki_list_1() {
        let lst = read_node(read("data/eki.csv"));
        let eki_lst = make_eki_list(&lst);
        assert_eq!(
            eki_lst[0],
            EkiT {
                name: "代々木上原".to_string(),
                shortest: f32::INFINITY,
                prevs: Vec::<String>::new()
            }
        )
    }
    #[test]
    fn make_eki_list_2() {
        let lst = read_node(read("data/eki.csv"));
        let eki_lst = make_eki_list(&lst);
        assert_eq!(
            eki_lst[167],
            EkiT {
                name: "和光市".to_string(),
                shortest: f32::INFINITY,
                prevs: Vec::<String>::new()
            }
        )
    }
    #[test]
    fn shokika_1() {
        let lst = read_node(read("data/eki.csv"));
        let eki_lst = make_eki_list(&lst);
        assert_eq!(
            shokika(eki_lst, &"和光市".to_string())[167],
            EkiT {
                name: "和光市".to_string(),
                shortest: 0.0,
                prevs: vec!["和光市".to_string()]
            }
        )
    }
    #[test]
    fn make_initial_eki_list_1() {
        let lst = read_node(read("data/eki.csv"));
        assert_eq!(
            make_initial_eki_list(&lst, &"和光市".to_string())[167],
            EkiT {
                name: "和光市".to_string(),
                shortest: 0.0,
                prevs: vec!["和光市".to_string()]
            }
        )
    }
    #[test]
    fn insert_node_1() {
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
    fn insert_node_2() {
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
    fn seiretsu_1() {
        assert_eq!(seiretsu(vec![]), vec![])
    }
    #[test]
    fn seiretsu_2() {
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
    fn koushin1_1() {
        let p = EkiT {
            name: "原宿".to_string(),
            shortest: 0.0,
            prevs: vec!["原宿".to_string()],
        };
        let q = EkiT::new("代々木".to_string());
        let lst: Vec<Edge> = vec![Edge {
            tail: "原宿".to_string(),
            head: "代々木".to_string(),
            line: "どっか".to_string(),
            dist: 1.0,
            time: 30,
        }];
        let expected = EkiT {
            name: "代々木".to_string(),
            shortest: 1.0,
            prevs: vec!["原宿".to_string()],
        };
        let tail = p.name.clone();
        let head = q.name.clone();
        assert_eq!(
            koushin1(p, q, get_ekikan_kyori(&lst, &tail, &head)),
            expected
        )
    }
    #[test]
    fn koushin1_2() {
        let p = EkiT {
            name: "原宿".to_string(),
            shortest: 0.5,
            prevs: vec!["原宿".to_string()],
        };
        let q = EkiT {
            name: "代々木".to_string(),
            shortest: 2.0,
            prevs: vec!["秋葉原".to_string()],
        };
        let lst: Vec<Edge> = vec![Edge {
            tail: "原宿".to_string(),
            head: "代々木".to_string(),
            line: "どっか".to_string(),
            dist: 1.0,
            time: 30,
        }];
        let expected = EkiT {
            name: "代々木".to_string(),
            shortest: 1.5,
            prevs: vec!["原宿".to_string()],
        };
        let tail = p.name.clone();
        let head = q.name.clone();
        assert_eq!(
            koushin1(p, q, get_ekikan_kyori(&lst, &tail, &head)),
            expected
        )
    }
    #[test]
    fn koushin_1() {
        let p = EkiT {
            name: "原宿".to_string(),
            shortest: 0.5,
            prevs: vec!["原宿".to_string()],
        };
        let mut lst = vec![EkiT::new("代々木".to_string()); 2];
        lst.push(EkiT::new("原宿".to_string()));
        let expected = vec![EkiT::new("代々木".to_string()); 2];
        assert_eq!(koushin(p, lst), expected)
    }
    #[test]
    fn koushin_2() {
        let p = EkiT::new("原宿".to_string());
        let lst = vec![];
        let expected = vec![];
        assert_eq!(koushin(p, lst), expected)
    }
    #[test]
    fn saitan_wo_bunri_1() {
        let p = EkiT {
            name: "原宿".to_string(),
            shortest: 0.5,
            prevs: vec!["原宿".to_string()],
        };
        let q = EkiT {
            name: "代々木".to_string(),
            shortest: 2.0,
            prevs: vec!["秋葉原".to_string()],
        };
        let r = EkiT {
            name: "溜池山王".to_string(),
            shortest: 1.0,
            prevs: vec!["原宿".to_string()],
        };
        let v = vec![r.clone(), q.clone(), p.clone()];
        let expected = (p, vec![q, r]);
        assert_eq!(saitan_wo_bunri(v), expected)
    }
    #[test]
    fn saitan_wo_bunri_2() {
        let p = EkiT {
            name: "原宿".to_string(),
            shortest: 0.5,
            prevs: vec!["原宿".to_string()],
        };
        let q = EkiT {
            name: "代々木".to_string(),
            shortest: 2.0,
            prevs: vec!["秋葉原".to_string()],
        };
        let r = EkiT {
            name: "溜池山王".to_string(),
            shortest: 0.5,
            prevs: vec!["原宿".to_string()],
        };
        let v = vec![r.clone(), q.clone(), p.clone()];
        let expected = (r, vec![q, p]);
        assert_eq!(saitan_wo_bunri(v), expected)
    }
    #[test]
    fn saitan_wo_bunri_3() {
        let p = EkiT {
            name: "原宿".to_string(),
            shortest: 0.5,
            prevs: vec!["原宿".to_string()],
        };
        let v = vec![p.clone()];
        let expected = (p, Vec::<EkiT>::new());
        assert_eq!(saitan_wo_bunri(v), expected)
    }
    #[test]
    fn sum_list_1() {
        let v = vec![3, 2, 1, 4];
        let expected = vec![3, 5, 6, 10];
        assert_eq!(sum_list(v), expected)
    }
}
